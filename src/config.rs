use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use chrono::{DateTime, Utc};
use crate::error::{ConfigError, ConfigResult};
use crate::types::ConfigPaths;

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Env,
    Yaml,
}

#[derive(Debug, Clone, Copy)]
pub enum ImportFormat {
    Json,
    Env,
    Yaml,
}

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub format: ExportFormat,
    pub include_metadata: bool,
    pub pretty_print: bool,
    pub configs: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ImportOptions {
    pub format: ImportFormat,
    pub force_overwrite: bool,
    pub merge_existing: bool,
    pub skip_validation: bool,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct ImportResult {
    pub imported: Vec<String>,
    pub conflicts: Vec<String>,
    pub errors: Vec<String>,
}

/// Configuration statistics
#[derive(Debug, Clone)]
pub struct ConfigStats {
    pub total_configs: usize,
    pub total_variables: usize,
    pub claude_configs: usize,
    pub active_config: Option<String>,
    pub backup_count: usize,
    pub last_modified: DateTime<Utc>,
    pub config_file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvConfig {
    pub alias: String,
    pub variables: HashMap<String, String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl EnvConfig {
    /// Create a new environment configuration
    pub fn new(alias: String, variables: HashMap<String, String>, description: Option<String>) -> ConfigResult<Self> {
        // Validate alias
        crate::error::validate_config_name(&alias)?;
        
        // Validate all environment variables
        for (key, value) in &variables {
            crate::types::validation::validate_env_var(key, value)?;
        }
        
        let now = Utc::now();
        Ok(Self {
            alias,
            variables,
            description,
            created_at: now,
            updated_at: now,
        })
    }
    
    /// Update the configuration with new variables
    pub fn update(&mut self, variables: HashMap<String, String>, description: Option<String>) -> ConfigResult<()> {
        // Validate all environment variables
        for (key, value) in &variables {
            crate::types::validation::validate_env_var(key, value)?;
        }
        
        self.variables = variables;
        if description.is_some() {
            self.description = description;
        }
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Get a summary of the configuration
    pub fn summary(&self) -> String {
        let var_count = self.variables.len();
        let desc = self.description.as_deref().unwrap_or("No description");
        format!("{} ({} variables) - {}", self.alias, var_count, desc)
    }
    
    /// Check if this configuration contains Claude-specific variables
    pub fn is_claude_config(&self) -> bool {
        self.variables.keys().any(|key| crate::types::validation::is_claude_env_var(key))
    }
    
    /// Get only Claude-specific variables from this configuration
    pub fn claude_variables(&self) -> HashMap<String, String> {
        self.variables
            .iter()
            .filter(|(key, _)| crate::types::validation::is_claude_env_var(key))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ConfigStore {
    pub configs: HashMap<String, EnvConfig>,
    pub active_config: Option<String>,
    #[serde(default = "Utc::now")]
    pub last_modified: DateTime<Utc>,
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_version() -> String {
    "1.0".to_string()
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self {
            configs: HashMap::new(),
            active_config: None,
            last_modified: Utc::now(),
            version: default_version(),
        }
    }
}

impl ConfigStore {
    /// Create a new empty configuration store
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a configuration to the store
    pub fn add_config(&mut self, config: EnvConfig) -> ConfigResult<()> {
        let alias = config.alias.clone();
        
        if self.configs.contains_key(&alias) {
            return Err(ConfigError::ConfigExists(alias));
        }
        
        self.configs.insert(alias, config);
        self.last_modified = Utc::now();
        Ok(())
    }
    
    /// Update an existing configuration
    pub fn update_config(&mut self, alias: &str, variables: HashMap<String, String>, description: Option<String>) -> ConfigResult<()> {
        let config = self.configs.get_mut(alias)
            .ok_or_else(|| ConfigError::ConfigNotFound(alias.to_string()))?;
        
        config.update(variables, description)?;
        self.last_modified = Utc::now();
        Ok(())
    }
    
    /// Remove a configuration from the store
    pub fn remove_config(&mut self, alias: &str) -> ConfigResult<EnvConfig> {
        let config = self.configs.remove(alias)
            .ok_or_else(|| ConfigError::ConfigNotFound(alias.to_string()))?;
        
        // If this was the active config, clear it
        if self.active_config.as_deref() == Some(alias) {
            self.active_config = None;
        }
        
        self.last_modified = Utc::now();
        Ok(config)
    }
    
    /// Get a configuration by alias
    pub fn get_config(&self, alias: &str) -> Option<&EnvConfig> {
        self.configs.get(alias)
    }
    
    /// Get a mutable reference to a configuration by alias
    pub fn get_config_mut(&mut self, alias: &str) -> Option<&mut EnvConfig> {
        self.configs.get_mut(alias)
    }
    
    /// List all configuration aliases
    pub fn list_aliases(&self) -> Vec<String> {
        let mut aliases: Vec<String> = self.configs.keys().cloned().collect();
        aliases.sort();
        aliases
    }
    
    /// Set the active configuration
    pub fn set_active(&mut self, alias: String) -> ConfigResult<()> {
        if !self.configs.contains_key(&alias) {
            return Err(ConfigError::ConfigNotFound(alias));
        }
        
        self.active_config = Some(alias);
        self.last_modified = Utc::now();
        Ok(())
    }
    
    /// Clear the active configuration
    pub fn clear_active(&mut self) {
        self.active_config = None;
        self.last_modified = Utc::now();
    }
    
    /// Get the active configuration
    pub fn get_active_config(&self) -> Option<&EnvConfig> {
        self.active_config.as_ref().and_then(|alias| self.configs.get(alias))
    }
    
    /// Check if the store is empty
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    
    /// Get the number of configurations
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    
    /// Validate the entire store
    pub fn validate(&self) -> ConfigResult<()> {
        for (alias, config) in &self.configs {
            if alias != &config.alias {
                return Err(ConfigError::ValidationError(
                    format!("Alias mismatch: key '{}' vs config alias '{}'", alias, config.alias)
                ));
            }
            
            crate::error::validate_config_name(alias)?;
            
            for (key, value) in &config.variables {
                crate::types::validation::validate_env_var(key, value)?;
            }
        }
        
        // Validate active config exists
        if let Some(active) = &self.active_config {
            if !self.configs.contains_key(active) {
                return Err(ConfigError::ValidationError(
                    format!("Active config '{}' does not exist", active)
                ));
            }
        }
        
        Ok(())
    }
}

pub trait ConfigManager {
    fn load_configs(&self) -> ConfigResult<ConfigStore>;
    fn save_configs(&self, store: &ConfigStore) -> ConfigResult<()>;
    fn create_config(&self, alias: String, variables: HashMap<String, String>, description: Option<String>) -> ConfigResult<()>;
    fn update_config(&self, alias: String, variables: HashMap<String, String>, description: Option<String>) -> ConfigResult<()>;
    fn delete_config(&self, alias: String) -> ConfigResult<()>;
    fn get_config(&self, alias: &str) -> ConfigResult<Option<EnvConfig>>;
    fn list_configs(&self) -> ConfigResult<Vec<String>>;
    fn set_active_config(&self, alias: String) -> ConfigResult<()>;
    fn get_active_config(&self) -> ConfigResult<Option<String>>;
    fn clear_active_config(&self) -> ConfigResult<()>;
}

/// File-based configuration manager
pub struct FileConfigManager {
    config_paths: ConfigPaths,
}

impl FileConfigManager {
    /// Create a new file-based configuration manager
    pub fn new() -> ConfigResult<Self> {
        let config_paths = ConfigPaths::new()?;
        Ok(Self { config_paths })
    }
    
    /// Create with custom paths (mainly for testing)
    pub fn with_paths(config_paths: ConfigPaths) -> Self {
        Self { config_paths }
    }
    
    /// Get the configuration file path
    pub fn config_file_path(&self) -> &std::path::Path {
        &self.config_paths.config_file
    }
    
    /// Check if configuration file exists
    pub fn config_file_exists(&self) -> bool {
        self.config_paths.config_file.exists()
    }
    
    /// Get configuration file size in bytes
    pub fn config_file_size(&self) -> ConfigResult<u64> {
        let metadata = fs::metadata(&self.config_paths.config_file)
            .map_err(ConfigError::FileError)?;
        Ok(metadata.len())
    }
    
    /// Create a backup of the current configuration file
    pub fn backup_config(&self) -> ConfigResult<std::path::PathBuf> {
        if !self.config_file_exists() {
            return Err(ConfigError::ConfigNotFound("Configuration file not found".to_string()));
        }
        
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S%.3f");
        let backup_name = format!("config_backup_{}.json", timestamp);
        let backup_path = self.config_paths.config_dir.join(backup_name);
        
        fs::copy(&self.config_paths.config_file, &backup_path)
            .map_err(ConfigError::FileError)?;
        
        Ok(backup_path)
    }
    
    /// Restore configuration from a backup file
    pub fn restore_from_backup(&self, backup_path: &std::path::Path) -> ConfigResult<()> {
        if !backup_path.exists() {
            return Err(ConfigError::FileError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Backup file not found"
            )));
        }
        
        // Validate the backup file by trying to load it
        let content = fs::read_to_string(backup_path)
            .map_err(ConfigError::FileError)?;
        let store: ConfigStore = serde_json::from_str(&content)
            .map_err(ConfigError::JsonError)?;
        store.validate()?;
        
        // Note: We don't automatically create a backup of the current config during restore
        // The user should create their own backup if needed before calling restore
        
        // Copy backup to config file
        self.ensure_config_dir()?;
        fs::copy(backup_path, &self.config_paths.config_file)
            .map_err(ConfigError::FileError)?;
        
        // Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.config_paths.config_file)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&self.config_paths.config_file, perms)?;
        }
        
        Ok(())
    }
    
    /// Export configurations to a file
    pub fn export_to_file(&self, export_path: &std::path::Path) -> ConfigResult<()> {
        let store = self.load_store()?;
        let content = serde_json::to_string_pretty(&store)
            .map_err(ConfigError::JsonError)?;
        
        fs::write(export_path, content)
            .map_err(ConfigError::FileError)?;
        
        Ok(())
    }
    
    /// Import configurations from a file
    pub fn import_from_file(&self, import_path: &std::path::Path, merge: bool) -> ConfigResult<Vec<String>> {
        if !import_path.exists() {
            return Err(ConfigError::FileError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Import file not found"
            )));
        }
        
        // Load and validate the import file
        let content = fs::read_to_string(import_path)
            .map_err(ConfigError::FileError)?;
        let import_store: ConfigStore = serde_json::from_str(&content)
            .map_err(ConfigError::JsonError)?;
        import_store.validate()?;
        
        let mut current_store = if merge {
            self.load_store()?
        } else {
            ConfigStore::default()
        };
        
        let mut imported_configs = Vec::new();
        let mut conflicts = Vec::new();
        
        // Process each configuration from import
        for (alias, config) in import_store.configs {
            if current_store.configs.contains_key(&alias) {
                conflicts.push(alias.clone());
                // For now, skip conflicting configs - in a real implementation,
                // we might want to ask the user what to do
                continue;
            }
            
            current_store.configs.insert(alias.clone(), config);
            imported_configs.push(alias);
        }
        
        // Update last modified timestamp
        current_store.last_modified = chrono::Utc::now();
        
        // Save the merged configuration
        self.save_store(&current_store)?;
        
        if !conflicts.is_empty() {
            return Err(ConfigError::ValidationError(
                format!("Conflicts found with existing configurations: {}", conflicts.join(", "))
            ));
        }
        
        Ok(imported_configs)
    }
    
    /// Export configurations to a file with advanced options
    pub fn export_to_file_with_options(&self, export_path: &std::path::Path, options: &ExportOptions) -> ConfigResult<()> {
        let store = self.load_store()?;
        
        // Filter configurations if specific ones are requested
        let configs_to_export = if let Some(config_names) = &options.configs {
            let mut filtered_configs = HashMap::new();
            for name in config_names {
                if let Some(config) = store.configs.get(name) {
                    filtered_configs.insert(name.clone(), config.clone());
                }
            }
            ConfigStore {
                configs: filtered_configs,
                active_config: store.active_config.clone(),
                last_modified: store.last_modified,
                version: store.version.clone(),
            }
        } else {
            store
        };
        
        match options.format {
            ExportFormat::Json => {
                let content = if options.pretty_print {
                    serde_json::to_string_pretty(&configs_to_export)
                } else {
                    serde_json::to_string(&configs_to_export)
                }.map_err(ConfigError::JsonError)?;
                
                fs::write(export_path, content)
                    .map_err(ConfigError::FileError)?;
            }
            ExportFormat::Env => {
                let mut content = String::new();
                
                if options.include_metadata {
                    content.push_str(&format!("# Exported from envswitch on {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
                    content.push_str(&format!("# Total configurations: {}\n", configs_to_export.configs.len()));
                    if let Some(active) = &configs_to_export.active_config {
                        content.push_str(&format!("# Active configuration: {}\n", active));
                    }
                    content.push_str("\n");
                }
                
                for (alias, config) in &configs_to_export.configs {
                    if options.include_metadata {
                        content.push_str(&format!("# Configuration: {}\n", alias));
                        if let Some(desc) = &config.description {
                            content.push_str(&format!("# Description: {}\n", desc));
                        }
                        content.push_str(&format!("# Created: {}\n", config.created_at.format("%Y-%m-%d %H:%M:%S UTC")));
                        content.push_str(&format!("# Updated: {}\n", config.updated_at.format("%Y-%m-%d %H:%M:%S UTC")));
                    }
                    
                    for (key, value) in &config.variables {
                        content.push_str(&format!("{}={}\n", key, value));
                    }
                    content.push_str("\n");
                }
                
                fs::write(export_path, content)
                    .map_err(ConfigError::FileError)?;
            }
            ExportFormat::Yaml => {
                // For now, convert to JSON and then to YAML-like format
                // In a real implementation, you'd use a YAML library
                let mut content = String::new();
                
                if options.include_metadata {
                    content.push_str(&format!("# Exported from envswitch on {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
                    content.push_str("\n");
                }
                
                content.push_str("configurations:\n");
                for (alias, config) in &configs_to_export.configs {
                    content.push_str(&format!("  {}:\n", alias));
                    if let Some(desc) = &config.description {
                        content.push_str(&format!("    description: \"{}\"\n", desc));
                    }
                    if options.include_metadata {
                        content.push_str(&format!("    created_at: \"{}\"\n", config.created_at.to_rfc3339()));
                        content.push_str(&format!("    updated_at: \"{}\"\n", config.updated_at.to_rfc3339()));
                    }
                    content.push_str("    variables:\n");
                    for (key, value) in &config.variables {
                        content.push_str(&format!("      {}: \"{}\"\n", key, value));
                    }
                    content.push_str("\n");
                }
                
                if let Some(active) = &configs_to_export.active_config {
                    content.push_str(&format!("active_config: \"{}\"\n", active));
                }
                
                fs::write(export_path, content)
                    .map_err(ConfigError::FileError)?;
            }
        }
        
        Ok(())
    }
    
    /// Import configurations from a file with advanced options
    pub fn import_from_file_with_options(&self, import_path: &std::path::Path, options: &ImportOptions) -> ConfigResult<ImportResult> {
        if !import_path.exists() {
            return Err(ConfigError::FileError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Import file not found"
            )));
        }
        
        // Parse the import file based on format
        let import_store = match options.format {
            ImportFormat::Json => {
                let content = fs::read_to_string(import_path)
                    .map_err(ConfigError::FileError)?;
                serde_json::from_str::<ConfigStore>(&content)
                    .map_err(ConfigError::JsonError)?
            }
            ImportFormat::Env => {
                self.parse_env_file(import_path)?
            }
            ImportFormat::Yaml => {
                // For now, return an error - YAML parsing would need a YAML library
                return Err(ConfigError::ValidationError("YAML import not yet implemented".to_string()));
            }
        };
        
        // Validate import data unless skipped
        if !options.skip_validation {
            if let Err(e) = import_store.validate() {
                return Err(e);
            }
        }
        
        let mut result = ImportResult {
            imported: Vec::new(),
            conflicts: Vec::new(),
            errors: Vec::new(),
        };
        
        if options.dry_run {
            // Just analyze what would happen
            let current_store = self.load_store()?;
            
            for (alias, _config) in &import_store.configs {
                if current_store.configs.contains_key(alias) {
                    result.conflicts.push(alias.clone());
                } else {
                    result.imported.push(alias.clone());
                }
            }
            
            return Ok(result);
        }
        
        // Load current configurations
        let mut current_store = if options.merge_existing {
            self.load_store()?
        } else {
            ConfigStore::default()
        };
        
        // Process each configuration from import
        for (alias, config) in import_store.configs {
            let config_exists = current_store.configs.contains_key(&alias);
            
            if config_exists && !options.force_overwrite && !options.merge_existing {
                result.conflicts.push(alias);
                continue;
            }
            
            if config_exists && options.merge_existing {
                // Merge variables with existing configuration
                if let Some(existing_config) = current_store.configs.get_mut(&alias) {
                    for (key, value) in config.variables {
                        existing_config.variables.insert(key, value);
                    }
                    existing_config.updated_at = chrono::Utc::now();
                    if config.description.is_some() {
                        existing_config.description = config.description;
                    }
                }
            } else {
                // Add or replace configuration
                current_store.configs.insert(alias.clone(), config);
            }
            
            result.imported.push(alias);
        }
        
        // Update last modified timestamp
        current_store.last_modified = chrono::Utc::now();
        
        // Save the updated configuration
        self.save_store(&current_store)?;
        
        Ok(result)
    }
    
    /// Parse an .env format file into a ConfigStore
    fn parse_env_file(&self, file_path: &std::path::Path) -> ConfigResult<ConfigStore> {
        let content = fs::read_to_string(file_path)
            .map_err(ConfigError::FileError)?;
        
        let mut configs = HashMap::new();
        let mut current_config_name = "imported".to_string();
        let mut current_description = None;
        let mut current_variables = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines
            if line.is_empty() {
                continue;
            }
            
            // Handle comments
            if line.starts_with('#') {
                // Check for special comments that define configuration metadata
                if line.starts_with("# Configuration:") {
                    // Save previous configuration if it has variables
                    if !current_variables.is_empty() {
                        let config = EnvConfig {
                            alias: current_config_name.clone(),
                            variables: current_variables.clone(),
                            description: current_description.clone(),
                            created_at: chrono::Utc::now(),
                            updated_at: chrono::Utc::now(),
                        };
                        configs.insert(current_config_name.clone(), config);
                    }
                    
                    // Start new configuration
                    current_config_name = line.replace("# Configuration:", "").trim().to_string();
                    current_description = None;
                    current_variables.clear();
                } else if line.starts_with("# Description:") {
                    current_description = Some(line.replace("# Description:", "").trim().to_string());
                }
                continue;
            }
            
            // Parse KEY=VALUE format
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                
                // Remove quotes if present
                let value = if (value.starts_with('"') && value.ends_with('"')) ||
                              (value.starts_with('\'') && value.ends_with('\'')) {
                    value[1..value.len()-1].to_string()
                } else {
                    value
                };
                
                current_variables.insert(key, value);
            }
        }
        
        // Save the last configuration if it has variables
        if !current_variables.is_empty() {
            let config = EnvConfig {
                alias: current_config_name.clone(),
                variables: current_variables,
                description: current_description,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            configs.insert(current_config_name, config);
        }
        
        Ok(ConfigStore {
            configs,
            active_config: None,
            last_modified: chrono::Utc::now(),
            version: default_version(),
        })
    }
    
    /// List all backup files in the configuration directory
    pub fn list_backups(&self) -> ConfigResult<Vec<std::path::PathBuf>> {
        let mut backups = Vec::new();
        
        if !self.config_paths.config_dir.exists() {
            return Ok(backups);
        }
        
        let entries = fs::read_dir(&self.config_paths.config_dir)
            .map_err(ConfigError::FileError)?;
        
        for entry in entries {
            let entry = entry.map_err(ConfigError::FileError)?;
            let path = entry.path();
            
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with("config_backup_") && filename.ends_with(".json") {
                    backups.push(path);
                }
            }
        }
        
        // Sort by modification time (newest first)
        backups.sort_by(|a, b| {
            let a_time = fs::metadata(a).and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
            let b_time = fs::metadata(b).and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
            b_time.cmp(&a_time)
        });
        
        Ok(backups)
    }
    
    /// Clean up old backup files, keeping only the most recent N backups
    pub fn cleanup_backups(&self, keep_count: usize) -> ConfigResult<usize> {
        let backups = self.list_backups()?;
        
        if backups.len() <= keep_count {
            return Ok(0);
        }
        
        let to_remove = &backups[keep_count..];
        let mut removed_count = 0;
        
        for backup_path in to_remove {
            if let Err(e) = fs::remove_file(backup_path) {
                eprintln!("Warning: Failed to remove backup file {:?}: {}", backup_path, e);
            } else {
                removed_count += 1;
            }
        }
        
        Ok(removed_count)
    }
    
    /// Get configuration statistics
    pub fn get_stats(&self) -> ConfigResult<ConfigStats> {
        let store = self.load_store()?;
        let backups = self.list_backups()?;
        
        let mut total_variables = 0;
        let mut claude_configs = 0;
        
        for config in store.configs.values() {
            total_variables += config.variables.len();
            if config.is_claude_config() {
                claude_configs += 1;
            }
        }
        
        Ok(ConfigStats {
            total_configs: store.configs.len(),
            total_variables,
            claude_configs,
            active_config: store.active_config.clone(),
            backup_count: backups.len(),
            last_modified: store.last_modified,
            config_file_size: if self.config_file_exists() { 
                Some(self.config_file_size()?) 
            } else { 
                None 
            },
        })
    }
    
    /// Ensure configuration directory exists
    fn ensure_config_dir(&self) -> ConfigResult<()> {
        self.config_paths.ensure_config_dir()
    }
    
    /// Load configuration store from file, creating default if not exists
    fn load_store(&self) -> ConfigResult<ConfigStore> {
        if !self.config_paths.config_file.exists() {
            return Ok(ConfigStore::default());
        }
        
        let content = fs::read_to_string(&self.config_paths.config_file)
            .map_err(ConfigError::FileError)?;
        
        let store: ConfigStore = serde_json::from_str(&content)
            .map_err(ConfigError::JsonError)?;
        
        // Validate the loaded store
        store.validate()?;
        
        Ok(store)
    }
    
    /// Save configuration store to file
    fn save_store(&self, store: &ConfigStore) -> ConfigResult<()> {
        self.ensure_config_dir()?;
        
        // Validate before saving
        store.validate()?;
        
        let content = serde_json::to_string_pretty(store)
            .map_err(ConfigError::JsonError)?;
        
        fs::write(&self.config_paths.config_file, content)
            .map_err(ConfigError::FileError)?;
        
        // Set restrictive permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.config_paths.config_file)?.permissions();
            perms.set_mode(0o600); // rw-------
            fs::set_permissions(&self.config_paths.config_file, perms)?;
        }
        
        Ok(())
    }
}

