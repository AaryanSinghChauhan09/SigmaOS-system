pub mod config;
pub mod crypto;
pub mod klib;
pub mod error;
pub mod orchestrator;
pub mod service;

pub mod abi;
pub mod atomic_upgrade;
pub mod automation;
pub mod boot_analyzer;
pub mod boot_manager;
pub mod cleanup;
pub mod common_tweaks;
pub mod cron;
pub mod declarative_state;
pub mod defrag;
pub mod dmesg;
pub mod duplicate;
pub mod gamemode_daemon;
pub mod generation_manager;
pub mod logrotate;
pub mod memory;
pub mod mint_tools;
pub mod monitor;
pub mod network_manager;
pub mod omarchy_atreyu_system_plugin;
pub mod omarchy_docker_rootless_engine;
pub mod optimizer;
pub mod power;
pub mod power_manager;
pub mod preload;
pub mod process_supervisor;
pub mod runit_supervisor;
pub mod sandbox;
pub mod scheduler_manager;
pub mod service_manager;
pub mod settings_manager;
pub mod shredder;
pub mod sigmaaliasmanager;
pub mod sigmaapplicationmanager;
pub mod sigmaaudiomanager2;
pub mod sigmaauditmanager;
pub mod sigmabluetoothmanager2;
pub mod sigmacapturemanager;
pub mod sigmadisplaymanager2;
pub mod sigmaenvironmentmanager;
pub mod sigmafilewatcher;
pub mod sigmahardwareprofiler;
pub mod sigmahelpermanager;
pub mod sigmainstallmanager;
pub mod sigmalvm;
pub mod sigmamigrationmanager;
pub mod sigmananny;
pub mod sigmanetworkmanager3;
pub mod sigmapackagemanager3;
pub mod sigmapathmanager;
pub mod sigmapowermanager2;
pub mod sigmaprivilegemanager;
pub mod sigmaprocessmanager;
pub mod sigmaprofilemanager;
pub mod sigmaraid;
pub mod sigmaschedulermanager;
pub mod sigmasecuritymanager2;
pub mod sigmaservicemanager4;
pub mod sigmasessionmanager;
pub mod sigmasetupmanager2;
pub mod sigmastoragemanager;
pub mod sigmatemplatemanager;
pub mod sigmathememanager2;
pub mod sigmawizardmanager;
pub mod signamespaces;
pub mod snapshot;
pub mod snapshot_schedule;
pub mod sovereign_init_and_supervisor_advancements_v52;
pub mod sovereign_init_and_supervisor_advancements_v53;
pub mod sovereign_init_and_supervisor_advancements_v60;
pub mod sovereign_init_and_supervisor_advancements_v61;
pub mod sovereign_init_and_supervisor_advancements_v62;
pub mod sovereign_init_and_supervisor_advancements_v63;
pub mod sovereign_init_and_supervisor_advancements_v63_pr;
pub mod sovereign_init_and_supervisor_advancements_v67;
pub mod sovereign_init_and_supervisor_advancements_v68_pr;
pub mod sovereign_init_and_supervisor_advancements_v69_pr;
pub mod sovereign_init_and_supervisor_advancements_v71_pr;
pub mod sovereign_init_and_supervisor_advancements_v75_pr;
pub mod sovereign_init_and_supervisor_advancements_v76_pr;
pub mod sovereign_init_and_supervisor_inspirations_pr;
pub mod sovereign_init_supervisor;
pub mod startup;
pub mod state;
pub mod syslog;
pub mod system_manager;
pub mod system_monitor;
pub mod task_manager;
pub mod task_scheduler;
pub mod update_manager;
pub mod user;
pub mod user_manager;

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
