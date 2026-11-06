
## Module wiring (2026-10)

`lib.rs` previously declared only 4 modules — 259 `src/*.rs` files (abi,
security, storage, power, codec, AI, desktop helpers…) were dead code that
never compiled. All top-level modules are now wired; `src/mod.rs` exposes the
`klib` support crate. 1,986 unit tests pass (`cargo test --lib
-- --test-threads=1`).

Known issue: ~139 modules use `static mut` singletons without synchronisation;
parallel test runs can race (SIGSEGV/SIGILL). Run tests single-threaded until
the globals are moved behind `Mutex`/`OnceLock`.
