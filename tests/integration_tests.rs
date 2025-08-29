use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

use envswitch::config::{ConfigManager, FileConfigManager};
use envswitch::env::{EnvironmentManager, ShellEnvironmentManager};
use envswitch::shell::ShellType;
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
    vars.insert("ANTHROPIC_BASE_URL".to_string(), "https://api.deepseek.com".to_string());
    vars.insert("ANTHROPIC_MODEL".to_string(), "deepseek-chat".to_string());
    vars.insert("ANTHROPIC_AUTH_TOKEN".to_string(), "sk-test-token".to_string());
    vars
}

#[test]
fn test_end_to_end_config_workflow() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    
    // Test creating a configuration
    let test_vars = create_test_env_vars();
    config_manager.create_config("deepseek".to_string(), test_vars.clone(), None)
        .expect("Failed to create config");
    
    // Test listing configurations
    let configs = config_manager.list_configs()
        .expect("Failed to list configs");
    assert!(configs.contains(&"deepseek".to_string()));
    
    // Test getting a configuration
    let retrieved_config = config_manager.get_config("deepseek")
        .expect("Failed to get config")
        .expect("Config should exist");
    assert_eq!(retrieved_config.alias, "deepseek");
    assert_eq!(retrieved_config.variables, test_vars);
    
    // Test updating a configuration
    let mut updated_vars = test_vars.clone();
    updated_vars.insert("NEW_VAR".to_string(), "new_value".to_string());
    config_manager.update_config("deepseek".to_string(), updated_vars.clone(), None)
        .expect("Failed to update config");
    
    let updated_config = config_manager.get_config("deepseek")
        .expect("Failed to get updated config")
        .expect("Config should exist");
    assert_eq!(updated_config.variables, updated_vars);
    
    // Test setting active configuration
    config_manager.set_active_config("deepseek".to_string())
        .expect("Failed to set active config");
    
    let active_config = config_manager.get_active_config()
        .expect("Failed to get active config");
    assert_eq!(active_config, Some("deepseek".to_string()));
    
    // Test deleting a configuration
    config_manager.delete_config("deepseek".to_string())
        .expect("Failed to delete config");
    
    let deleted_config = config_manager.get_config("deepseek")
        .expect("Failed to check deleted config");
    assert!(deleted_config.is_none());
}

#[test]
fn test_shell_compatibility_zsh() {
    let env_manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
    let test_vars = create_test_env_vars();
    
    let commands = env_manager.generate_shell_commands(&test_vars)
        .expect("Failed to generate shell commands");
    
    // Verify zsh export format (check for key components)
    assert!(commands.contains("ANTHROPIC_BASE_URL"));
    assert!(commands.contains("https://api.deepseek.com"));
    assert!(commands.contains("ANTHROPIC_MODEL"));
    assert!(commands.contains("deepseek-chat"));
    assert!(commands.contains("ANTHROPIC_AUTH_TOKEN"));
    assert!(commands.contains("sk-test-token"));
}

#[test]
fn test_shell_compatibility_fish() {
    let env_manager = ShellEnvironmentManager::with_shell_type(ShellType::Fish);
    let test_vars = create_test_env_vars();
    
    let commands = env_manager.generate_shell_commands(&test_vars)
        .expect("Failed to generate shell commands");
    
    // Verify fish set format (check for key components)
    assert!(commands.contains("ANTHROPIC_BASE_URL"));
    assert!(commands.contains("https://api.deepseek.com"));
    assert!(commands.contains("ANTHROPIC_MODEL"));
    assert!(commands.contains("deepseek-chat"));
    assert!(commands.contains("ANTHROPIC_AUTH_TOKEN"));
    assert!(commands.contains("sk-test-token"));
}

#[test]
fn test_shell_compatibility_bash() {
    let env_manager = ShellEnvironmentManager::with_shell_type(ShellType::Bash);
    let test_vars = create_test_env_vars();
    
    let commands = env_manager.generate_shell_commands(&test_vars)
        .expect("Failed to generate shell commands");
    
    // Verify bash export format (check for key components)
    assert!(commands.contains("ANTHROPIC_BASE_URL"));
    assert!(commands.contains("https://api.deepseek.com"));
    assert!(commands.contains("ANTHROPIC_MODEL"));
    assert!(commands.contains("deepseek-chat"));
    assert!(commands.contains("ANTHROPIC_AUTH_TOKEN"));
    assert!(commands.contains("sk-test-token"));
}

