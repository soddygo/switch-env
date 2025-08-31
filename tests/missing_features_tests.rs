use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

use envswitch::config::{ConfigManager, FileConfigManager};
use envswitch::commands::import_export::{handle_export_command, handle_import_command};
use envswitch::commands::config_commands::handle_delete_command;
use envswitch::utils::file_utils::{detect_file_format, validate_file_format, FileFormat};
use envswitch::utils::feedback::{format_file_size, ProgressIndicator};
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

/// Helper function to create test JSON file
fn create_test_json_file(path: &Path, vars: &HashMap<String, String>) {
    use serde_json::json;
    
    let config_data = json!({
        "configs": {
            "test_config": {
                "alias": "test_config",
                "variables": vars,
                "description": "Test configuration",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z"
            }
        },
        "active_config": null
    });
    
    let json_content = serde_json::to_string_pretty(&config_data).unwrap();
    fs::write(path, json_content).unwrap();
}

/// Helper function to create test ENV file
fn create_test_env_file(path: &Path, vars: &HashMap<String, String>) {
    let mut content = String::new();
    for (key, value) in vars {
        content.push_str(&format!("{}={}\n", key, value));
    }
    fs::write(path, content).unwrap();
}

#[cfg(test)]
mod export_command_tests {
    use super::*;

    #[test]
    fn test_export_command_with_default_output() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create test configuration
        let test_vars = create_test_env_vars();
        config_manager.create_config("test_config".to_string(), test_vars, None)
            .expect("Failed to create test config");
        
        // Test export with default output file
        let result = handle_export_command(
            &config_manager,
            None, // Default output
            vec![], // All configs
            "json".to_string(),
            false, // No metadata
            false, // No pretty print
            false, // Not verbose
        );
        
        assert!(result.is_ok());
        
        // Check if default export file was created
        let default_export_path = Path::new("envswitch_export.json");
        if default_export_path.exists() {
            fs::remove_file(default_export_path).ok();
        }
    }

    #[test]
    fn test_export_command_with_specific_configs() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create multiple test configurations
        let test_vars1 = create_test_env_vars();
        config_manager.create_config("config1".to_string(), test_vars1, None)
            .expect("Failed to create config1");
        
        let mut test_vars2 = HashMap::new();
        test_vars2.insert("VAR1".to_string(), "value1".to_string());
        config_manager.create_config("config2".to_string(), test_vars2, None)
            .expect("Failed to create config2");
        
        // Test export with specific configs
        let export_path = config_paths.config_dir.join("specific_export.json");
        let result = handle_export_command(
            &config_manager,
            Some(export_path.to_string_lossy().to_string()),
            vec!["config1".to_string()], // Only config1
            "json".to_string(),
            true, // Include metadata
            true, // Pretty print
            false, // Not verbose
        );
        
        assert!(result.is_ok());
        assert!(export_path.exists());
        
        // Verify export content
        let export_content = fs::read_to_string(&export_path).unwrap();
        assert!(export_content.contains("config1"));
        assert!(!export_content.contains("config2")); // Should not include config2
    }

    #[test]
    fn test_export_command_invalid_format() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths);
        
        // Test export with invalid format
        let result = handle_export_command(
            &config_manager,
            Some("test.txt".to_string()),
            vec![],
            "invalid_format".to_string(),
            false,
            false,
            false,
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported format"));
    }

    #[test]
    fn test_export_command_nonexistent_config() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths);
        
        // Test export with non-existent config
        let result = handle_export_command(
            &config_manager,
            Some("test.json".to_string()),
            vec!["nonexistent".to_string()],
            "json".to_string(),
            false,
            false,
            false,
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_export_command_different_formats() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create test configuration
        let test_vars = create_test_env_vars();
        config_manager.create_config("test_config".to_string(), test_vars, None)
            .expect("Failed to create test config");
        
        // Test JSON export
        let json_path = config_paths.config_dir.join("export.json");
        let result = handle_export_command(
            &config_manager,
            Some(json_path.to_string_lossy().to_string()),
            vec![],
            "json".to_string(),
            false,
            false,
            false,
        );
        assert!(result.is_ok());
        assert!(json_path.exists());
        
        // Test ENV export
        let env_path = config_paths.config_dir.join("export.env");
        let result = handle_export_command(
            &config_manager,
            Some(env_path.to_string_lossy().to_string()),
            vec![],
            "env".to_string(),
            false,
            false,
            false,
        );
        assert!(result.is_ok());
        assert!(env_path.exists());
        
        // Test YAML export
        let yaml_path = config_paths.config_dir.join("export.yaml");
        let result = handle_export_command(
            &config_manager,
            Some(yaml_path.to_string_lossy().to_string()),
            vec![],
            "yaml".to_string(),
            false,
            false,
            false,
        );
        assert!(result.is_ok());
        assert!(yaml_path.exists());
    }
}

