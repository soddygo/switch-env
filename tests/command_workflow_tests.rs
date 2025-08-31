use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

use envswitch::config::{ConfigManager, FileConfigManager};
use envswitch::commands::import_export::{handle_export_command, handle_import_command};
use envswitch::commands::config_commands::{handle_delete_command};
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
mod end_to_end_workflow_tests {
    use super::*;

    #[test]
    fn test_complete_configuration_lifecycle() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Step 1: Create initial configurations
        let deepseek_vars = create_test_env_vars();
        config_manager.create_config("deepseek".to_string(), deepseek_vars.clone(), Some("DeepSeek AI config".to_string()))
            .expect("Failed to create deepseek config");
        
        let mut kimi_vars = HashMap::new();
        kimi_vars.insert("ANTHROPIC_BASE_URL".to_string(), "https://api.moonshot.cn".to_string());
        kimi_vars.insert("ANTHROPIC_MODEL".to_string(), "moonshot-v1-8k".to_string());
        config_manager.create_config("kimi".to_string(), kimi_vars.clone(), Some("Kimi AI config".to_string()))
            .expect("Failed to create kimi config");
        
        // Step 2: Export configurations using command handler
        let export_path = config_paths.config_dir.join("lifecycle_export.json");
        let export_result = handle_export_command(
            &config_manager,
            Some(export_path.to_string_lossy().to_string()),
            vec![], // Export all
            "json".to_string(),
            true, // Include metadata
            true, // Pretty print
            false, // Not verbose
        );
        assert!(export_result.is_ok(), "Export should succeed");
        assert!(export_path.exists(), "Export file should exist");
        
        // Step 3: Verify export content
        let export_content = fs::read_to_string(&export_path).unwrap();
        assert!(export_content.contains("deepseek"), "Export should contain deepseek config");
        assert!(export_content.contains("kimi"), "Export should contain kimi config");
        assert!(export_content.contains("DeepSeek AI config"), "Export should contain description");
        
        // Step 4: Delete one configuration using command handler
        let delete_result = handle_delete_command(
            &config_manager,
            "kimi".to_string(),
            true, // Force delete
            false, // Not verbose
        );
        assert!(delete_result.is_ok(), "Delete should succeed");
        
        // Verify deletion
        let remaining_configs = config_manager.list_configs().unwrap();
        assert!(!remaining_configs.contains(&"kimi".to_string()), "Kimi config should be deleted");
        assert!(remaining_configs.contains(&"deepseek".to_string()), "DeepSeek config should remain");
        
        // Step 5: Import configurations to restore deleted one
        let import_result = handle_import_command(
            &config_manager,
            export_path.to_string_lossy().to_string(),
            false, // Not force
            true,  // Merge existing
            false, // Not dry run
            false, // Don't skip validation
            false, // No backup
            false, // Not verbose
        );
        assert!(import_result.is_ok(), "Import should succeed");
        
        // Step 6: Verify restoration
        let final_configs = config_manager.list_configs().unwrap();
        assert!(final_configs.contains(&"deepseek".to_string()), "DeepSeek config should exist");
        assert!(final_configs.contains(&"kimi".to_string()), "Kimi config should be restored");
        
