use std::collections::HashMap;
use std::env;
use crate::error::{EnvError, EnvResult};
use crate::shell::{ShellType, ShellDetector};

/// Environment variable status information
#[derive(Debug, Clone, PartialEq)]
pub struct EnvVarStatus {
    pub key: String,
    pub value: Option<String>,
    pub is_set: bool,
}

impl EnvVarStatus {
    pub fn new(key: String, value: Option<String>) -> Self {
        let is_set = value.is_some();
        Self { key, value, is_set }
    }
    
    pub fn summary(&self) -> String {
        match &self.value {
            Some(val) => {
                if val.len() > 50 {
                    format!("{}={}...", self.key, &val[..47])
                } else {
                    format!("{}={}", self.key, val)
                }
            }
            None => format!("{}=(unset)", self.key),
        }
    }
}

pub trait EnvironmentManager {
    fn set_variables(&self, variables: &HashMap<String, String>) -> EnvResult<()>;
    fn unset_variables(&self, keys: &[String]) -> EnvResult<()>;
    fn get_variable(&self, key: &str) -> Option<String>;
    fn get_current_variables(&self, keys: &[String]) -> HashMap<String, Option<String>>;
    fn get_variable_status(&self, keys: &[String]) -> Vec<EnvVarStatus>;
    fn generate_shell_commands(&self, variables: &HashMap<String, String>) -> EnvResult<String>;
    fn generate_unset_commands(&self, keys: &[String]) -> EnvResult<String>;
    fn get_shell_type(&self) -> &ShellType;
}

pub struct ShellEnvironmentManager {
    shell_type: ShellType,
}

impl ShellEnvironmentManager {
    pub fn new() -> Self {
        Self {
            shell_type: ShellDetector::detect_shell(),
        }
    }
    
    pub fn with_shell_type(shell_type: ShellType) -> Self {
        Self { shell_type }
    }
    
    /// Generate commands to switch to a configuration
    pub fn generate_switch_commands(&self, variables: &HashMap<String, String>) -> EnvResult<String> {
        self.generate_shell_commands(variables)
    }
    
    /// Generate commands to clear specific environment variables
    pub fn generate_clear_commands(&self, keys: &[String]) -> EnvResult<String> {
        self.generate_unset_commands(keys)
    }
    
    /// Check if a variable is currently set
    pub fn is_variable_set(&self, key: &str) -> bool {
        self.get_variable(key).is_some()
    }
    
    /// Get Claude-specific environment variables status
    pub fn get_claude_variables_status(&self) -> Vec<EnvVarStatus> {
        let claude_vars = [
            "ANTHROPIC_BASE_URL",
            "ANTHROPIC_MODEL", 
            "ANTHROPIC_AUTH_TOKEN",
            "ANTHROPIC_SMALL_FAST_MODEL",
        ];
        
        self.get_variable_status(&claude_vars.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }
    
    /// Generate shell integration instructions
    pub fn get_integration_instructions(&self) -> String {
        crate::shell::ShellDetector::get_shell_integration_instructions(&self.shell_type)
    }
}

impl Default for ShellEnvironmentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentManager for ShellEnvironmentManager {
    fn set_variables(&self, _variables: &HashMap<String, String>) -> EnvResult<()> {
        // Note: We don't actually set environment variables in the current process
        // because they need to be set in the parent shell. Instead, we generate
        // shell commands that the user can evaluate.
        Ok(())
    }
    
    fn unset_variables(&self, _keys: &[String]) -> EnvResult<()> {
        // Note: Similar to set_variables, we generate unset commands instead
        // of actually unsetting variables in the current process.
        Ok(())
    }
    
    fn get_variable(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }
    
    fn get_current_variables(&self, keys: &[String]) -> HashMap<String, Option<String>> {
        keys.iter()
            .map(|key| (key.clone(), self.get_variable(key)))
            .collect()
    }
    
