use std::collections::HashMap;
use crate::config::{FileConfigManager, ConfigManager};
use crate::env::{ShellEnvironmentManager, EnvironmentManager};
use crate::handlers::interactive_env_input;
use crate::utils::{read_env_file, is_sensitive_key, mask_sensitive_value, is_claude_configuration, find_similar_configs};

/// Handle the set command to create or update configurations
pub fn handle_set_command(
    config_manager: &FileConfigManager,
    alias: String,
    env_vars: Vec<(String, String)>,
    description: Option<String>,
    file: Option<String>,
    replace: bool,
    interactive: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate alias name
    if alias.trim().is_empty() {
        return Err("Configuration name cannot be empty. Please provide a name for your configuration.".into());
    }
    
    // Check for invalid characters
    if !alias.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(format!(
            "Configuration name '{}' contains invalid characters. Use only letters, numbers, hyphens (-), and underscores (_).", 
            alias
        ).into());
    }
    
    // Check length
    if alias.len() > 50 {
        return Err("Configuration name is too long. Please use a name with 50 characters or less.".into());
    }
    
    if verbose {
        println!("Creating/updating configuration '{}'...", alias);
    }
    
    // Collect variables from different sources
    let mut variables: HashMap<String, String> = HashMap::new();
    
    // Add variables from command line
    variables.extend(env_vars.into_iter());
    
    // Add variables from file if specified
    if let Some(file_path) = file {
        let file_vars = read_env_file(&file_path)?;
        if verbose {
            println!("Read {} variables from file: {}", file_vars.len(), file_path);
        }
        variables.extend(file_vars);
    }
    
    // Interactive mode
    if interactive {
        variables.extend(interactive_env_input(verbose)?);
    }
    
    if variables.is_empty() {
        println!("No environment variables provided.");
        println!("Examples:");
        println!("  envswitch set {} -e ANTHROPIC_BASE_URL=https://api.deepseek.com -e ANTHROPIC_MODEL=deepseek-chat", alias);
        println!("  envswitch set {} -e API_KEY=your-key -d \"My API configuration\"", alias);
        return Ok(());
    }
    
    if verbose {
        println!("Variables to set ({}):", variables.len());
        let mut sorted_vars: Vec<_> = variables.iter().collect();
        sorted_vars.sort_by_key(|(k, _)| *k);
        for (key, value) in sorted_vars {
            // Mask sensitive values in verbose output
            let display_value = if is_sensitive_key(key) {
                mask_sensitive_value(value)
            } else {
                value.clone()
            };
            println!("  {} = {}", key, display_value);
        }
    }
    
    // Check if config already exists
    let existing_config = config_manager.get_config(&alias)?;
    let exists = existing_config.is_some();
    
    if exists {
        let existing = existing_config.unwrap();
        
        if verbose {
            println!("Updating existing configuration:");
            println!("  Created: {}", existing.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("  Previous variables: {}", existing.variables.len());
            
            // Show what's changing
            let mut added = Vec::new();
            let mut updated = Vec::new();
            let mut removed = Vec::new();
            
            for (key, new_value) in &variables {
                match existing.variables.get(key) {
                    Some(old_value) if old_value != new_value => updated.push(key.clone()),
                    None => added.push(key.clone()),
                    _ => {} // No change
                }
            }
            
            for key in existing.variables.keys() {
                if !variables.contains_key(key as &str) {
                    removed.push(key.clone());
                }
            }
            
            if !added.is_empty() {
                println!("  Adding: {}", added.join(", "));
            }
            if !updated.is_empty() {
                println!("  Updating: {}", updated.join(", "));
            }
            if !removed.is_empty() {
                println!("  Removing: {}", removed.join(", "));
            }
        }
        
        // Handle variable merging based on replace flag
        let final_variables = if replace {
            if verbose {
                println!("Replacing all variables (--replace mode)");
            }
            variables.clone()
        } else {
            // Merge with existing variables (update mode)
            let mut merged_variables = existing.variables.clone();
            merged_variables.extend(variables.clone());
            merged_variables
        };
        
        let var_count = final_variables.len();
        config_manager.update_config(alias.clone(), final_variables, description.clone())?;
        println!("✅ Configuration '{}' updated successfully!", alias);
        
        if verbose {
            println!("  Total variables: {}", var_count);
        }
    } else {
        config_manager.create_config(alias.clone(), variables.clone(), description.clone())?;
        println!("✅ Configuration '{}' created successfully!", alias);
        println!("📝 {} environment variables configured", variables.len());
        if let Some(desc) = description {
            println!("📄 Description: {}", desc);
        }
        println!();
        println!("🚀 Next steps:");
        println!("   envswitch use {}           # Activate this configuration", alias);
        println!("   envswitch show {}          # View configuration details", alias);
        println!("   envswitch list             # See all configurations");
    }
    
    // Detect if this looks like a Claude configuration
    if is_claude_configuration(&variables) {
        println!("💡 This appears to be a Claude configuration. Use 'envswitch status --claude' to check Claude variables.");
    }
    
    if verbose {
        println!("Configuration saved to: {}", config_manager.config_file_path().display());
    }
    
    Ok(())
}

