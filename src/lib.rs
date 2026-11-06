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
pub mod sigma3d;
pub mod sigmaacpi;
pub mod sigmaaliasmanager;
pub mod sigmaananicy;
pub mod sigmaani;
pub mod sigmaapplicationmanager;
pub mod sigmaarchivemanager;
pub mod sigmaart;
pub mod sigmaaudioconfig;
pub mod sigmaaudiomanager2;
pub mod sigmaauditmanager;
pub mod sigmaautomation;
pub mod sigmabackup;
pub mod sigmabackupmanager;
pub mod sigmabluetooth;
pub mod sigmabluetoothmanager2;
pub mod sigmacache;
pub mod sigmacachemanager2;
pub mod sigmacad;
pub mod sigmacapturemanager;
pub mod sigmacertificatemanager;
pub mod sigmacgroup;
pub mod sigmacleaner;
pub mod sigmaclipboardmanager;
pub mod sigmacodec;
pub mod sigmacolorscheme;
pub mod sigmacommandmanager;
pub mod sigmacompressionmanager;
pub mod sigmaconfig;
pub mod sigmaconfigmanager;
pub mod sigmacontainer;
pub mod sigmacontainermanager;
pub mod sigmacraftlibs;
pub mod sigmacron;
pub mod sigmacronmanager2;
pub mod sigmacrypto;
pub mod sigmadb;
pub mod sigmadeck;
pub mod sigmadefrag;
pub mod sigmadesign;
pub mod sigmadevice;
pub mod sigmadevicemanager2;
pub mod sigmadhcp;
pub mod sigmadiskmanager;
pub mod sigmadiskusage;
pub mod sigmadisplaymanager;
pub mod sigmadisplaymanager2;
pub mod sigmadistfs;
pub mod sigmadns;
pub mod sigmadnsmanager2;
pub mod sigmadrivermanager;
pub mod sigmaeffect;
pub mod sigmaencryptionmanager;
pub mod sigmaenvironmentmanager;
pub mod sigmaevent;
pub mod sigmafig;
pub mod sigmafilemanager;
pub mod sigmafilesystemmanager;
pub mod sigmafilewatcher;
pub mod sigmafilm;
pub mod sigmafirewall;
pub mod sigmafirewallmanager2;
pub mod sigmaflatpak;
pub mod sigmafont;
pub mod sigmafs;
pub mod sigmafsck;
pub mod sigmafsmon;
pub mod sigmaftp;
pub mod sigmagrid;
pub mod sigmahardwaremanager;
pub mod sigmahardwareprofiler;
pub mod sigmahelpermanager;
pub mod sigmahibernation;
pub mod sigmahttp;
pub mod sigmahwmon;
pub mod sigmaindexmanager;
pub mod sigmainstaller;
pub mod sigmainstallmanager;
pub mod sigmaintegrity;
pub mod sigmaintegritymanager2;
pub mod sigmait;
pub mod sigmajails;
pub mod sigmakernelmanager;
pub mod sigmakeyboardmanager;
pub mod sigmalaunchmanager;
pub mod sigmalight;
pub mod sigmalogmanager;
pub mod sigmalogrotate;
pub mod sigmalogrotationmanager;
pub mod sigmalvm;
pub mod sigmamail;
pub mod sigmamenumanager;
pub mod sigmametricsmanager;
pub mod sigmamicrophone;
pub mod sigmamigrationmanager;
pub mod sigmamkfs;
pub mod sigmamonitor;
pub mod sigmamount;
pub mod sigmamq;
pub mod sigmananny;
pub mod sigmanetwork;
pub mod sigmanetworkmanager3;
pub mod sigmanetworkmonitor;
pub mod sigmanfs;
pub mod sigmanic;
pub mod sigmanotificationmanager;
pub mod sigmanotifyenhanced;
pub mod sigmantp;
pub mod sigmapackagemanager;
pub mod sigmapackagemanager2;
pub mod sigmapackagemanager3;
pub mod sigmapackageupdates;
pub mod sigmaparted;
pub mod sigmapathmanager;
pub mod sigmapdf;
pub mod sigmaperformancemonitor;
pub mod sigmaphoto;
pub mod sigmapkg;
pub mod sigmaports;
pub mod sigmapower;
pub mod sigmapowermanager;
pub mod sigmapowermanager2;
pub mod sigmaprint2;
pub mod sigmaprinter;
pub mod sigmaprivilegemanager;
pub mod sigmaprocess;
pub mod sigmaprocessmanager;
pub mod sigmaprocessmonitor;
pub mod sigmaprofilemanager;
pub mod sigmaproxy;
pub mod sigmaquickshell;
pub mod sigmaquota;
pub mod sigmaraid;
pub mod sigmarecovery;
pub mod sigmarecovery2;
pub mod sigmarefreshmanager;
pub mod sigmaremotemanager;
pub mod sigmarepositorymanager;
pub mod sigmaresource;
pub mod sigmaresourcemanager;
pub mod sigmarestartmanager;
pub mod sigmarestoremanager;
pub mod sigmascanner;
pub mod sigmascheduledfsck;
pub mod sigmaschedulermanager;
pub mod sigmasearch;
pub mod sigmasecurityaudit;
pub mod sigmasecuritymanager;
pub mod sigmasecuritymanager2;
pub mod sigmaservice;
pub mod sigmaservicemanager3;
pub mod sigmaservicemanager4;
pub mod sigmasession;
pub mod sigmasessionmanager;
pub mod sigmasetupmanager;
pub mod sigmasetupmanager2;
pub mod sigmasmb;
pub mod sigmasnap;
pub mod sigmasnapshot;
pub mod sigmasnapshotmanager;
pub mod sigmasoftwaremanager;
pub mod sigmasound;
pub mod sigmassh;
pub mod sigmastallmanager;
pub mod sigmastorage;
pub mod sigmastoragemanager;
pub mod sigmaswap;
pub mod sigmasystemd;
pub mod sigmasysteminfo;
pub mod sigmasystemmonitor;
pub mod sigmasystemmonitor2;
pub mod sigmasystemtools;
pub mod sigmataskmanager;
pub mod sigmataskscheduler;
pub mod sigmatemplatemanager;
pub mod sigmathemeinstaller;
pub mod sigmathememanager;
pub mod sigmathememanager2;
pub mod sigmathernal;
pub mod sigmatime;
pub mod sigmatimer;
pub mod sigmatimeshift;
pub mod sigmatogglemanager;
pub mod sigmatrim;
pub mod sigmaupdate;
pub mod sigmaupdatemanager;
pub mod sigmaupdatemanager2;
pub mod sigmausbwriter;
pub mod sigmausermanager;
pub mod sigmavector;
pub mod sigmavirtualization;
pub mod sigmavirtualizationmanager;
pub mod sigmavoice;
pub mod sigmawalker;
pub mod sigmawallpaper;
pub mod sigmawebcam;
pub mod sigmawizardmanager;
pub mod sigmaword;
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