impl Default for FileConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create FileConfigManager")
    }
}

impl ConfigManager for FileConfigManager {
    fn load_configs(&self) -> ConfigResult<ConfigStore> {
        self.load_store()
    }
    
    fn save_configs(&self, store: &ConfigStore) -> ConfigResult<()> {
        self.save_store(store)
    }
    
    fn create_config(&self, alias: String, variables: HashMap<String, String>, description: Option<String>) -> ConfigResult<()> {
        let mut store = self.load_store()?;
        let config = EnvConfig::new(alias, variables, description)?;
        store.add_config(config)?;
        self.save_store(&store)
    }
    
    fn update_config(&self, alias: String, variables: HashMap<String, String>, description: Option<String>) -> ConfigResult<()> {
        let mut store = self.load_store()?;
        store.update_config(&alias, variables, description)?;
        self.save_store(&store)
    }
    
    fn delete_config(&self, alias: String) -> ConfigResult<()> {
        let mut store = self.load_store()?;
        store.remove_config(&alias)?;
        self.save_store(&store)
    }
    
    fn get_config(&self, alias: &str) -> ConfigResult<Option<EnvConfig>> {
        let store = self.load_store()?;
        Ok(store.get_config(alias).cloned())
    }
    
    fn list_configs(&self) -> ConfigResult<Vec<String>> {
        let store = self.load_store()?;
        Ok(store.list_aliases())
    }
    
