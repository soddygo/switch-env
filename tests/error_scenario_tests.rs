use std::collections::HashMap;
use tempfile::TempDir;

use envswitch::config::{ConfigManager, FileConfigManager};
use envswitch::error::{ConfigError, EnvError};
use envswitch::env::{EnvironmentManager, ShellEnvironmentManager};
use envswitch::types::ConfigPaths;

/// Helper function to create a temporary config directory
fn create_temp_config() -> (TempDir, ConfigPaths) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_dir = temp_dir.path().to_path_buf();
    let config_paths = ConfigPaths {
        config_dir: config_dir.clone(),
        config_file: config_dir.join("config.json"),
        state_file: config_dir.join("state.json"),
    };
    (temp_dir, config_paths)
}

/// Helper function to create test environment variables
fn create_test_env_vars() -> HashMap<String, String> {
    let mut vars = HashMap::new();
    vars.insert("TEST_VAR".to_string(), "test_value".to_string());
    vars
}

#[test]
fn test_config_not_found_error() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    
    // Test getting non-existent configuration
    let result = config_manager.get_config("nonexistent");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
    
    // Test deleting non-existent configuration
    let result = config_manager.delete_config("nonexistent".to_string());
    assert!(result.is_err());
    if let Err(ConfigError::ConfigNotFound(name)) = result {
        assert_eq!(name, "nonexistent");
    } else {
        panic!("Expected ConfigNotFound error");
    }
    
    // Test updating non-existent configuration
    let test_vars = create_test_env_vars();
    let result = config_manager.update_config("nonexistent".to_string(), test_vars, None);
    assert!(result.is_err());
    if let Err(ConfigError::ConfigNotFound(name)) = result {
        assert_eq!(name, "nonexistent");
    } else {
        panic!("Expected ConfigNotFound error");
    }
}

#[test]
fn test_invalid_alias_errors() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    let test_vars = create_test_env_vars();
    
    // Test empty alias
    let result = config_manager.create_config("".to_string(), test_vars.clone(), None);
    assert!(result.is_err());
    
    // Test alias with spaces
    let result = config_manager.create_config("invalid alias".to_string(), test_vars.clone(), None);
    assert!(result.is_err());
}

#[test]
fn test_invalid_environment_variable_names() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    
    // Test variable name starting with number
    let mut invalid_vars = HashMap::new();
    invalid_vars.insert("123INVALID".to_string(), "value".to_string());
    let result = config_manager.create_config("test".to_string(), invalid_vars, None);
    assert!(result.is_err());
    
    // Test variable name with hyphens
    let mut invalid_vars = HashMap::new();
    invalid_vars.insert("INVALID-VAR".to_string(), "value".to_string());
    let result = config_manager.create_config("test".to_string(), invalid_vars, None);
    assert!(result.is_err());
}

#[test]
fn test_duplicate_config_error() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    let test_vars = create_test_env_vars();
    
    // Create initial configuration
    config_manager.create_config("duplicate".to_string(), test_vars.clone(), None)
        .expect("Failed to create initial config");
    
    // Try to create duplicate configuration
    let result = config_manager.create_config("duplicate".to_string(), test_vars, None);
    assert!(result.is_err());
    if let Err(ConfigError::ConfigExists(name)) = result {
        assert_eq!(name, "duplicate");
    } else {
        panic!("Expected ConfigExists error");
    }
}

#[test]
fn test_active_config_errors() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    
    // Try to set non-existent configuration as active
    let result = config_manager.set_active_config("nonexistent".to_string());
    assert!(result.is_err());
    if let Err(ConfigError::ConfigNotFound(name)) = result {
        assert_eq!(name, "nonexistent");
    } else {
        panic!("Expected ConfigNotFound error");
    }
}

#[test]
fn test_environment_variable_errors() {
    let manager = ShellEnvironmentManager::new();
    
    // Test with invalid variable names
    let mut invalid_vars = HashMap::new();
    invalid_vars.insert("123INVALID".to_string(), "value".to_string());
    
    let result = manager.generate_shell_commands(&invalid_vars);
    assert!(result.is_err());
    if let Err(EnvError::InvalidVariableName(msg)) = result {
        assert!(msg.contains("123INVALID"));
        assert!(msg.contains("must start with"));
    } else {
        panic!("Expected InvalidVariableName error");
    }
}

#[test]
fn test_error_message_formatting() {
    // Test ConfigError display
    let config_not_found = ConfigError::ConfigNotFound("test".to_string());
    assert_eq!(format!("{}", config_not_found), "Configuration 'test' not found");
    
    let config_exists = ConfigError::ConfigExists("test".to_string());
    assert_eq!(format!("{}", config_exists), "Configuration 'test' already exists");
    
    // Test EnvError display
    let invalid_var = EnvError::InvalidVariableName("123INVALID".to_string());
    assert_eq!(format!("{}", invalid_var), "Invalid environment variable name: 123INVALID");
    
    let shell_detection_failed = EnvError::ShellDetectionFailed;
    assert_eq!(format!("{}", shell_detection_failed), "Shell detection failed");
}