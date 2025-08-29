use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Environment variable key-value pairs
pub type EnvVars = HashMap<String, String>;

/// Configuration alias name
pub type ConfigAlias = String;

/// Application constants
pub mod constants {
    /// Default configuration directory name
    pub const CONFIG_DIR_NAME: &str = "envswitch";
    
    /// Configuration file name
    pub const CONFIG_FILE_NAME: &str = "config.json";
    
    /// State file name
    pub const STATE_FILE_NAME: &str = "state.json";
    
    /// Maximum number of configurations
    pub const MAX_CONFIGS: usize = 100;
    
    /// Maximum length for configuration names
    pub const MAX_CONFIG_NAME_LENGTH: usize = 50;
    
    /// Maximum length for environment variable names
    pub const MAX_ENV_VAR_NAME_LENGTH: usize = 100;
    
    /// Maximum length for environment variable values
    pub const MAX_ENV_VAR_VALUE_LENGTH: usize = 1000;
    
    /// Supported environment variable prefixes for Claude Code
    pub const CLAUDE_ENV_VARS: &[&str] = &[
        "ANTHROPIC_BASE_URL",
        "ANTHROPIC_MODEL", 
        "ANTHROPIC_AUTH_TOKEN",
        "ANTHROPIC_SMALL_FAST_MODEL",
    ];
}

/// Application configuration paths
#[derive(Debug, Clone)]
pub struct ConfigPaths {
    pub config_dir: std::path::PathBuf,
    pub config_file: std::path::PathBuf,
    pub state_file: std::path::PathBuf,
}

impl ConfigPaths {
    /// Create new ConfigPaths with default locations
    pub fn new() -> Result<Self, crate::error::ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or(crate::error::ConfigError::InvalidConfigDir)?
            .join(constants::CONFIG_DIR_NAME);
            
        let config_file = config_dir.join(constants::CONFIG_FILE_NAME);
        let state_file = config_dir.join(constants::STATE_FILE_NAME);
        
        Ok(Self {
            config_dir,
            config_file,
            state_file,
        })
    }
    
    /// Ensure configuration directory exists
    pub fn ensure_config_dir(&self) -> Result<(), crate::error::ConfigError> {
        if !self.config_dir.exists() {
            std::fs::create_dir_all(&self.config_dir)?;
            
            // Set restrictive permissions (Unix only)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&self.config_dir)?.permissions();
                perms.set_mode(0o700); // rwx------
                std::fs::set_permissions(&self.config_dir, perms)?;
            }
        }
        Ok(())
    }
}

impl Default for ConfigPaths {
    fn default() -> Self {
        Self::new().expect("Failed to create default config paths")
    }
}

/// Runtime state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    pub active_config: Option<String>,
    pub shell_type: String,
    pub pid: u32,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl RuntimeState {
    pub fn new(active_config: Option<String>, shell_type: String) -> Self {
        Self {
            active_config,
            shell_type,
            pid: std::process::id(),
            started_at: chrono::Utc::now(),
        }
    }
}

/// Validation utilities
pub mod validation {
    use crate::error::{ConfigError, EnvError};
    use super::constants::*;
    
    /// Validate environment variable name and value
    pub fn validate_env_var(name: &str, value: &str) -> Result<(), EnvError> {
        crate::error::validate_env_var_name(name)?;
        
        if value.len() > MAX_ENV_VAR_VALUE_LENGTH {
            return Err(EnvError::InvalidVariableValue(
                format!("Value too long (max {} characters)", MAX_ENV_VAR_VALUE_LENGTH)
            ));
        }
        
        Ok(())
    }
    
    /// Validate configuration alias
    pub fn validate_config_alias(alias: &str) -> Result<(), ConfigError> {
        crate::error::validate_config_name(alias)
    }
    
    /// Check if environment variable is commonly used with Claude Code
    pub fn is_claude_env_var(name: &str) -> bool {
        CLAUDE_ENV_VARS.iter().any(|&var| name == var)
    }
}#[
cfg(test)]
mod tests {
    use super::*;
    use super::validation::*;

    #[test]
    fn test_validate_env_var_valid() {
        assert!(validate_env_var("VALID_NAME", "valid_value").is_ok());
        assert!(validate_env_var("ANTHROPIC_BASE_URL", "https://api.example.com").is_ok());
    }

    #[test]
    fn test_validate_env_var_invalid() {
        assert!(validate_env_var("", "value").is_err());
        assert!(validate_env_var("123INVALID", "value").is_err());
        assert!(validate_env_var("VALID_NAME", &"x".repeat(1001)).is_err()); // Too long value
    }

    #[test]
    fn test_is_claude_env_var() {
        assert!(is_claude_env_var("ANTHROPIC_BASE_URL"));
        assert!(is_claude_env_var("ANTHROPIC_MODEL"));
        assert!(is_claude_env_var("ANTHROPIC_AUTH_TOKEN"));
        assert!(is_claude_env_var("ANTHROPIC_SMALL_FAST_MODEL"));
        assert!(!is_claude_env_var("OTHER_VAR"));
        assert!(!is_claude_env_var("ANTHROPIC_OTHER"));
    }

    #[test]
    fn test_runtime_state_creation() {
        let state = RuntimeState::new(Some("test".to_string()), "zsh".to_string());
        assert_eq!(state.active_config, Some("test".to_string()));
        assert_eq!(state.shell_type, "zsh");
        assert_eq!(state.pid, std::process::id());
    }

    #[test]
    fn test_config_paths_creation() {
        // This test might fail in some environments where config_dir is not available
        if let Ok(paths) = ConfigPaths::new() {
            assert!(paths.config_dir.ends_with("envswitch"));
            assert!(paths.config_file.ends_with("config.json"));
            assert!(paths.state_file.ends_with("state.json"));
        }
    }
}