#[cfg(test)]
mod import_command_tests {
    use super::*;

    #[test]
    fn test_import_command_json_file() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create test JSON import file
        let test_vars = create_test_env_vars();
        let import_path = config_paths.config_dir.join("import.json");
        create_test_json_file(&import_path, &test_vars);
        
        // Test import
        let result = handle_import_command(
            &config_manager,
            import_path.to_string_lossy().to_string(),
            false, // Not force
            false, // Not merge
            false, // Not dry run
            false, // Don't skip validation
            false, // No backup
            false, // Not verbose
        );
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_import_command_env_file() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create test ENV import file
        let test_vars = create_test_env_vars();
        let import_path = config_paths.config_dir.join("import.env");
        create_test_env_file(&import_path, &test_vars);
        
        // Test import
        let result = handle_import_command(
            &config_manager,
            import_path.to_string_lossy().to_string(),
            false,
            false,
            false,
            false,
            false,
            false,
        );
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_import_command_nonexistent_file() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths);
        
        // Test import with non-existent file
        let result = handle_import_command(
            &config_manager,
            "nonexistent.json".to_string(),
            false,
            false,
            false,
            false,
            false,
            false,
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_import_command_dry_run() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create test JSON import file
        let test_vars = create_test_env_vars();
        let import_path = config_paths.config_dir.join("import.json");
        create_test_json_file(&import_path, &test_vars);
        
        // Test dry run import
        let result = handle_import_command(
            &config_manager,
            import_path.to_string_lossy().to_string(),
            false,
            false,
            true, // Dry run
            false,
            false,
            false,
        );
        
        assert!(result.is_ok());
        
        // Verify no configurations were actually imported
        let configs = config_manager.list_configs().unwrap();
        assert!(configs.is_empty());
    }

    #[test]
    fn test_import_command_with_backup() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create existing configuration
        let existing_vars = create_test_env_vars();
        config_manager.create_config("existing".to_string(), existing_vars, None)
            .expect("Failed to create existing config");
        
        // Create test JSON import file
        let import_vars = HashMap::from([
            ("NEW_VAR".to_string(), "new_value".to_string()),
        ]);
        let import_path = config_paths.config_dir.join("import.json");
        create_test_json_file(&import_path, &import_vars);
        
        // Test import with backup
        let result = handle_import_command(
            &config_manager,
            import_path.to_string_lossy().to_string(),
            false,
            false,
            false,
            false,
            true, // Create backup
            false,
        );
        
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod delete_command_tests {
    use super::*;

    #[test]
    fn test_delete_command_with_force() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths);
        
        // Create test configuration
        let test_vars = create_test_env_vars();
        config_manager.create_config("test_config".to_string(), test_vars, None)
            .expect("Failed to create test config");
        
        // Test delete with force flag
        let result = handle_delete_command(
            &config_manager,
            "test_config".to_string(),
            true, // Force
            false, // Not verbose
        );
        
        assert!(result.is_ok());
        
        // Verify configuration was deleted
        let config = config_manager.get_config("test_config").unwrap();
        assert!(config.is_none());
    }

    #[test]
    fn test_delete_command_nonexistent_config() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths);
        
        // Test delete non-existent configuration
        let result = handle_delete_command(
            &config_manager,
            "nonexistent".to_string(),
            true,
            false,
        );
        
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not found") || error_msg.contains("No configurations exist"));
    }

    #[test]
    fn test_delete_active_configuration() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths);
        
        // Create and set active configuration
        let test_vars = create_test_env_vars();
        config_manager.create_config("active_config".to_string(), test_vars, None)
            .expect("Failed to create active config");
        config_manager.set_active_config("active_config".to_string())
            .expect("Failed to set active config");
        
        // Test delete active configuration
        let result = handle_delete_command(
            &config_manager,
            "active_config".to_string(),
            true,
            false,
        );
        
        assert!(result.is_ok());
        
        // Verify active configuration was cleared
        let active = config_manager.get_active_config().unwrap();
        assert!(active.is_none());
    }
}

#[cfg(test)]
mod format_detection_tests {
    use super::*;

    #[test]
    fn test_detect_json_format() {
        let temp_dir = TempDir::new().unwrap();
        let json_path = temp_dir.path().join("test.json");
        
        let test_vars = create_test_env_vars();
        create_test_json_file(&json_path, &test_vars);
        
        let detected_format = detect_file_format(&json_path).unwrap();
        assert_eq!(detected_format, FileFormat::Json);
    }