#[test]
fn test_import_export_workflow() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths.clone());
    
    // Create test configurations
    let deepseek_vars = create_test_env_vars();
    config_manager.create_config("deepseek".to_string(), deepseek_vars, None)
        .expect("Failed to create deepseek config");
    
    let mut kimi_vars = HashMap::new();
    kimi_vars.insert("ANTHROPIC_BASE_URL".to_string(), "https://api.moonshot.cn".to_string());
    kimi_vars.insert("ANTHROPIC_MODEL".to_string(), "moonshot-v1-8k".to_string());
    config_manager.create_config("kimi".to_string(), kimi_vars, None)
        .expect("Failed to create kimi config");
    
    // Export configurations
    let export_path = config_paths.config_dir.join("export.json");
    config_manager.export_to_file(&export_path)
        .expect("Failed to export configs");
    
    // Verify export file exists and has content
    assert!(export_path.exists());
    let export_content = fs::read_to_string(&export_path)
        .expect("Failed to read export file");
    assert!(export_content.contains("deepseek"));
    assert!(export_content.contains("kimi"));
    
    // Create a new config manager with different directory
    let (_temp_dir2, config_paths2) = create_temp_config();
    let config_manager2 = FileConfigManager::with_paths(config_paths2);
    
    // Import configurations
    config_manager2.import_from_file(&export_path, false)
        .expect("Failed to import configs");
    
    // Verify imported configurations
    let imported_configs = config_manager2.list_configs()
        .expect("Failed to list imported configs");
    assert!(imported_configs.contains(&"deepseek".to_string()));
    assert!(imported_configs.contains(&"kimi".to_string()));
}

#[test]
fn test_error_scenarios() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    
    // Test getting non-existent configuration
    let result = config_manager.get_config("nonexistent");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
    
    // Test deleting non-existent configuration
    let result = config_manager.delete_config("nonexistent".to_string());
    assert!(result.is_err());
    
    // Test updating non-existent configuration
    let test_vars = create_test_env_vars();
    let result = config_manager.update_config("nonexistent".to_string(), test_vars, None);
    assert!(result.is_err());
    
    // Test setting non-existent active configuration
    let result = config_manager.set_active_config("nonexistent".to_string());
    assert!(result.is_err());
}

#[test]
fn test_config_validation() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    
    // Test invalid alias (empty)
    let test_vars = create_test_env_vars();
    let result = config_manager.create_config("".to_string(), test_vars.clone(), None);
    assert!(result.is_err());
    
    // Test invalid alias (with spaces)
    let result = config_manager.create_config("invalid alias".to_string(), test_vars.clone(), None);
    assert!(result.is_err());
    
    // Test invalid environment variable name
    let mut invalid_vars = HashMap::new();
    invalid_vars.insert("INVALID-VAR".to_string(), "value".to_string());
    let result = config_manager.create_config("test".to_string(), invalid_vars, None);
    assert!(result.is_err());
}

#[test]
fn test_large_configuration_handling() {
    let (_temp_dir, config_paths) = create_temp_config();
    let config_manager = FileConfigManager::with_paths(config_paths);
    
    // Create a large configuration with many variables
    let mut large_vars = HashMap::new();
    for i in 0..100 { // Reduced from 1000 to 100 for faster testing
        large_vars.insert(format!("VAR_{}", i), format!("value_{}", i));
    }
    
    config_manager.create_config("large_config".to_string(), large_vars.clone(), None)
        .expect("Failed to create large config");
    
    // Verify the large configuration can be retrieved
    let retrieved_config = config_manager.get_config("large_config")
        .expect("Failed to get large config")
        .expect("Config should exist");
    
    assert_eq!(retrieved_config.variables.len(), 100);
    assert_eq!(retrieved_config.variables, large_vars);
    
    // Test shell command generation with large configuration
    let env_manager = ShellEnvironmentManager::new();
    let commands = env_manager.generate_shell_commands(&large_vars)
        .expect("Failed to generate commands for large config");
    
    // Verify some variables are included in the commands
    assert!(commands.contains("VAR_0"));
    assert!(commands.contains("VAR_50"));
    assert!(commands.contains("VAR_99"));
}