    fn get_variable_status(&self, keys: &[String]) -> Vec<EnvVarStatus> {
        keys.iter()
            .map(|key| {
                let value = self.get_variable(key);
                EnvVarStatus::new(key.clone(), value)
            })
            .collect()
    }
    
    fn generate_shell_commands(&self, variables: &HashMap<String, String>) -> EnvResult<String> {
        if variables.is_empty() {
            return Ok(String::new());
        }
        
        // Validate all variable names before generating commands
        for (key, value) in variables {
            crate::types::validation::validate_env_var(key, value)?;
        }
        
        crate::shell::ShellDetector::generate_env_commands(&self.shell_type, variables)
    }
    
    fn generate_unset_commands(&self, keys: &[String]) -> EnvResult<String> {
        if keys.is_empty() {
            return Ok(String::new());
        }
        
        // Validate all variable names
        for key in keys {
            crate::error::validate_env_var_name(key)?;
        }
        
        crate::shell::ShellDetector::generate_unset_commands(&self.shell_type, keys)
    }
    
    fn get_shell_type(&self) -> &ShellType {
        &self.shell_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_variables() -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("ANTHROPIC_BASE_URL".to_string(), "https://api.deepseek.com".to_string());
        vars.insert("ANTHROPIC_MODEL".to_string(), "deepseek-chat".to_string());
        vars.insert("TEST_VAR".to_string(), "test_value".to_string());
        vars
    }

    #[test]
    fn test_env_var_status_creation() {
        let status = EnvVarStatus::new("TEST_VAR".to_string(), Some("test_value".to_string()));
        assert_eq!(status.key, "TEST_VAR");
        assert_eq!(status.value, Some("test_value".to_string()));
        assert!(status.is_set);

        let unset_status = EnvVarStatus::new("UNSET_VAR".to_string(), None);
        assert_eq!(unset_status.key, "UNSET_VAR");
        assert_eq!(unset_status.value, None);
        assert!(!unset_status.is_set);
    }

    #[test]
    fn test_env_var_status_summary() {
        let status = EnvVarStatus::new("TEST_VAR".to_string(), Some("test_value".to_string()));
        assert_eq!(status.summary(), "TEST_VAR=test_value");

        let unset_status = EnvVarStatus::new("UNSET_VAR".to_string(), None);
        assert_eq!(unset_status.summary(), "UNSET_VAR=(unset)");

        // Test long value truncation
        let long_value = "a".repeat(60);
        let long_status = EnvVarStatus::new("LONG_VAR".to_string(), Some(long_value));
        let summary = long_status.summary();
        assert!(summary.starts_with("LONG_VAR="));
        assert!(summary.ends_with("..."));
        // key (8) + "=" (1) + 47 chars + "..." (3) = 59 chars max
        assert!(summary.len() <= 59);
    }

    #[test]
    fn test_shell_environment_manager_creation() {
        let manager = ShellEnvironmentManager::new();
        assert!(matches!(manager.shell_type, ShellType::Zsh | ShellType::Fish | ShellType::Bash | ShellType::Unknown(_)));

        let zsh_manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
        assert!(matches!(zsh_manager.shell_type, ShellType::Zsh));

        let fish_manager = ShellEnvironmentManager::with_shell_type(ShellType::Fish);
        assert!(matches!(fish_manager.shell_type, ShellType::Fish));
    }

    #[test]
    fn test_shell_environment_manager_default() {
        let manager = ShellEnvironmentManager::default();
        assert!(matches!(manager.shell_type, ShellType::Zsh | ShellType::Fish | ShellType::Bash | ShellType::Unknown(_)));
    }

    #[test]
    fn test_get_variable() {
        let manager = ShellEnvironmentManager::new();
        
        // Test getting an environment variable that should exist
        let path = manager.get_variable("PATH");
        assert!(path.is_some());
        
        // Test getting a variable that shouldn't exist
        let nonexistent = manager.get_variable("NONEXISTENT_VAR_12345");
        assert!(nonexistent.is_none());
    }