    #[test]
    fn test_detect_env_format() {
        let temp_dir = TempDir::new().unwrap();
        let env_path = temp_dir.path().join("test.env");
        
        let test_vars = create_test_env_vars();
        create_test_env_file(&env_path, &test_vars);
        
        let detected_format = detect_file_format(&env_path).unwrap();
        assert_eq!(detected_format, FileFormat::Env);
    }

    #[test]
    fn test_detect_yaml_format() {
        let temp_dir = TempDir::new().unwrap();
        let yaml_path = temp_dir.path().join("test.yaml");
        
        // Create basic YAML content
        let yaml_content = "key1: value1\nkey2: value2\n";
        fs::write(&yaml_path, yaml_content).unwrap();
        
        let detected_format = detect_file_format(&yaml_path).unwrap();
        assert_eq!(detected_format, FileFormat::Yaml);
    }

    #[test]
    fn test_validate_json_format() {
        let temp_dir = TempDir::new().unwrap();
        let json_path = temp_dir.path().join("test.json");
        
        let test_vars = create_test_env_vars();
        create_test_json_file(&json_path, &test_vars);
        
        let validation = validate_file_format(&json_path, &FileFormat::Json).unwrap();
        assert!(validation.is_valid);
        assert_eq!(validation.format, Some(FileFormat::Json));
    }

    #[test]
    fn test_validate_invalid_json_format() {
        let temp_dir = TempDir::new().unwrap();
        let json_path = temp_dir.path().join("invalid.json");
        
        // Create invalid JSON
        fs::write(&json_path, "{ invalid json }").unwrap();
        
        let validation = validate_file_format(&json_path, &FileFormat::Json).unwrap();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }

    #[test]
    fn test_validate_env_format() {
        let temp_dir = TempDir::new().unwrap();
        let env_path = temp_dir.path().join("test.env");
        
        let test_vars = create_test_env_vars();
        create_test_env_file(&env_path, &test_vars);
        
        let validation = validate_file_format(&env_path, &FileFormat::Env).unwrap();
        assert!(validation.is_valid);
        assert_eq!(validation.format, Some(FileFormat::Env));
    }

    #[test]
    fn test_validate_invalid_env_format() {
        let temp_dir = TempDir::new().unwrap();
        let env_path = temp_dir.path().join("invalid.env");
        
        // Create invalid ENV content (missing = signs)
        fs::write(&env_path, "INVALID_LINE\nANOTHER_INVALID_LINE").unwrap();
        
        let validation = validate_file_format(&env_path, &FileFormat::Env).unwrap();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }

    #[test]
    fn test_detect_format_by_content() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test JSON content without .json extension
        let json_file = temp_dir.path().join("test.txt");
        let test_vars = create_test_env_vars();
        create_test_json_file(&json_file, &test_vars);
        
        let detected_format = detect_file_format(&json_file).unwrap();
        assert_eq!(detected_format, FileFormat::Json);
        
        // Test ENV content without .env extension
        let env_file = temp_dir.path().join("test2.txt");
        create_test_env_file(&env_file, &test_vars);
        
        let detected_format = detect_file_format(&env_file).unwrap();
        assert_eq!(detected_format, FileFormat::Env);
    }
}

#[cfg(test)]
mod feedback_utilities_tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
        assert_eq!(format_file_size(2048), "2.0 KB");
        assert_eq!(format_file_size(2560), "2.5 KB");
    }

    #[test]
    fn test_progress_indicator() {
        let mut progress = ProgressIndicator::new("Testing progress");
        
        // Test initial state
        assert!(!progress.is_running);
        
        // Test start
        progress.start();
        assert!(progress.is_running);
        
        // Test finish success
        progress.finish_success("Completed successfully");
        assert!(!progress.is_running);
        
        // Test restart and finish with error
        let mut progress2 = ProgressIndicator::new("Testing error");
        progress2.start();
        progress2.finish_error("Failed with error");
        assert!(!progress2.is_running);
        
        // Test restart and finish with warning
        let mut progress3 = ProgressIndicator::new("Testing warning");
        progress3.start();
        progress3.finish_warning("Completed with warnings");
        assert!(!progress3.is_running);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use envswitch::error::{ConfigError, validate_env_var_name, validate_config_name};

    #[test]
    fn test_config_error_user_messages() {
        let error = ConfigError::ConfigNotFound("test".to_string());
        let message = error.user_message();
        assert!(message.contains("test"));
        assert!(message.contains("envswitch list"));
    }

    #[test]
    fn test_validate_env_var_name() {
        // Valid names
        assert!(validate_env_var_name("VALID_NAME").is_ok());
        assert!(validate_env_var_name("_VALID").is_ok());
        assert!(validate_env_var_name("VAR123").is_ok());
        assert!(validate_env_var_name("A").is_ok());
        
        // Invalid names
        assert!(validate_env_var_name("").is_err());
        assert!(validate_env_var_name("123INVALID").is_err());
        assert!(validate_env_var_name("INVALID-NAME").is_err());
        assert!(validate_env_var_name("INVALID.NAME").is_err());
        assert!(validate_env_var_name("INVALID NAME").is_err());
    }

    #[test]
    fn test_validate_config_name() {
        // Valid names
        assert!(validate_config_name("valid-name").is_ok());
        assert!(validate_config_name("valid_name").is_ok());
        assert!(validate_config_name("ValidName123").is_ok());
        assert!(validate_config_name("a").is_ok());
        
        // Invalid names
        assert!(validate_config_name("").is_err());
        assert!(validate_config_name("-invalid").is_err());
        assert!(validate_config_name("invalid.name").is_err());
        assert!(validate_config_name("invalid name").is_err());
        assert!(validate_config_name(&"a".repeat(51)).is_err()); // Too long
    }
}

