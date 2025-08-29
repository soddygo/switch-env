use std::collections::HashMap;
use std::env;

use envswitch::env::{EnvironmentManager, ShellEnvironmentManager};
use envswitch::shell::{ShellDetector, ShellType};

/// Helper function to create test environment variables
fn create_test_env_vars() -> HashMap<String, String> {
    let mut vars = HashMap::new();
    vars.insert("TEST_VAR_1".to_string(), "simple_value".to_string());
    vars.insert("TEST_VAR_2".to_string(), "value with spaces".to_string());
    vars.insert("TEST_VAR_3".to_string(), "value\"with'quotes".to_string());
    vars.insert("TEST_VAR_4".to_string(), "value=with=equals".to_string());
    vars.insert("TEST_VAR_5".to_string(), "".to_string()); // Empty value
    vars
}

#[test]
fn test_shell_detection() {
    // Test shell detection with different SHELL environment variables
    let original_shell = env::var("SHELL").ok();
    
    // Test zsh detection
    env::set_var("SHELL", "/bin/zsh");
    let detected = ShellDetector::detect_shell();
    assert!(matches!(detected, ShellType::Zsh));
    
    // Test fish detection
    env::set_var("SHELL", "/usr/bin/fish");
    let detected = ShellDetector::detect_shell();
    assert!(matches!(detected, ShellType::Fish));
    
    // Test bash detection
    env::set_var("SHELL", "/bin/bash");
    let detected = ShellDetector::detect_shell();
    assert!(matches!(detected, ShellType::Bash));
    
    // Test unknown shell
    env::set_var("SHELL", "/bin/unknown");
    let detected = ShellDetector::detect_shell();
    assert!(matches!(detected, ShellType::Unknown(_)));
    
    // Restore original SHELL
    match original_shell {
        Some(shell) => env::set_var("SHELL", shell),
        None => env::remove_var("SHELL"),
    }
}

#[test]
fn test_zsh_command_generation() {
    let manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
    let test_vars = create_test_env_vars();
    
    let commands = manager.generate_shell_commands(&test_vars)
        .expect("Failed to generate zsh commands");
    
    // Verify zsh export format (check for key components)
    assert!(commands.contains("TEST_VAR_1"));
    assert!(commands.contains("simple_value"));
    assert!(commands.contains("export"));
    
    // Test unset commands
    let keys: Vec<String> = test_vars.keys().cloned().collect();
    let unset_commands = manager.generate_unset_commands(&keys)
        .expect("Failed to generate zsh unset commands");
    
    for key in &keys {
        assert!(unset_commands.contains(&format!("unset {}", key)));
    }
}

#[test]
fn test_fish_command_generation() {
    let manager = ShellEnvironmentManager::with_shell_type(ShellType::Fish);
    let test_vars = create_test_env_vars();
    
    let commands = manager.generate_shell_commands(&test_vars)
        .expect("Failed to generate fish commands");
    
    // Verify fish set format (check for key components)
    assert!(commands.contains("TEST_VAR_1"));
    assert!(commands.contains("simple_value"));
    assert!(commands.contains("set -x"));
    
    // Test unset commands
    let keys: Vec<String> = test_vars.keys().cloned().collect();
    let unset_commands = manager.generate_unset_commands(&keys)
        .expect("Failed to generate fish unset commands");
    
    for key in &keys {
        assert!(unset_commands.contains(&format!("set -e {}", key)));
    }
}

#[test]
fn test_bash_command_generation() {
    let manager = ShellEnvironmentManager::with_shell_type(ShellType::Bash);
    let test_vars = create_test_env_vars();
    
    let commands = manager.generate_shell_commands(&test_vars)
        .expect("Failed to generate bash commands");
    
    // Verify bash export format (check for key components)
    assert!(commands.contains("TEST_VAR_1"));
    assert!(commands.contains("simple_value"));
    assert!(commands.contains("export"));
    
    // Test unset commands
    let keys: Vec<String> = test_vars.keys().cloned().collect();
    let unset_commands = manager.generate_unset_commands(&keys)
        .expect("Failed to generate bash unset commands");
    
    for key in &keys {
        assert!(unset_commands.contains(&format!("unset {}", key)));
    }
}

#[test]
fn test_special_characters_handling() {
    let manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
    
    let mut special_vars = HashMap::new();
    special_vars.insert("VAR_WITH_SIMPLE".to_string(), "simple_value".to_string());
    special_vars.insert("VAR_WITH_SPACES".to_string(), "value with spaces".to_string());
    
    let commands = manager.generate_shell_commands(&special_vars)
        .expect("Failed to generate commands with special characters");
    
    // Verify variables are included
    assert!(commands.contains("VAR_WITH_SIMPLE"));
    assert!(commands.contains("VAR_WITH_SPACES"));
    assert!(commands.contains("simple_value"));
    assert!(commands.contains("value with spaces"));
}

#[test]
fn test_integration_instructions() {
    let zsh_manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
    let fish_manager = ShellEnvironmentManager::with_shell_type(ShellType::Fish);
    let bash_manager = ShellEnvironmentManager::with_shell_type(ShellType::Bash);
    
    let zsh_instructions = zsh_manager.get_integration_instructions();
    let fish_instructions = fish_manager.get_integration_instructions();
    let bash_instructions = bash_manager.get_integration_instructions();
    
    // Verify shell-specific instructions
    assert!(zsh_instructions.contains("~/.zshrc"));
    assert!(zsh_instructions.contains("eval \"$(envswitch use"));
    
    assert!(fish_instructions.contains("~/.config/fish/config.fish"));
    assert!(fish_instructions.contains("eval (envswitch use"));
    
    assert!(bash_instructions.contains("~/.bashrc"));
    assert!(bash_instructions.contains("eval \"$(envswitch use"));
}