    fn set_active_config(&self, alias: String) -> ConfigResult<()> {
        let mut store = self.load_store()?;
        store.set_active(alias)?;
        self.save_store(&store)
    }
    
    fn get_active_config(&self) -> ConfigResult<Option<String>> {
        let store = self.load_store()?;
        Ok(store.active_config)
    }
    
    fn clear_active_config(&self) -> ConfigResult<()> {
        let mut store = self.load_store()?;
        store.clear_active();
        self.save_store(&store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;


    fn create_test_variables() -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("ANTHROPIC_BASE_URL".to_string(), "https://api.deepseek.com".to_string());
        vars.insert("ANTHROPIC_MODEL".to_string(), "deepseek-chat".to_string());
        vars
    }

    fn create_test_config_paths() -> ConfigPaths {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_path_buf();
        let config_file = config_dir.join("config.json");
        let state_file = config_dir.join("state.json");
        
        // Keep temp_dir alive by leaking it (for test purposes only)
        std::mem::forget(temp_dir);
        
        ConfigPaths {
            config_dir,
            config_file,
            state_file,
        }
    }

    #[test]
    fn test_env_config_creation() {
        let variables = create_test_variables();
        let config = EnvConfig::new(
            "test".to_string(),
            variables.clone(),
            Some("Test configuration".to_string())
        ).unwrap();

        assert_eq!(config.alias, "test");
        assert_eq!(config.variables, variables);
        assert_eq!(config.description, Some("Test configuration".to_string()));
        assert!(config.created_at <= Utc::now());
        assert!(config.updated_at <= Utc::now());
    }

    #[test]
    fn test_env_config_invalid_alias() {
        let variables = create_test_variables();
        let result = EnvConfig::new(
            "invalid-name-with-spaces and-symbols!".to_string(),
            variables,
            None
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_env_config_invalid_variables() {
        let mut variables = HashMap::new();
        variables.insert("123INVALID".to_string(), "value".to_string());
        
        let result = EnvConfig::new("test".to_string(), variables, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_env_config_update() {
        let variables = create_test_variables();
        let mut config = EnvConfig::new("test".to_string(), variables, None).unwrap();
        
        let original_updated_at = config.updated_at;
        
        // Wait a bit to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let mut new_variables = HashMap::new();
        new_variables.insert("NEW_VAR".to_string(), "new_value".to_string());
        
        config.update(new_variables.clone(), Some("Updated description".to_string())).unwrap();
        
        assert_eq!(config.variables, new_variables);
        assert_eq!(config.description, Some("Updated description".to_string()));
        assert!(config.updated_at > original_updated_at);
    }

    #[test]
    fn test_env_config_summary() {
        let variables = create_test_variables();
        let config = EnvConfig::new(
            "test".to_string(),
            variables,
            Some("Test config".to_string())
        ).unwrap();

        let summary = config.summary();
        assert!(summary.contains("test"));
        assert!(summary.contains("2 variables"));
        assert!(summary.contains("Test config"));
    }

    #[test]
    fn test_env_config_claude_detection() {
        let variables = create_test_variables();
        let config = EnvConfig::new("test".to_string(), variables, None).unwrap();
        
        assert!(config.is_claude_config());
        
        let claude_vars = config.claude_variables();
        assert_eq!(claude_vars.len(), 2);
        assert!(claude_vars.contains_key("ANTHROPIC_BASE_URL"));
        assert!(claude_vars.contains_key("ANTHROPIC_MODEL"));
    }

    #[test]
    fn test_config_store_operations() {
        let mut store = ConfigStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);

        // Add configuration
        let variables = create_test_variables();
        let config = EnvConfig::new("test".to_string(), variables, None).unwrap();
        store.add_config(config).unwrap();

        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);
        assert!(store.get_config("test").is_some());

        // Set active
        store.set_active("test".to_string()).unwrap();
        assert_eq!(store.active_config, Some("test".to_string()));
        assert!(store.get_active_config().is_some());

        // List aliases
        let aliases = store.list_aliases();
        assert_eq!(aliases, vec!["test"]);

        // Remove configuration
        let removed = store.remove_config("test").unwrap();
        assert_eq!(removed.alias, "test");
        assert!(store.is_empty());
        assert!(store.active_config.is_none());
    }

    #[test]
    fn test_config_store_duplicate_alias() {
        let mut store = ConfigStore::new();
        let variables = create_test_variables();
        
        let config1 = EnvConfig::new("test".to_string(), variables.clone(), None).unwrap();
        let config2 = EnvConfig::new("test".to_string(), variables, None).unwrap();
        
        store.add_config(config1).unwrap();
        let result = store.add_config(config2);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::ConfigExists(_)));
    }

    #[test]
    fn test_config_store_nonexistent_config() {
        let store = ConfigStore::new();
        
        assert!(store.get_config("nonexistent").is_none());
        
        let mut store = store;
        let result = store.set_active("nonexistent".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::ConfigNotFound(_)));
    }

