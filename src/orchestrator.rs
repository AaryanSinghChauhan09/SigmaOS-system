#[derive(Debug, Clone, PartialEq)]
pub enum SystemState {
    Booting,
    Running,
    ShuttingDown,
    Halted,
}

pub struct SystemOrchestrator {
    state: SystemState,
    pid: u32,
}

impl SystemOrchestrator {
    pub fn new() -> Self {
        Self {
            state: SystemState::Booting,
            pid: std::process::id(),
        }
    }

    pub fn transition(&mut self, new_state: SystemState) {
        log::info!("System state: {:?} -> {:?}", self.state, new_state);
        self.state = new_state;
    }

    pub fn state(&self) -> &SystemState {
        &self.state
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }
}

impl Default for SystemOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}