#[cfg(test)]
mod integration_workflow_tests {
    use super::*;

    #[test]
    fn test_complete_export_import_workflow() {
        // Setup source configuration
        let (_temp_dir1, config_paths1) = create_temp_config();
        let source_manager = FileConfigManager::with_paths(config_paths1.clone());
        
        // Create test configurations
        let deepseek_vars = create_test_env_vars();
        source_manager.create_config("deepseek".to_string(), deepseek_vars, Some("DeepSeek AI config".to_string()))
            .expect("Failed to create deepseek config");
        
        let mut kimi_vars = HashMap::new();
        kimi_vars.insert("ANTHROPIC_BASE_URL".to_string(), "https://api.moonshot.cn".to_string());
        kimi_vars.insert("ANTHROPIC_MODEL".to_string(), "moonshot-v1-8k".to_string());
        source_manager.create_config("kimi".to_string(), kimi_vars, Some("Kimi AI config".to_string()))
            .expect("Failed to create kimi config");
        
        // Export configurations
        let export_path = config_paths1.config_dir.join("full_export.json");
        let export_result = handle_export_command(
            &source_manager,
            Some(export_path.to_string_lossy().to_string()),
            vec![], // Export all
            "json".to_string(),
            true, // Include metadata
            true, // Pretty print
            false,
        );
        assert!(export_result.is_ok());
        assert!(export_path.exists());
        
        // Setup destination configuration
        let (_temp_dir2, config_paths2) = create_temp_config();
        let dest_manager = FileConfigManager::with_paths(config_paths2);
        
        // Import configurations
        let import_result = handle_import_command(
            &dest_manager,
            export_path.to_string_lossy().to_string(),
            false,
            false,
            false,
            false,
            false,
            false,
        );
        assert!(import_result.is_ok());
        
        // Verify imported configurations
        let imported_configs = dest_manager.list_configs().unwrap();
        assert!(imported_configs.contains(&"deepseek".to_string()));
        assert!(imported_configs.contains(&"kimi".to_string()));
        
        // Verify configuration details
        let imported_deepseek = dest_manager.get_config("deepseek").unwrap().unwrap();
        assert_eq!(imported_deepseek.description, Some("DeepSeek AI config".to_string()));
        assert!(imported_deepseek.variables.contains_key("ANTHROPIC_BASE_URL"));
    }

    #[test]
    fn test_cross_format_export_import() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create test configuration
        let test_vars = create_test_env_vars();
        config_manager.create_config("test_config".to_string(), test_vars.clone(), None)
            .expect("Failed to create test config");
        
        // Export as JSON
        let json_export = config_paths.config_dir.join("export.json");
        let export_result = handle_export_command(
            &config_manager,
            Some(json_export.to_string_lossy().to_string()),
            vec![],
            "json".to_string(),
            false,
            false,
            false,
        );
        assert!(export_result.is_ok());
        
        // Export as ENV
        let env_export = config_paths.config_dir.join("export.env");
        let export_result = handle_export_command(
            &config_manager,
            Some(env_export.to_string_lossy().to_string()),
            vec![],
            "env".to_string(),
            false,
            false,
            false,
        );
        assert!(export_result.is_ok());
        
        // Verify both files exist and have different formats
        assert!(json_export.exists());
        assert!(env_export.exists());
        
        let json_content = fs::read_to_string(&json_export).unwrap();
        let env_content = fs::read_to_string(&env_export).unwrap();
        
        assert!(json_content.contains("{"));
        assert!(env_content.contains("="));
        assert!(!env_content.contains("{"));
    }
}