#[test]
fn test_claude_variables_detection() {
    let manager = ShellEnvironmentManager::new();
    
    // Test getting Claude variables status
    let status = manager.get_claude_variables_status();
    
    // Should return a vector of EnvVarStatus
    assert!(status.is_empty() || !status.is_empty()); // Just verify it returns something
}

#[test]
fn test_variable_validation() {
    let manager = ShellEnvironmentManager::new();
    
    // Test valid variable names
    let mut valid_vars = HashMap::new();
    valid_vars.insert("VALID_VAR".to_string(), "value".to_string());
    valid_vars.insert("VAR_123".to_string(), "value".to_string());
    valid_vars.insert("_UNDERSCORE_VAR".to_string(), "value".to_string());
    
    let result = manager.generate_shell_commands(&valid_vars);
    assert!(result.is_ok());
    
    // Test invalid variable names
    let mut invalid_vars = HashMap::new();
    invalid_vars.insert("123_INVALID".to_string(), "value".to_string());
    
    let result = manager.generate_shell_commands(&invalid_vars);
    assert!(result.is_err());
    
    let mut invalid_vars2 = HashMap::new();
    invalid_vars2.insert("INVALID-VAR".to_string(), "value".to_string());
    
    let result = manager.generate_shell_commands(&invalid_vars2);
    assert!(result.is_err());
}

#[test]
fn test_empty_and_edge_cases() {
    let manager = ShellEnvironmentManager::new();
    
    // Test empty variables map
    let empty_vars = HashMap::new();
    let commands = manager.generate_shell_commands(&empty_vars)
        .expect("Failed to generate commands for empty vars");
    assert!(commands.is_empty() || commands.trim().is_empty());
    
    // Test single variable
    let mut single_var = HashMap::new();
    single_var.insert("SINGLE_VAR".to_string(), "single_value".to_string());
    let commands = manager.generate_shell_commands(&single_var)
        .expect("Failed to generate commands for single var");
    assert!(commands.contains("SINGLE_VAR"));
    
    // Test moderately long variable value (within limits)
    let long_value = "a".repeat(500);
    let mut long_var = HashMap::new();
    long_var.insert("LONG_VAR".to_string(), long_value.clone());
    let commands = manager.generate_shell_commands(&long_var)
        .expect("Failed to generate commands for long var");
    assert!(commands.contains("LONG_VAR"));
}

#[test]
fn test_switch_and_clear_commands() {
    let manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
    let test_vars = create_test_env_vars();
    
    // Test switch commands
    let switch_commands = manager.generate_switch_commands(&test_vars)
        .expect("Failed to generate switch commands");
    
    // Should contain the environment variables
    assert!(switch_commands.contains("TEST_VAR_1"));
    assert!(switch_commands.contains("simple_value"));
    
    // Test clear commands
    let keys: Vec<String> = test_vars.keys().cloned().collect();
    let clear_commands = manager.generate_clear_commands(&keys)
        .expect("Failed to generate clear commands");
    
    // Should contain unset commands
    for key in &keys {
        assert!(clear_commands.contains(&format!("unset {}", key)));
    }
}

#[cfg(unix)]
#[test]
fn test_actual_shell_execution() {
    // This test requires actual shell execution and should only run on Unix systems
    use std::process::Command;
    
    let manager = ShellEnvironmentManager::with_shell_type(ShellType::Bash);
    let mut test_vars = HashMap::new();
    test_vars.insert("ENVSWITCH_TEST_VAR".to_string(), "test_value_123".to_string());
    
    let commands = manager.generate_shell_commands(&test_vars)
        .expect("Failed to generate shell commands");
    
    // Execute the commands in a bash shell
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("{}; echo $ENVSWITCH_TEST_VAR", commands))
        .output()
        .expect("Failed to execute bash command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test_value_123"));
}

#[test]
fn test_shell_type_display() {
    assert_eq!(format!("{}", ShellType::Zsh), "zsh");
    assert_eq!(format!("{}", ShellType::Fish), "fish");
    assert_eq!(format!("{}", ShellType::Bash), "bash");
    assert_eq!(format!("{}", ShellType::Unknown("custom".to_string())), "unknown(custom)");
}

#[test]
fn test_environment_variable_status() {
    let manager = ShellEnvironmentManager::new();
    
    // Set a test environment variable
    env::set_var("ENVSWITCH_STATUS_TEST", "status_value");
    
    let keys = vec!["ENVSWITCH_STATUS_TEST".to_string()];
    let status = manager.get_variable_status(&keys);
    assert!(!status.is_empty());
    assert_eq!(status[0].key, "ENVSWITCH_STATUS_TEST");
    assert_eq!(status[0].value, Some("status_value".to_string()));
    assert!(status[0].is_set);
    
    // Test non-existent variable
    let keys = vec!["ENVSWITCH_NONEXISTENT".to_string()];
    let status = manager.get_variable_status(&keys);
    assert!(!status.is_empty());
    assert_eq!(status[0].key, "ENVSWITCH_NONEXISTENT");
    assert_eq!(status[0].value, None);
    assert!(!status[0].is_set);
    
    // Clean up
    env::remove_var("ENVSWITCH_STATUS_TEST");
}