    #[test]
    fn test_get_current_variables() {
        let manager = ShellEnvironmentManager::new();
        let keys = vec!["PATH".to_string(), "NONEXISTENT_VAR".to_string()];
        
        let variables = manager.get_current_variables(&keys);
        
        assert_eq!(variables.len(), 2);
        assert!(variables.contains_key("PATH"));
        assert!(variables.contains_key("NONEXISTENT_VAR"));
        assert!(variables.get("PATH").unwrap().is_some());
        assert!(variables.get("NONEXISTENT_VAR").unwrap().is_none());
    }

    #[test]
    fn test_get_variable_status() {
        let manager = ShellEnvironmentManager::new();
        let keys = vec!["PATH".to_string(), "NONEXISTENT_VAR".to_string()];
        
        let statuses = manager.get_variable_status(&keys);
        
        assert_eq!(statuses.len(), 2);
        
        let path_status = statuses.iter().find(|s| s.key == "PATH").unwrap();
        assert!(path_status.is_set);
        assert!(path_status.value.is_some());
        
        let nonexistent_status = statuses.iter().find(|s| s.key == "NONEXISTENT_VAR").unwrap();
        assert!(!nonexistent_status.is_set);
        assert!(nonexistent_status.value.is_none());
    }

    #[test]
    fn test_generate_shell_commands_zsh() {
        let manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
        let variables = create_test_variables();
        
        let commands = manager.generate_shell_commands(&variables).unwrap();
        
        assert!(commands.contains("export ANTHROPIC_BASE_URL="));
        assert!(commands.contains("export ANTHROPIC_MODEL="));
        assert!(commands.contains("export TEST_VAR="));
        assert!(commands.contains("https://api.deepseek.com"));
        assert!(commands.contains("deepseek-chat"));
        assert!(commands.contains("test_value"));
    }

    #[test]
    fn test_generate_shell_commands_fish() {
        let manager = ShellEnvironmentManager::with_shell_type(ShellType::Fish);
        let variables = create_test_variables();
        
        let commands = manager.generate_shell_commands(&variables).unwrap();
        
        assert!(commands.contains("set -x ANTHROPIC_BASE_URL"));
        assert!(commands.contains("set -x ANTHROPIC_MODEL"));
        assert!(commands.contains("set -x TEST_VAR"));
        assert!(commands.contains("https://api.deepseek.com"));
        assert!(commands.contains("deepseek-chat"));
        assert!(commands.contains("test_value"));
    }

    #[test]
    fn test_generate_shell_commands_bash() {
        let manager = ShellEnvironmentManager::with_shell_type(ShellType::Bash);
        let variables = create_test_variables();
        
        let commands = manager.generate_shell_commands(&variables).unwrap();
        
        assert!(commands.contains("export ANTHROPIC_BASE_URL="));
        assert!(commands.contains("export ANTHROPIC_MODEL="));
        assert!(commands.contains("export TEST_VAR="));
        assert!(commands.contains("https://api.deepseek.com"));
        assert!(commands.contains("deepseek-chat"));
        assert!(commands.contains("test_value"));
    }

    #[test]
    fn test_generate_shell_commands_empty() {
        let manager = ShellEnvironmentManager::new();
        let empty_vars = HashMap::new();
        
        let commands = manager.generate_shell_commands(&empty_vars).unwrap();
        assert!(commands.is_empty());
    }

