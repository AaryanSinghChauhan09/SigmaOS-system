use sigmaos_system::{Config, SystemComponent, Component};

fn main() {
    env_logger::init();
    let config = Config::default();
    let mut component = SystemComponent::new(config);
    if let Err(e) = component.initialize() {
        eprintln!("Failed to initialize: {}", e);
        std::process::exit(1);
    }
    println!("SigmaOS system running");
    if let Err(e) = component.shutdown() {
        eprintln!("Shutdown error: {}", e);
    }
}