    #[test]
    fn test_config_store_validation() {
        let mut store = ConfigStore::new();
        
        // Valid store should pass validation
        assert!(store.validate().is_ok());
        
        // Add a valid config
        let variables = create_test_variables();
        let config = EnvConfig::new("test".to_string(), variables, None).unwrap();
        store.add_config(config).unwrap();
        store.set_active("test".to_string()).unwrap();
        
        assert!(store.validate().is_ok());
    }

    #[test]
    fn test_config_store_serialization() {
        let mut store = ConfigStore::new();
        let variables = create_test_variables();
        let config = EnvConfig::new("test".to_string(), variables, Some("Test".to_string())).unwrap();
        store.add_config(config).unwrap();
        store.set_active("test".to_string()).unwrap();

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&store).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("ANTHROPIC_BASE_URL"));

        // Deserialize from JSON
        let deserialized: ConfigStore = serde_json::from_str(&json).unwrap();
        assert_eq!(store, deserialized);
    }

    #[test]
    fn test_file_config_manager_basic_operations() {
        let config_paths = create_test_config_paths();
        let manager = FileConfigManager::with_paths(config_paths);

        // Initially no configs
        let configs = manager.list_configs().unwrap();
        assert!(configs.is_empty());

        // Create a config
        let variables = create_test_variables();
        manager.create_config(
            "test".to_string(),
            variables.clone(),
            Some("Test config".to_string())
        ).unwrap();

        // List configs
        let configs = manager.list_configs().unwrap();
        assert_eq!(configs, vec!["test"]);

        // Get config
        let config = manager.get_config("test").unwrap().unwrap();
        assert_eq!(config.alias, "test");
        assert_eq!(config.variables, variables);

        // Set active
        manager.set_active_config("test".to_string()).unwrap();
        let active = manager.get_active_config().unwrap();
        assert_eq!(active, Some("test".to_string()));

        // Update config
        let mut new_variables = HashMap::new();
        new_variables.insert("NEW_VAR".to_string(), "new_value".to_string());
        manager.update_config(
            "test".to_string(),
            new_variables.clone(),
            Some("Updated".to_string())
        ).unwrap();

        let updated_config = manager.get_config("test").unwrap().unwrap();
        assert_eq!(updated_config.variables, new_variables);
        assert_eq!(updated_config.description, Some("Updated".to_string()));

        // Delete config
        manager.delete_config("test".to_string()).unwrap();
        let configs = manager.list_configs().unwrap();
        assert!(configs.is_empty());
    }

    #[test]
    fn test_file_config_manager_persistence() {
        let config_paths = create_test_config_paths();
        
        // Create config with first manager instance
        {
            let manager = FileConfigManager::with_paths(config_paths.clone());
            let variables = create_test_variables();
            manager.create_config("test".to_string(), variables, None).unwrap();
            manager.set_active_config("test".to_string()).unwrap();
        }
        
        // Load with second manager instance
        {
            let manager = FileConfigManager::with_paths(config_paths);
            let configs = manager.list_configs().unwrap();
            assert_eq!(configs, vec!["test"]);
            
            let active = manager.get_active_config().unwrap();
            assert_eq!(active, Some("test".to_string()));
        }
    }

    #[test]
    fn test_file_config_manager_error_handling() {
        let config_paths = create_test_config_paths();
        let manager = FileConfigManager::with_paths(config_paths);

        // Try to get nonexistent config
        let result = manager.get_config("nonexistent").unwrap();
        assert!(result.is_none());

        // Try to delete nonexistent config
        let result = manager.delete_config("nonexistent".to_string());
        assert!(result.is_err());

        // Try to set nonexistent config as active
        let result = manager.set_active_config("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_file_config_manager_backup_restore() {
        let config_paths = create_test_config_paths();
        let manager = FileConfigManager::with_paths(config_paths);

        // Create a config
        let variables = create_test_variables();
        manager.create_config("test".to_string(), variables.clone(), Some("Original".to_string())).unwrap();

        // Create backup
        let backup_path = manager.backup_config().unwrap();
        assert!(backup_path.exists());

        // Modify the config
        let mut new_variables = HashMap::new();
        new_variables.insert("MODIFIED_VAR".to_string(), "modified_value".to_string());
        manager.update_config("test".to_string(), new_variables, Some("Modified".to_string())).unwrap();

        // Verify modification
        let modified_config = manager.get_config("test").unwrap().unwrap();
        assert_eq!(modified_config.description, Some("Modified".to_string()));

        // Restore from backup
        manager.restore_from_backup(&backup_path).unwrap();

        // Verify restoration - the restored config should have the original data
        let restored_config = manager.get_config("test").unwrap().unwrap();
        assert_eq!(restored_config.description, Some("Original".to_string()));
        assert_eq!(restored_config.variables, variables);
    }

    #[test]
    fn test_file_config_manager_export_import() {
        let config_paths1 = create_test_config_paths();
        let config_paths2 = create_test_config_paths();
        let export_path = config_paths1.config_dir.join("export.json");
        let manager1 = FileConfigManager::with_paths(config_paths1);
        let manager2 = FileConfigManager::with_paths(config_paths2);

        // Create configs in first manager
        let variables1 = create_test_variables();
        let mut variables2 = HashMap::new();
        variables2.insert("OTHER_VAR".to_string(), "other_value".to_string());

        manager1.create_config("config1".to_string(), variables1.clone(), Some("Config 1".to_string())).unwrap();
        manager1.create_config("config2".to_string(), variables2.clone(), Some("Config 2".to_string())).unwrap();
        manager1.set_active_config("config1".to_string()).unwrap();

        // Export from first manager
        manager1.export_to_file(&export_path).unwrap();
        assert!(export_path.exists());

        // Import to second manager
        let imported = manager2.import_from_file(&export_path, false).unwrap();
        assert_eq!(imported.len(), 2);
        assert!(imported.contains(&"config1".to_string()));
        assert!(imported.contains(&"config2".to_string()));

        // Verify imported configs
        let config1 = manager2.get_config("config1").unwrap().unwrap();
        assert_eq!(config1.variables, variables1);
        assert_eq!(config1.description, Some("Config 1".to_string()));

        let config2 = manager2.get_config("config2").unwrap().unwrap();
        assert_eq!(config2.variables, variables2);
        assert_eq!(config2.description, Some("Config 2".to_string()));
    }

    #[test]
    fn test_file_config_manager_backup_management() {
        let config_paths = create_test_config_paths();
        let manager = FileConfigManager::with_paths(config_paths);

        // Create a config
        let variables = create_test_variables();
        manager.create_config("test".to_string(), variables, None).unwrap();

        // Initially no backups
        let backups = manager.list_backups().unwrap();
        assert_eq!(backups.len(), 0);

        // Create multiple backups with small delays to ensure unique timestamps
        let _backup1 = manager.backup_config().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let _backup2 = manager.backup_config().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let _backup3 = manager.backup_config().unwrap();

        // List backups
        let backups = manager.list_backups().unwrap();
        assert_eq!(backups.len(), 3);

        // Cleanup old backups (keep only 1)
        let removed = manager.cleanup_backups(1).unwrap();
        assert_eq!(removed, 2);

        let remaining_backups = manager.list_backups().unwrap();
        assert_eq!(remaining_backups.len(), 1);
    }

    #[test]
    fn test_file_config_manager_stats() {
        let config_paths = create_test_config_paths();
        let manager = FileConfigManager::with_paths(config_paths);

        // Initially no stats
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_configs, 0);
        assert_eq!(stats.total_variables, 0);
        assert_eq!(stats.claude_configs, 0);
        assert!(stats.active_config.is_none());
        assert!(stats.config_file_size.is_none());

        // Create configs
        let claude_vars = create_test_variables(); // Contains Claude variables
        let mut other_vars = HashMap::new();
        other_vars.insert("OTHER_VAR".to_string(), "value".to_string());

        manager.create_config("claude".to_string(), claude_vars, Some("Claude config".to_string())).unwrap();
        manager.create_config("other".to_string(), other_vars, Some("Other config".to_string())).unwrap();
        manager.set_active_config("claude".to_string()).unwrap();

        // Create a backup
        manager.backup_config().unwrap();

        // Check stats
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_configs, 2);
        assert_eq!(stats.total_variables, 3); // 2 Claude vars + 1 other var
        assert_eq!(stats.claude_configs, 1);
        assert_eq!(stats.active_config, Some("claude".to_string()));
        assert_eq!(stats.backup_count, 1);
        assert!(stats.config_file_size.is_some());
        assert!(stats.config_file_size.unwrap() > 0);
    }

    #[test]
    fn test_file_config_manager_file_operations() {
        let config_paths = create_test_config_paths();
        let manager = FileConfigManager::with_paths(config_paths);

        // Initially no config file
        assert!(!manager.config_file_exists());

        // Create a config
        let variables = create_test_variables();
        manager.create_config("test".to_string(), variables, None).unwrap();

        // Now config file should exist
        assert!(manager.config_file_exists());
        assert!(manager.config_file_size().unwrap() > 0);

        // Check config file path
        let path = manager.config_file_path();
        assert!(path.ends_with("config.json"));
    }
}