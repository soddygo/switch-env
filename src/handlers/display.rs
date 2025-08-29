use std::collections::HashMap;
use crate::config::{FileConfigManager, ConfigManager};
use crate::env::{ShellEnvironmentManager, EnvVarStatus, EnvironmentManager};
use crate::utils::{is_sensitive_key, mask_sensitive_value};

/// Display configurations in list format
pub fn display_configs_list(
    configs: &[String],
    config_manager: &FileConfigManager,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let active_config = config_manager.get_active_config()?;
    
    println!("Available configurations:");
    
    for config_alias in configs {
        let is_active = active_config.as_ref() == Some(config_alias);
        let marker = if is_active { " (active)" } else { "" };
        
        if let Ok(Some(config)) = config_manager.get_config(config_alias) {
            let var_count = config.variables.len();
            let desc = config.description.as_deref().unwrap_or("No description");
            
            if verbose {
                println!("  {} - {} ({} variables){}", config_alias, desc, var_count, marker);
                println!("    Created: {}", config.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
                println!("    Updated: {}", config.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));
                
                if !config.variables.is_empty() {
                    println!("    Variables:");
                    let mut sorted_vars: Vec<_> = config.variables.iter().collect();
                    sorted_vars.sort_by_key(|(k, _)| *k);
                    
                    for (key, value) in sorted_vars {
                        let display_value = if is_sensitive_key(key) {
                            mask_sensitive_value(value)
                        } else if value.len() > 50 {
                            format!("{}...", &value[..47])
                        } else {
                            value.to_string()
                        };
                        println!("      {} = {}", key, display_value);
                    }
                }
                println!();
            } else {
                println!("  {} - {} ({} variables){}", config_alias, desc, var_count, marker);
            }
        } else {
            println!("  {}{}", config_alias, marker);
        }
    }
    
    Ok(())
}

/// Display configurations in table format
pub fn display_configs_table(
    configs: &[String],
    config_manager: &FileConfigManager,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let active_config = config_manager.get_active_config()?;
    
    // Calculate column widths
    let mut max_name_width = 4; // "Name"
    let mut max_desc_width = 11; // "Description"
    let mut max_vars_width = 9; // "Variables"
    
    for config_alias in configs {
        max_name_width = max_name_width.max(config_alias.len());
        
        if let Ok(Some(config)) = config_manager.get_config(config_alias) {
            let desc = config.description.as_deref().unwrap_or("No description");
            max_desc_width = max_desc_width.max(desc.len().min(50));
            max_vars_width = max_vars_width.max(config.variables.len().to_string().len());
        }
    }
    
    // Add padding
    max_name_width += 2;
    max_desc_width += 2;
    max_vars_width += 2;
    
    // Print header
    println!("{:<width_name$} {:<width_desc$} {:<width_vars$} {:<8} {:<19}",
        "Name", "Description", "Variables", "Active", "Updated",
        width_name = max_name_width,
        width_desc = max_desc_width,
        width_vars = max_vars_width
    );
    
    println!("{} {} {} {} {}",
        "-".repeat(max_name_width),
        "-".repeat(max_desc_width),
        "-".repeat(max_vars_width),
        "-".repeat(8),
        "-".repeat(19)
    );
    
    // Print configurations
    for config_alias in configs {
        let is_active = active_config.as_ref() == Some(config_alias);
        let active_marker = if is_active { "✓" } else { "" };
        
        if let Ok(Some(config)) = config_manager.get_config(config_alias) {
            let desc = config.description.as_deref().unwrap_or("No description");
            let truncated_desc = if desc.len() > 50 {
                format!("{}...", &desc[..47])
            } else {
                desc.to_string()
            };
            
            println!("{:<width_name$} {:<width_desc$} {:<width_vars$} {:<8} {}",
                config_alias,
                truncated_desc,
                config.variables.len(),
                active_marker,
                config.updated_at.format("%Y-%m-%d %H:%M:%S"),
                width_name = max_name_width,
                width_desc = max_desc_width,
                width_vars = max_vars_width
            );
            
            if verbose && !config.variables.is_empty() {
                println!("  Variables:");
                let mut sorted_vars: Vec<_> = config.variables.iter().collect();
                sorted_vars.sort_by_key(|(k, _)| *k);
                
                for (key, value) in sorted_vars {
                    let display_value = if is_sensitive_key(key) {
                        mask_sensitive_value(value)
                    } else if value.len() > 40 {
                        format!("{}...", &value[..37])
                    } else {
                        value.clone()
                    };
                    println!("    {} = {}", key, display_value);
                }
                println!();
            }
        } else {
            println!("{:<width_name$} {:<width_desc$} {:<width_vars$} {:<8} {}",
                config_alias,
                "Error loading config",
                "?",
                active_marker,
                "Unknown",
                width_name = max_name_width,
                width_desc = max_desc_width,
                width_vars = max_vars_width
            );
        }
    }
    
    Ok(())
}