    #[test]
    fn test_generate_shell_commands_invalid_var_name() {
        let manager = ShellEnvironmentManager::new();
        let mut invalid_vars = HashMap::new();
        invalid_vars.insert("123INVALID".to_string(), "value".to_string());
        
        let result = manager.generate_shell_commands(&invalid_vars);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_unset_commands_zsh() {
        let manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
        let keys = vec!["VAR1".to_string(), "VAR2".to_string()];
        
        let commands = manager.generate_unset_commands(&keys).unwrap();
        
        assert!(commands.contains("unset VAR1"));
        assert!(commands.contains("unset VAR2"));
    }

    #[test]
    fn test_generate_unset_commands_fish() {
        let manager = ShellEnvironmentManager::with_shell_type(ShellType::Fish);
        let keys = vec!["VAR1".to_string(), "VAR2".to_string()];
        
        let commands = manager.generate_unset_commands(&keys).unwrap();
        
        assert!(commands.contains("set -e VAR1"));
        assert!(commands.contains("set -e VAR2"));
    }

    #[test]
    fn test_generate_unset_commands_empty() {
        let manager = ShellEnvironmentManager::new();
        let empty_keys = vec![];
        
        let commands = manager.generate_unset_commands(&empty_keys).unwrap();
        assert!(commands.is_empty());
    }

    #[test]
    fn test_generate_unset_commands_invalid_var_name() {
        let manager = ShellEnvironmentManager::new();
        let invalid_keys = vec!["123INVALID".to_string()];
        
        let result = manager.generate_unset_commands(&invalid_keys);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_and_unset_variables() {
        let manager = ShellEnvironmentManager::new();
        let variables = create_test_variables();
        let keys = vec!["VAR1".to_string(), "VAR2".to_string()];
        
        // These methods don't actually set/unset variables in the current process
        // They're designed to generate shell commands instead
        assert!(manager.set_variables(&variables).is_ok());
        assert!(manager.unset_variables(&keys).is_ok());
    }

    #[test]
    fn test_get_shell_type() {
        let zsh_manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
        assert!(matches!(zsh_manager.get_shell_type(), ShellType::Zsh));
        
        let fish_manager = ShellEnvironmentManager::with_shell_type(ShellType::Fish);
        assert!(matches!(fish_manager.get_shell_type(), ShellType::Fish));
    }

    #[test]
    fn test_generate_switch_commands() {
        let manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
        let variables = create_test_variables();
        
        let commands = manager.generate_switch_commands(&variables).unwrap();
        assert!(commands.contains("export"));
        assert!(commands.contains("ANTHROPIC_BASE_URL"));
    }

    #[test]
    fn test_generate_clear_commands() {
        let manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
        let keys = vec!["VAR1".to_string(), "VAR2".to_string()];
        
        let commands = manager.generate_clear_commands(&keys).unwrap();
        assert!(commands.contains("unset"));
        assert!(commands.contains("VAR1"));
        assert!(commands.contains("VAR2"));
    }

    #[test]
    fn test_is_variable_set() {
        let manager = ShellEnvironmentManager::new();
        
        // PATH should be set in most environments
        assert!(manager.is_variable_set("PATH"));
        
        // This variable should not be set
        assert!(!manager.is_variable_set("NONEXISTENT_VAR_12345"));
    }

    #[test]
    fn test_get_claude_variables_status() {
        let manager = ShellEnvironmentManager::new();
        
        let statuses = manager.get_claude_variables_status();
        assert_eq!(statuses.len(), 4);
        
        let expected_vars = ["ANTHROPIC_BASE_URL", "ANTHROPIC_MODEL", "ANTHROPIC_AUTH_TOKEN", "ANTHROPIC_SMALL_FAST_MODEL"];
        for expected_var in &expected_vars {
            assert!(statuses.iter().any(|s| s.key == *expected_var));
        }
    }

    #[test]
    fn test_get_integration_instructions() {
        let zsh_manager = ShellEnvironmentManager::with_shell_type(ShellType::Zsh);
        let instructions = zsh_manager.get_integration_instructions();
        assert!(!instructions.is_empty());
        assert!(instructions.contains("zsh") || instructions.contains("bash"));
        
        let fish_manager = ShellEnvironmentManager::with_shell_type(ShellType::Fish);
        let fish_instructions = fish_manager.get_integration_instructions();
        assert!(!fish_instructions.is_empty());
        assert!(fish_instructions.contains("fish"));
    }
}