"""Init and service orchestration for SigmaOS-system.

Principles: deterministic boot ordering (alphabetical tie-breaks), fail-fast
on dependency cycles and unknown units, pure plan layer (no service is
started by the planning functions), separation of mechanism (the dependency
graph) and policy (target selection).
"""
import argparse, json, os, sys
from collections import deque

ENGINE = {
    "name": "sigma-init",
    "summary": "Plan boot stages, service ordering, targets and recovery",
    "language": "python",
    "verbs": ["boot-plan", "services", "target", "recover"],
    "principles": ["Separation of Mechanism & Policy", "Fail-Fast",
                   "Pure Plan / Thin Execute", "KISS", "Deterministic Testing"],
}

cli_doc = """\
$ sigma-init boot-plan --target graphical
$ sigma-init services units.json --target multi-user
$ sigma-init target units.json graphical
$ sigma-init recover fs-error bad-config
"""

BOOT_STAGES = ("firmware", "loader", "kernel", "initramfs", "init", "local-fs",
               "sysinit", "services", "network", "multi-user", "graphical")

RECOVERY = {
    "fs-error": (10, "fsck -a /dev/root"),
    "bad-config": (20, "boot-rollback last-good"),
    "missing-unit": (30, "isolate rescue.target"),
    "timeout": (40, "enter rescue shell"),
}


class SystemError(Exception):
    pass


def boot_plan(target="graphical"):
    """Pure: ordered boot stages up to and including the target stage."""
    if target not in BOOT_STAGES:
        raise SystemError(f"unknown boot target: {target!r}")
    return list(BOOT_STAGES[:BOOT_STAGES.index(target) + 1])


def _validate_units(services):
    for name, spec in services.items():
        if not isinstance(spec, dict):
            raise SystemError(f"{name}: bad unit spec")
        for key in ("deps", "targets"):
            if not isinstance(spec.get(key, []), list):
                raise SystemError(f"{name}: {key} must be a list")
        for dep in spec.get("deps", []):
            if dep not in services:
                raise SystemError(f"{name}: unknown dependency {dep!r}")


def topological(services):
    """Pure: deterministic (alphabetical Kahn) ordering of all units."""
    _validate_units(services)
    indeg = {n: len(spec.get("deps", [])) for n, spec in services.items()}
    dependents = {n: [] for n in services}
    for n, spec in services.items():
        for dep in spec.get("deps", []):
            dependents[dep].append(n)
    ready = deque(sorted(n for n, d in indeg.items() if d == 0))
    order = []
    while ready:
        n = ready.popleft()
        order.append(n)
        for m in sorted(dependents[n]):
            indeg[m] -= 1
            if indeg[m] == 0:
                ready.append(m)
        ready = deque(sorted(ready))
    if len(order) != len(services):
        cyclic = sorted(n for n, d in indeg.items() if d > 0)
        raise SystemError("dependency cycle: " + ", ".join(cyclic))
    return order


def target_services(services, target):
    """Pure: units belonging to a target, dependency-ordered (closure over deps)."""
    _validate_units(services)
    members = {n for n, spec in services.items() if target in spec.get("targets", [])}
    if not members:
        raise SystemError(f"unknown or empty target: {target!r}")
    closure = set()

    def add(n):
        if n in closure:
            return
        closure.add(n)
        for dep in services[n].get("deps", []):
            add(dep)

    for n in members:
        add(n)
    return [n for n in topological(services) if n in closure]


