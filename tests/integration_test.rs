use sigmaos_system::{Config, SystemComponent, SystemOrchestrator, SystemState, Component};

#[test]
fn test_config_default_aaa() {
    // Arrange + Act
    let config = Config::default();
    // Assert
    assert!(config.enabled);
    assert_eq!(config.log_level, "info");
    assert_eq!(config.max_processes, 1024);
}

#[test]
fn test_config_validate_aaa() {
    // Arrange
    let config = Config::default();
    // Act
    let result = config.validate();
    // Assert
    assert!(result.is_ok());
}

#[test]
fn test_config_validate_empty_log_level_aaa() {
    // Arrange
    let mut config = Config::default();
    config.log_level = String::new();
    // Act
    let result = config.validate();
    // Assert
    assert!(result.is_err());
}

#[test]
fn test_component_initialize_shutdown_aaa() {
    // Arrange
    let config = Config::default();
    let mut component = SystemComponent::new(config);
    // Act
    let init = component.initialize();
    // Assert
    assert!(init.is_ok());
    // Act
    let shutdown = component.shutdown();
    // Assert
    assert!(shutdown.is_ok());
}

#[test]
fn test_component_name_aaa() {
    // Arrange
    let component = SystemComponent::new(Config::default());
    // Act + Assert
    assert_eq!(component.name(), "sigmaos-system");
}

#[test]
fn test_health_check_running_aaa() {
    // Arrange
    let mut component = SystemComponent::new(Config::default());
    component.initialize().unwrap();
    // Act
    let health = component.health_check();
    // Assert
    assert!(health.is_ok());
}

#[test]
fn test_health_check_stopped_aaa() {
    // Arrange
    let component = SystemComponent::new(Config::default());
    // Act
    let health = component.health_check();
    // Assert
    assert!(health.is_err());
}

#[test]
fn test_orchestrator_state_transition_aaa() {
    // Arrange
    let mut orch = SystemOrchestrator::new();
    // Assert initial state
    assert_eq!(orch.state(), &SystemState::Booting);
    // Act
    orch.transition(SystemState::Running);
    // Assert
    assert_eq!(orch.state(), &SystemState::Running);
}

#[test]
fn test_orchestrator_pid_aaa() {
    // Arrange + Act
    let orch = SystemOrchestrator::new();
    // Assert
    assert!(orch.pid() > 0);
}

#[test]
fn test_system_full_lifecycle_aaa() {
    // Arrange
    let mut component = SystemComponent::new(Config::default());
    // Act: initialize transitions orchestrator to Running
    component.initialize().unwrap();
    assert_eq!(component.orchestrator().state(), &SystemState::Running);
    // Act: shutdown transitions to Halted
    component.shutdown().unwrap();
    assert_eq!(component.orchestrator().state(), &SystemState::Halted);
}