/// Display Claude-specific status
pub fn display_claude_status(
    env_manager: &ShellEnvironmentManager,
    table: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let claude_vars = [
        "ANTHROPIC_BASE_URL",
        "ANTHROPIC_MODEL",
        "ANTHROPIC_AUTH_TOKEN",
        "ANTHROPIC_SMALL_FAST_MODEL",
        "ANTHROPIC_API_KEY",
    ];
    
    let mut claude_env_vars = HashMap::new();
    for var in &claude_vars {
        if let Some(value) = env_manager.get_variable(var) {
            claude_env_vars.insert(var.to_string(), value);
        }
    }
    
    if claude_env_vars.is_empty() {
        println!("No Claude environment variables found");
        println!("Common Claude variables:");
        for var in &claude_vars {
            println!("  {}", var);
        }
        return Ok(());
    }
    
    println!("Claude Environment Variables:");
    
    let keys: Vec<String> = claude_env_vars.keys().cloned().collect();
    let statuses = env_manager.get_variable_status(&keys);
    
    if table {
        display_claude_status_table(&statuses, verbose)?;
    } else {
        for status in &statuses {
            let value_display = if is_sensitive_key(&status.key) {
                mask_sensitive_value(&status.value.as_deref().unwrap_or(""))
            } else {
                status.value.as_deref().unwrap_or("(not set)").to_string()
            };
            
            println!("  {} = {}", status.key, value_display);
        }
    }
    
    Ok(())
}

/// Display Claude status in table format
pub fn display_claude_status_table(
    statuses: &[EnvVarStatus],
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if statuses.is_empty() {
        return Ok(());
    }
    
    // Calculate column widths
    let max_key_width = statuses.iter()
        .map(|s| s.key.len())
        .max()
        .unwrap_or(8)
        .max(8) + 2;
    
    let max_value_width = 30;
    
    // Print header
    println!("{:<width_key$} {:<width_value$} {:<6}",
        "Variable", "Value", "Status",
        width_key = max_key_width,
        width_value = max_value_width
    );
    
    println!("{} {} {}",
        "-".repeat(max_key_width),
        "-".repeat(max_value_width),
        "-".repeat(6)
    );
    
    // Print variables
    for status in statuses {
        let value_display = if let Some(ref value) = status.value {
            if is_sensitive_key(&status.key) {
                mask_sensitive_value(value)
            } else if value.len() > max_value_width - 2 {
                format!("{}...", &value[..max_value_width - 5])
            } else {
                value.clone()
            }
        } else {
            "(not set)".to_string()
        };
        
        let status_symbol = if status.value.is_some() { "✓" } else { "✗" };
        
        println!("{:<width_key$} {:<width_value$} {:<6}",
            status.key,
            value_display,
            status_symbol,
            width_key = max_key_width,
            width_value = max_value_width
        );
    }
    
    Ok(())
}

/// Display status in list format
pub fn display_status_list(
    statuses: &[EnvVarStatus],
    expected_variables: &HashMap<String, String>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Environment Variable Status:");
    
    for status in statuses {
        let expected_value = expected_variables.get(&status.key);
        let matches_expected = expected_value.map_or(false, |expected| {
            status.value.as_deref() == Some(expected)
        });
        let status_symbol = if matches_expected { "✓" } else { "✗" };
        
        println!("  {} {}", status_symbol, status.key);
        
        if verbose || !matches_expected {
            if let Some(current) = &status.value {
                let display_current = if is_sensitive_key(&status.key) {
                    mask_sensitive_value(current)
                } else {
                    current.to_string()
                };
                println!("    Current: {}", display_current);
            } else {
                println!("    Current: (not set)");
            }
            
            if let Some(expected) = expected_value {
                let display_expected = if is_sensitive_key(&status.key) {
                    mask_sensitive_value(expected)
                } else {
                    expected.clone()
                };
                println!("    Expected: {}", display_expected);
            }
        }
    }
    
    Ok(())
}

/// Display status in table format
pub fn display_status_table(
    statuses: &[EnvVarStatus],
    expected_variables: &HashMap<String, String>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if statuses.is_empty() {
        return Ok(());
    }
    
    // Calculate column widths
    let max_key_width = statuses.iter()
        .map(|s| s.key.len())
        .max()
        .unwrap_or(8)
        .max(8) + 2;
    
    let max_value_width = 25;
    
    // Print header
    println!("{:<width_key$} {:<width_value$} {:<width_value$} {:<6}",
        "Variable", "Current", "Expected", "Match",
        width_key = max_key_width,
        width_value = max_value_width
    );
    
    println!("{} {} {} {}",
        "-".repeat(max_key_width),
        "-".repeat(max_value_width),
        "-".repeat(max_value_width),
        "-".repeat(6)
    );
    
    // Print variables
    for status in statuses {
        let current_display = if let Some(ref current) = status.value {
            if is_sensitive_key(&status.key) {
                mask_sensitive_value(current)
            } else if current.len() > max_value_width - 2 {
                format!("{}...", &current[..max_value_width - 5])
            } else {
                current.to_string()
            }
        } else {
            "(not set)".to_string()
        };
        
        let expected_display = if let Some(expected) = expected_variables.get(&status.key) {
            if is_sensitive_key(&status.key) {
                mask_sensitive_value(expected)
            } else if expected.len() > max_value_width - 2 {
                format!("{}...", &expected[..max_value_width - 5])
            } else {
                expected.clone()
            }
        } else {
            "(none)".to_string()
        };
        
        let expected_value = expected_variables.get(&status.key);
        let matches_expected = expected_value.map_or(false, |expected| {
            status.value.as_deref() == Some(expected)
        });
        let match_symbol = if matches_expected { "✓" } else { "✗" };
        
        println!("{:<width_key$} {:<width_value$} {:<width_value$} {:<6}",
            status.key,
            current_display,
            expected_display,
            match_symbol,
            width_key = max_key_width,
            width_value = max_value_width
        );
    }
    
    Ok(())
}