def recover_plan(issues):
    """Pure: prioritized recovery action plan (unique, ordered)."""
    steps, seen = [], set()
    for code in issues:
        if code not in RECOVERY:
            raise SystemError(f"unknown recovery issue: {code!r}")
        if code in seen:
            continue
        seen.add(code)
        steps.append({"priority": RECOVERY[code][0], "issue": code,
                      "action": RECOVERY[code][1]})
    return sorted(steps, key=lambda s: s["priority"])


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog=ENGINE["name"], description=ENGINE["summary"])
    sub = p.add_subparsers(dest="cmd", required=True)
    b = sub.add_parser("boot-plan", help="ordered boot stages")
    b.add_argument("--target", default="graphical", choices=BOOT_STAGES)
    s = sub.add_parser("services", help="dependency-ordered units for a target")
    s.add_argument("units", help="JSON object: unit -> {deps, targets}")
    s.add_argument("--target", default="multi-user")
    t = sub.add_parser("target", help="units activated by a target (with deps)")
    t.add_argument("units", help="JSON object: unit -> {deps, targets}")
    t.add_argument("target_name", help="target name")
    r = sub.add_parser("recover", help="prioritized recovery plan")
    r.add_argument("issues", nargs="+", help="recovery issue codes")
    return p


def main(argv=None) -> int:
    args = build_parser().parse_args(argv)
    try:
        if args.cmd == "boot-plan":
            for stage in boot_plan(args.target):
                print(stage)
            return 0
        if args.cmd == "recover":
            for step in recover_plan(args.issues):
                print(f"[{step['priority']:02}] {step['issue']}: {step['action']}")
            return 0
        with open(args.units, "r", encoding="utf-8") as fh:
            services = json.load(fh)
        names = (target_services(services, args.target_name)
                 if args.cmd == "target" else topological(services))
        for n in names:
            print(n)
        return 0
    except (SystemError, OSError, ValueError, json.JSONDecodeError) as e:
        print(f"error: {e}", file=sys.stderr)
        return 1


def self_test() -> None:
    # boot stages
    assert boot_plan("init") == ["firmware", "loader", "kernel", "initramfs", "init"]
    assert boot_plan()[-1] == "graphical"
    try:
        boot_plan("hastur")
    except SystemError:
        pass
    else:
        raise AssertionError("unknown boot target not rejected")
    units = {
        "sshd": {"deps": ["networkd"], "targets": ["multi-user"]},
        "networkd": {"deps": [], "targets": ["multi-user"]},
        "shell": {"deps": ["sshd", "networkd"], "targets": ["multi-user"]},
        "greetd": {"deps": ["wayland"], "targets": ["graphical"]},
        "wayland": {"deps": ["networkd"], "targets": ["graphical"]},
    }
    # topological order: deps before dependents, alphabetical tie-break
    order = topological(units)
    assert order.index("networkd") < order.index("sshd") < order.index("shell")
    assert order.index("networkd") < order.index("wayland") < order.index("greetd")
    assert order == sorted(order, key=order.index)  # deterministic
    # target closure
    mu = target_services(units, "multi-user")
    assert mu == ["networkd", "sshd", "shell"]
    g = target_services(units, "graphical")
    assert g == ["networkd", "wayland", "greetd"]  # pulls in networkd via deps
    try:
        target_services(units, "rescue")
    except SystemError:
        pass
    else:
        raise AssertionError("unknown target not rejected")
    # cycles and unknown deps are fail-fast
    cyclic = {"a": {"deps": ["b"]}, "b": {"deps": ["a"]}}
    try:
        topological(cyclic)
    except SystemError as e:
        assert "cycle" in str(e)
    else:
        raise AssertionError("cycle not detected")
    try:
        topological({"a": {"deps": ["ghost"]}})
    except SystemError:
        pass
    else:
        raise AssertionError("unknown dep not detected")
    # recovery prioritization
    rp = recover_plan(["timeout", "fs-error", "fs-error"])
    assert [s["issue"] for s in rp] == ["fs-error", "timeout"]
    assert rp[0]["action"].startswith("fsck")
    try:
        recover_plan(["unknown-thing"])
    except SystemError:
        pass
    else:
        raise AssertionError("unknown issue not rejected")
    # CLI smoke through main()
    import tempfile
    with tempfile.TemporaryDirectory() as d:
        upath = os.path.join(d, "units.json")
        with open(upath, "w", encoding="utf-8") as fh:
            json.dump(units, fh)
        assert main(["boot-plan", "--target", "multi-user"]) == 0
        assert main(["services", upath]) == 0
        assert main(["target", upath, "graphical"]) == 0
        assert main(["recover", "fs-error", "timeout"]) == 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