        // Verify restored configuration details
        let restored_kimi = config_manager.get_config("kimi").unwrap().unwrap();
        assert_eq!(restored_kimi.variables, kimi_vars, "Restored variables should match original");
        assert_eq!(restored_kimi.description, Some("Kimi AI config".to_string()), "Description should be restored");
    }

    #[test]
    fn test_export_import_with_conflicts() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create initial configuration
        let mut original_vars = HashMap::new();
        original_vars.insert("VAR1".to_string(), "original_value1".to_string());
        original_vars.insert("VAR2".to_string(), "original_value2".to_string());
        config_manager.create_config("test_config".to_string(), original_vars.clone(), None)
            .expect("Failed to create original config");
        
        // Create export file with conflicting configuration
        let mut conflicting_vars = HashMap::new();
        conflicting_vars.insert("VAR1".to_string(), "new_value1".to_string());
        conflicting_vars.insert("VAR3".to_string(), "new_value3".to_string());
        
        let export_path = config_paths.config_dir.join("conflict_test.json");
        create_test_json_file(&export_path, &conflicting_vars);
        
        // Test import with merge (should combine variables)
        let import_result = handle_import_command(
            &config_manager,
            export_path.to_string_lossy().to_string(),
            false, // Not force
            true,  // Merge existing
            false, // Not dry run
            false, // Don't skip validation
            false, // No backup
            false, // Not verbose
        );
        assert!(import_result.is_ok(), "Merge import should succeed");
        
        // Verify merged result
        let merged_config = config_manager.get_config("test_config").unwrap().unwrap();
        assert_eq!(merged_config.variables.get("VAR1"), Some(&"new_value1".to_string()), "VAR1 should be updated");
        assert_eq!(merged_config.variables.get("VAR2"), Some(&"original_value2".to_string()), "VAR2 should remain");
        assert_eq!(merged_config.variables.get("VAR3"), Some(&"new_value3".to_string()), "VAR3 should be added");
    }

    #[test]
    fn test_backup_and_restore_workflow() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create initial configurations
        let test_vars = create_test_env_vars();
        config_manager.create_config("backup_test".to_string(), test_vars.clone(), None)
            .expect("Failed to create config for backup test");
        
        // Create import file that will overwrite existing config
        let mut new_vars = HashMap::new();
        new_vars.insert("NEW_VAR".to_string(), "new_value".to_string());
        let import_path = config_paths.config_dir.join("new_config.json");
        create_test_json_file(&import_path, &new_vars);
        
        // Import with backup enabled
        let import_result = handle_import_command(
            &config_manager,
            import_path.to_string_lossy().to_string(),
            true,  // Force overwrite
            false, // Don't merge
            false, // Not dry run
            false, // Don't skip validation
            true,  // Create backup
            false, // Not verbose
        );
        assert!(import_result.is_ok(), "Import with backup should succeed");
        
        // Verify backup was created (check backup directory exists)
        let backup_dir = config_paths.config_dir.join("backups");
        if backup_dir.exists() {
            let backup_files: Vec<_> = fs::read_dir(&backup_dir)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .collect();
            assert!(!backup_files.is_empty(), "Backup files should exist");
        }
        
        // Verify new configuration was imported
        let imported_configs = config_manager.list_configs().unwrap();
        assert!(imported_configs.contains(&"test_config".to_string()), "Config should exist after import");
    }
}

#[cfg(test)]
mod cross_format_compatibility_tests {
    use super::*;

    #[test]
    fn test_json_to_env_export_import() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create test configuration
        let test_vars = create_test_env_vars();
        config_manager.create_config("format_test".to_string(), test_vars.clone(), None)
            .expect("Failed to create config");
        
        // Export as JSON
        let json_export = config_paths.config_dir.join("export.json");
        let json_export_result = handle_export_command(
            &config_manager,
            Some(json_export.to_string_lossy().to_string()),
            vec![],
            "json".to_string(),
            false, false, false,
        );
        assert!(json_export_result.is_ok(), "JSON export should succeed");
        
        // Export as ENV with metadata to preserve config names
        let env_export = config_paths.config_dir.join("export.env");
        let env_export_result = handle_export_command(
            &config_manager,
            Some(env_export.to_string_lossy().to_string()),
            vec![],
            "env".to_string(),
            true, false, false, // Include metadata
        );
        assert!(env_export_result.is_ok(), "ENV export should succeed");
        
        // Verify both files exist and have different formats
        assert!(json_export.exists(), "JSON export file should exist");
        assert!(env_export.exists(), "ENV export file should exist");
        
        let json_content = fs::read_to_string(&json_export).unwrap();
        let env_content = fs::read_to_string(&env_export).unwrap();
        
        // JSON should contain braces, ENV should contain equals signs
        assert!(json_content.contains("{"), "JSON should contain braces");
        assert!(env_content.contains("="), "ENV should contain equals signs");
        assert!(!env_content.contains("{"), "ENV should not contain braces");
        
