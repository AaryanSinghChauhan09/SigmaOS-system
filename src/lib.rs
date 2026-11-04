pub mod config;
pub mod error;
pub mod orchestrator;
pub mod service;

pub use config::Config;
pub use error::{ComponentError, Result};
pub use orchestrator::{SystemOrchestrator, SystemState};
pub use service::{Service, ServiceStatus};

pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> error::Result<()>;
    fn shutdown(&mut self) -> error::Result<()>;
    fn health_check(&self) -> error::Result<()>;
}

pub struct SystemComponent {
    service: service::Service,
    orchestrator: orchestrator::SystemOrchestrator,
}

impl SystemComponent {
    pub fn new(config: Config) -> Self {
        Self {
            service: service::Service::new(config),
            orchestrator: orchestrator::SystemOrchestrator::new(),
        }
    }

    pub fn orchestrator(&self) -> &orchestrator::SystemOrchestrator {
        &self.orchestrator
    }

    pub fn orchestrator_mut(&mut self) -> &mut orchestrator::SystemOrchestrator {
        &mut self.orchestrator
    }
}

impl Component for SystemComponent {
    fn name(&self) -> &str {
        "sigmaos-system"
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn initialize(&mut self) -> error::Result<()> {
        self.service.start()?;
        self.orchestrator.transition(orchestrator::SystemState::Running);
        Ok(())
    }

    fn shutdown(&mut self) -> error::Result<()> {
        self.orchestrator.transition(orchestrator::SystemState::ShuttingDown);
        self.service.stop()?;
        self.orchestrator.transition(orchestrator::SystemState::Halted);
        Ok(())
    }

    fn health_check(&self) -> error::Result<()> {
        if self.service.is_running() {
            Ok(())
        } else {
            Err(error::ComponentError::Service("system service not running".to_string()))
        }
    }
}