/// Handle the use command to switch configurations
pub fn handle_use_command(
    config_manager: &FileConfigManager,
    env_manager: &ShellEnvironmentManager,
    alias: String,
    dry_run: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate alias
    if alias.trim().is_empty() {
        return Err("Configuration name cannot be empty. Please specify which configuration to use.".into());
    }
    
    let config = config_manager.get_config(&alias)?
        .ok_or_else(|| {
            let available_configs = config_manager.list_configs().unwrap_or_default();
            if available_configs.is_empty() {
                format!("Configuration '{}' not found. No configurations exist yet.\n💡 Create your first configuration with: envswitch set {} -e KEY=value", alias, alias)
            } else {
                let suggestions = find_similar_configs(&alias, &available_configs);
                if suggestions.is_empty() {
                    format!("Configuration '{}' not found.\nAvailable configurations: {}\n💡 Use 'envswitch list' to see all configurations", 
                        alias, available_configs.join(", "))
                } else {
                    format!("Configuration '{}' not found.\nDid you mean: {}?\nAvailable configurations: {}", 
                        alias, suggestions.join(", "), available_configs.join(", "))
                }
            }
        })?;
    
    if verbose {
        println!("Switching to configuration: {}", alias);
        println!("Description: {}", config.description.as_deref().unwrap_or("No description"));
        println!("Variables: {}", config.variables.len());
        println!("Created: {}", config.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Updated: {}", config.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));
    }
    
    let commands = env_manager.generate_shell_commands(&config.variables)?;
    
    if dry_run {
        println!("# Commands that would be executed:");
        println!("{}", commands);
        return Ok(());
    }
    
    // Set as active configuration
    config_manager.set_active_config(alias.clone())?;
    
    // Output the commands for shell evaluation
    println!("{}", commands);
    
    if verbose {
        println!("# Configuration '{}' activated", alias);
        println!("# {} environment variables set", config.variables.len());
    }
    
    Ok(())
}

/// Handle the list command to show all configurations
pub fn handle_list_command(
    config_manager: &FileConfigManager, 
    verbose: bool, 
    table: bool, 
    active: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let configs = config_manager.list_configs()?;
    
    if configs.is_empty() {
        println!("📭 No configurations found");
        println!();
        println!("🚀 Get started by creating your first configuration:");
        println!("   envswitch set my-config -e API_KEY=your-key -e ENV=development");
        println!();
        println!("💡 Or try the tutorial:");
        println!("   envswitch tutorial");
        return Ok(());
    }
    
    if active {
        // Show only active configuration
        if let Some(active_config) = config_manager.get_active_config()? {
            println!("Active configuration: {}", active_config);
        } else {
            println!("No active configuration");
        }
        return Ok(());
    }
    
    if table {
        display_configs_table(&configs, config_manager, verbose)?;
    } else {
        display_configs_list(&configs, config_manager, verbose)?;
    }
    
    Ok(())
}

/// Handle the status command to show current environment status
pub fn handle_status_command(
    config_manager: &FileConfigManager,
    env_manager: &ShellEnvironmentManager,
    claude: bool,
    table: bool,
    mismatched: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if claude {
        display_claude_status(env_manager, table, verbose)?;
        return Ok(());
    }
    
    // Get active configuration
    let active_config_name = config_manager.get_active_config()?;
    
    if let Some(config_name) = active_config_name {
        let config = config_manager.get_config(&config_name)?
            .ok_or_else(|| format!("Active configuration '{}' not found", config_name))?;
        
        println!("Active configuration: {}", config_name);
        if let Some(description) = &config.description {
            println!("Description: {}", description);
        }
        println!("Variables: {}", config.variables.len());
        println!("Created: {}", config.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Updated: {}", config.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!();
        
        // Check environment variable status
        let keys: Vec<String> = config.variables.keys().cloned().collect();
        let statuses = env_manager.get_variable_status(&keys);
        
        if mismatched {
            let mismatched_vars: Vec<_> = statuses.iter()
                .filter(|status| {
                    let expected_value = config.variables.get(&status.key);
                    !expected_value.map_or(false, |expected| {
                        status.value.as_deref() == Some(expected)
                    })
                })
                .cloned()
                .collect();
            
            if mismatched_vars.is_empty() {
                println!("✅ All variables match expected values");
            } else {
                println!("⚠️  {} variables don't match expected values:", mismatched_vars.len());
                if table {
                    display_status_table(&mismatched_vars, &config.variables, verbose)?;
                } else {
                    display_status_list(&mismatched_vars, &config.variables, verbose)?;
                }
            }
        } else {
            if table {
                display_status_table(&statuses, &config.variables, verbose)?;
            } else {
                display_status_list(&statuses, &config.variables, verbose)?;
            }
        }
    } else {
        println!("No active configuration");
        println!("Use 'envswitch use <config-name>' to activate a configuration");
        
        let configs = config_manager.list_configs()?;
        if !configs.is_empty() {
            println!();
            println!("Available configurations:");
            for config in configs {
                println!("  {}", config);
            }
        }
    }
    
    Ok(())
}



// Import display functions that will be moved to handlers module
use crate::handlers::{display_configs_table, display_configs_list, display_claude_status, display_status_table, display_status_list};
// Handle the edit command to interactively edit a configuration
pub fn handle_edit_command(
    config_manager: &FileConfigManager,
    alias: String,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // This function will be moved from main.rs
    // For now, return a placeholder
    println!("Edit command - to be implemented");
    Ok(())
}

/// Handle the delete command to remove a configuration
pub fn handle_delete_command(
    config_manager: &FileConfigManager,
    alias: String,
    force: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // This function will be moved from main.rs
    // For now, return a placeholder
    println!("Delete command - to be implemented");
    Ok(())
}