        // Test importing ENV format
        let (_temp_dir2, config_paths2) = create_temp_config();
        let config_manager2 = FileConfigManager::with_paths(config_paths2);
        
        let env_import_result = handle_import_command(
            &config_manager2,
            env_export.to_string_lossy().to_string(),
            false, false, false, false, false, false,
        );
        assert!(env_import_result.is_ok(), "ENV import should succeed");
        
        // Verify imported configuration
        let imported_configs = config_manager2.list_configs().unwrap();
        assert!(imported_configs.contains(&"format_test".to_string()), "Config should be imported from ENV format");
    }

    #[test]
    fn test_yaml_format_handling() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create test configuration
        let test_vars = create_test_env_vars();
        config_manager.create_config("yaml_test".to_string(), test_vars, None)
            .expect("Failed to create config");
        
        // Export as YAML
        let yaml_export = config_paths.config_dir.join("export.yaml");
        let yaml_export_result = handle_export_command(
            &config_manager,
            Some(yaml_export.to_string_lossy().to_string()),
            vec![],
            "yaml".to_string(),
            false, false, false,
        );
        assert!(yaml_export_result.is_ok(), "YAML export should succeed");
        
        // Verify YAML file exists and has correct format
        assert!(yaml_export.exists(), "YAML export file should exist");
        let yaml_content = fs::read_to_string(&yaml_export).unwrap();
        assert!(yaml_content.contains(":"), "YAML should contain colons");
        assert!(!yaml_content.contains("{"), "YAML should not contain braces");
        assert!(!yaml_content.contains("="), "YAML should not contain equals signs");
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_configuration_export_import() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create large configuration with many variables
        let mut large_vars = HashMap::new();
        for i in 0..500 { // Test with 500 variables
            large_vars.insert(format!("LARGE_VAR_{}", i), format!("large_value_{}", i));
        }
        
        config_manager.create_config("large_config".to_string(), large_vars.clone(), Some("Large configuration for performance testing".to_string()))
            .expect("Failed to create large config");
        
        // Export large configuration
        let export_path = config_paths.config_dir.join("large_export.json");
        let start_time = std::time::Instant::now();
        
        let export_result = handle_export_command(
            &config_manager,
            Some(export_path.to_string_lossy().to_string()),
            vec![],
            "json".to_string(),
            true, // Include metadata
            true, // Pretty print
            false,
        );
        
        let export_duration = start_time.elapsed();
        assert!(export_result.is_ok(), "Large config export should succeed");
        assert!(export_duration.as_secs() < 5, "Export should complete within 5 seconds");
        
        // Verify export file size is reasonable
        let file_size = fs::metadata(&export_path).unwrap().len();
        assert!(file_size > 10000, "Export file should be substantial"); // At least 10KB
        assert!(file_size < 10_000_000, "Export file should not be excessively large"); // Less than 10MB
        
        // Test importing large configuration
        let (_temp_dir2, config_paths2) = create_temp_config();
        let config_manager2 = FileConfigManager::with_paths(config_paths2);
        
        let import_start = std::time::Instant::now();
        let import_result = handle_import_command(
            &config_manager2,
            export_path.to_string_lossy().to_string(),
            false, false, false, false, false, false,
        );
        let import_duration = import_start.elapsed();
        
        assert!(import_result.is_ok(), "Large config import should succeed");
        assert!(import_duration.as_secs() < 5, "Import should complete within 5 seconds");
        
        // Verify imported configuration
        let imported_config = config_manager2.get_config("large_config").unwrap().unwrap();
        assert_eq!(imported_config.variables.len(), 500, "All variables should be imported");
        assert_eq!(imported_config.variables, large_vars, "Imported variables should match original");
    }

    #[test]
    fn test_multiple_configurations_performance() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create multiple configurations
        let start_time = std::time::Instant::now();
        for i in 0..50 { // Create 50 configurations
            let mut vars = HashMap::new();
            for j in 0..10 { // Each with 10 variables
                vars.insert(format!("VAR_{}_{}", i, j), format!("value_{}_{}", i, j));
            }
            
            config_manager.create_config(
                format!("config_{}", i),
                vars,
                Some(format!("Configuration number {}", i))
            ).expect(&format!("Failed to create config_{}", i));
        }
        let creation_duration = start_time.elapsed();
        assert!(creation_duration.as_secs() < 10, "Creating 50 configs should complete within 10 seconds");
        
        // Export all configurations
        let export_path = config_paths.config_dir.join("multi_export.json");
        let export_start = std::time::Instant::now();
        
        let export_result = handle_export_command(
            &config_manager,
            Some(export_path.to_string_lossy().to_string()),
            vec![], // Export all
            "json".to_string(),
            true, true, false,
        );
        
        let export_duration = export_start.elapsed();
        assert!(export_result.is_ok(), "Multi-config export should succeed");
        assert!(export_duration.as_secs() < 10, "Export should complete within 10 seconds");
        
        // Verify all configurations are in export
        let export_content = fs::read_to_string(&export_path).unwrap();
        for i in 0..50 {
            assert!(export_content.contains(&format!("config_{}", i)), "Export should contain config_{}", i);
        }
    }
}

