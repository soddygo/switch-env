use thiserror::Error;


#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration '{0}' not found")]
    ConfigNotFound(String),
    
    #[error("Configuration file error: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Configuration '{0}' already exists")]
    ConfigExists(String),
    
    #[error("Invalid configuration directory")]
    InvalidConfigDir,
    
    #[error("Invalid configuration name: {0}")]
    InvalidConfigName(String),
    
    #[error("Configuration validation failed: {0}")]
    ValidationError(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Environment variable error: {0}")]
    EnvError(#[from] EnvError),
}

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("Shell detection failed")]
    ShellDetectionFailed,
    
    #[error("Environment variable setting failed: {0}")]
    SetVariableFailed(String),
    
    #[error("Unsupported shell: {0}")]
    UnsupportedShell(String),
    
    #[error("Invalid environment variable name: {0}")]
    InvalidVariableName(String),
    
    #[error("Invalid environment variable value: {0}")]
    InvalidVariableValue(String),
    
    #[error("Command generation failed: {0}")]
    CommandGenerationFailed(String),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("Environment error: {0}")]
    Environment(#[from] EnvError),
    
    #[error("CLI argument error: {0}")]
    CliError(String),
    
    #[error("General error: {0}")]
    General(String),
}

// Type aliases for convenience
pub type ConfigResult<T> = Result<T, ConfigError>;
pub type EnvResult<T> = Result<T, EnvError>;
pub type AppResult<T> = Result<T, AppError>;

impl ConfigError {
    /// Provides user-friendly error messages with suggestions
    pub fn user_message(&self) -> String {
        match self {
            ConfigError::ConfigNotFound(name) => {
                format!("Configuration '{}' not found. Use 'envswitch list' to see available configurations.", name)
            }
            ConfigError::FileError(err) => {
                format!("File operation failed: {}. Check file permissions and disk space.", err)
            }
            ConfigError::JsonError(err) => {
                format!("Configuration file format error: {}. The file may be corrupted.", err)
            }
            ConfigError::ConfigExists(name) => {
                format!("Configuration '{}' already exists. Use 'envswitch edit {}' to modify it.", name, name)
            }
            ConfigError::InvalidConfigDir => {
                "Cannot access configuration directory. Check permissions for ~/.config/envswitch/".to_string()
            }
            ConfigError::InvalidConfigName(name) => {
                format!("Invalid configuration name '{}'. Names must contain only letters, numbers, hyphens, and underscores.", name)
            }
            ConfigError::ValidationError(msg) => {
                format!("Configuration validation failed: {}", msg)
            }
            ConfigError::PermissionDenied(path) => {
                format!("Permission denied accessing '{}'. Check file permissions.", path)
            }
            ConfigError::EnvError(env_err) => {
                format!("Environment variable error: {}", env_err.user_message())
            }
        }
    }
}

impl EnvError {
    /// Provides user-friendly error messages with suggestions
    pub fn user_message(&self) -> String {
        match self {
            EnvError::ShellDetectionFailed => {
                "Could not detect your shell. Please set the SHELL environment variable or use a supported shell (zsh, fish, bash).".to_string()
            }
            EnvError::SetVariableFailed(var) => {
                format!("Failed to set environment variable '{}'. Check if the variable name is valid.", var)
            }
            EnvError::UnsupportedShell(shell) => {
                format!("Shell '{}' is not fully supported. Falling back to generic export commands.", shell)
            }
            EnvError::InvalidVariableName(name) => {
                format!("Invalid environment variable name '{}'. Names must start with a letter and contain only letters, numbers, and underscores.", name)
            }
            EnvError::InvalidVariableValue(value) => {
                format!("Invalid environment variable value: {}", value)
            }
            EnvError::CommandGenerationFailed(msg) => {
                format!("Failed to generate shell commands: {}", msg)
            }
        }
    }
}

/// Validates environment variable names according to POSIX standards
pub fn validate_env_var_name(name: &str) -> Result<(), EnvError> {
    if name.is_empty() {
        return Err(EnvError::InvalidVariableName("Name cannot be empty".to_string()));
    }
    
    // Must start with letter or underscore
    let first_char = name.chars().next().unwrap();
    if !first_char.is_ascii_alphabetic() && first_char != '_' {
        return Err(EnvError::InvalidVariableName(
            format!("Name '{}' must start with a letter or underscore", name)
        ));
    }
    
    // Rest must be alphanumeric or underscore
    for (i, c) in name.chars().enumerate() {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return Err(EnvError::InvalidVariableName(
                format!("Name '{}' contains invalid character '{}' at position {}", name, c, i)
            ));
        }
    }
    
    Ok(())
}

/// Validates configuration alias names
pub fn validate_config_name(name: &str) -> Result<(), ConfigError> {
    if name.is_empty() {
        return Err(ConfigError::InvalidConfigName("Name cannot be empty".to_string()));
    }
    
    if name.len() > 50 {
        return Err(ConfigError::InvalidConfigName("Name too long (max 50 characters)".to_string()));
    }
    
    // Allow letters, numbers, hyphens, and underscores
    for (i, c) in name.chars().enumerate() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return Err(ConfigError::InvalidConfigName(
                format!("Name '{}' contains invalid character '{}' at position {}", name, c, i)
            ));
        }
    }
    
    // Cannot start with hyphen
    if name.starts_with('-') {
        return Err(ConfigError::InvalidConfigName("Name cannot start with hyphen".to_string()));
    }
    
    Ok(())
}
#[cfg
(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_env_var_name_valid() {
        assert!(validate_env_var_name("VALID_NAME").is_ok());
        assert!(validate_env_var_name("_VALID").is_ok());
        assert!(validate_env_var_name("VAR123").is_ok());
        assert!(validate_env_var_name("A").is_ok());
    }

    #[test]
    fn test_validate_env_var_name_invalid() {
        assert!(validate_env_var_name("").is_err());
        assert!(validate_env_var_name("123INVALID").is_err());
        assert!(validate_env_var_name("INVALID-NAME").is_err());
        assert!(validate_env_var_name("INVALID.NAME").is_err());
        assert!(validate_env_var_name("INVALID NAME").is_err());
    }

    #[test]
    fn test_validate_config_name_valid() {
        assert!(validate_config_name("valid-name").is_ok());
        assert!(validate_config_name("valid_name").is_ok());
        assert!(validate_config_name("ValidName123").is_ok());
        assert!(validate_config_name("a").is_ok());
    }

    #[test]
    fn test_validate_config_name_invalid() {
        assert!(validate_config_name("").is_err());
        assert!(validate_config_name("-invalid").is_err());
        assert!(validate_config_name("invalid.name").is_err());
        assert!(validate_config_name("invalid name").is_err());
        assert!(validate_config_name(&"a".repeat(51)).is_err()); // Too long
    }

    #[test]
    fn test_error_user_messages() {
        let config_error = ConfigError::ConfigNotFound("test".to_string());
        let message = config_error.user_message();
        assert!(message.contains("test"));
        assert!(message.contains("envswitch list"));

        let env_error = EnvError::ShellDetectionFailed;
        let message = env_error.user_message();
        assert!(message.contains("shell"));
        assert!(message.contains("SHELL"));
    }
}