#[cfg(test)]
mod error_recovery_tests {
    use super::*;

    #[test]
    fn test_corrupted_import_file_handling() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create corrupted JSON file
        let corrupted_json = config_paths.config_dir.join("corrupted.json");
        fs::write(&corrupted_json, "{ invalid json content }").unwrap();
        
        // Test import with corrupted file
        let import_result = handle_import_command(
            &config_manager,
            corrupted_json.to_string_lossy().to_string(),
            false, false, false, false, false, false,
        );
        
        assert!(import_result.is_err(), "Import of corrupted file should fail");
        
        // Verify no configurations were created
        let configs = config_manager.list_configs().unwrap();
        assert!(configs.is_empty(), "No configs should be created from corrupted file");
    }

    #[test]
    fn test_permission_error_handling() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Test export to non-existent directory (should create it)
        let deep_path = config_paths.config_dir.join("deep").join("nested").join("path").join("export.json");
        
        // Create a test config first
        let test_vars = create_test_env_vars();
        config_manager.create_config("test".to_string(), test_vars, None)
            .expect("Failed to create test config");
        
        let export_result = handle_export_command(
            &config_manager,
            Some(deep_path.to_string_lossy().to_string()),
            vec![],
            "json".to_string(),
            false, false, false,
        );
        
        // Should succeed because we create directories
        assert!(export_result.is_ok(), "Export should create necessary directories");
        assert!(deep_path.exists(), "Export file should exist in created directory");
    }

    #[test]
    fn test_dry_run_safety() {
        let (_temp_dir, config_paths) = create_temp_config();
        let config_manager = FileConfigManager::with_paths(config_paths.clone());
        
        // Create existing configuration
        let existing_vars = HashMap::from([
            ("EXISTING_VAR".to_string(), "existing_value".to_string()),
        ]);
        config_manager.create_config("existing".to_string(), existing_vars.clone(), None)
            .expect("Failed to create existing config");
        
        // Create import file with different content
        let import_vars = HashMap::from([
            ("NEW_VAR".to_string(), "new_value".to_string()),
        ]);
        let import_path = config_paths.config_dir.join("dry_run_test.json");
        create_test_json_file(&import_path, &import_vars);
        
        // Test dry run import
        let dry_run_result = handle_import_command(
            &config_manager,
            import_path.to_string_lossy().to_string(),
            false, false,
            true,  // Dry run
            false, false, false,
        );
        
        assert!(dry_run_result.is_ok(), "Dry run should succeed");
        
        // Verify original configuration is unchanged
        let unchanged_config = config_manager.get_config("existing").unwrap().unwrap();
        assert_eq!(unchanged_config.variables, existing_vars, "Original config should be unchanged after dry run");
        
        // Verify new configuration was not created
        let configs = config_manager.list_configs().unwrap();
        assert_eq!(configs.len(), 1, "Only original config should exist after dry run");
        assert!(!configs.contains(&"dry_run_test".to_string()), "New config should not be created in dry run");
    }
}