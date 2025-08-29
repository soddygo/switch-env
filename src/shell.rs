use std::env;
use std::collections::HashMap;
use crate::error::{EnvError, EnvResult};

#[derive(Debug, Clone, PartialEq)]
pub enum ShellType {
    Zsh,
    Fish,
    Bash,
    Unknown(String),
}

#[derive(Debug, Clone)]
pub enum ShellCommandFormat {
    Export, // export KEY=VALUE
    Set,    // set -x KEY VALUE
}

pub struct ShellDetector;

impl ShellDetector {
    /// Detect the current shell type using multiple methods
    pub fn detect_shell() -> ShellType {
        // Method 1: Check $SHELL environment variable
        if let Ok(shell_path) = env::var("SHELL") {
            if let Some(shell_type) = Self::parse_shell_from_path(&shell_path) {
                return shell_type;
            }
        }
        
        // Method 2: Check $0 (current process name)
        if let Ok(args) = env::var("_") {
            if let Some(shell_type) = Self::parse_shell_from_path(&args) {
                return shell_type;
            }
        }
        
        // Method 3: Check parent process (Unix only)
        #[cfg(unix)]
        {
            if let Some(shell_type) = Self::detect_parent_shell() {
                return shell_type;
            }
        }
        
        // Method 4: Check common shell-specific environment variables
        if env::var("ZSH_VERSION").is_ok() {
            return ShellType::Zsh;
        }
        if env::var("FISH_VERSION").is_ok() {
            return ShellType::Fish;
        }
        if env::var("BASH_VERSION").is_ok() {
            return ShellType::Bash;
        }
        
        // Default to unknown
        ShellType::Unknown("unknown".to_string())
    }
    
    /// Parse shell type from a path string
    fn parse_shell_from_path(path: &str) -> Option<ShellType> {
        let path_lower = path.to_lowercase();
        
        if path_lower.contains("zsh") {
            Some(ShellType::Zsh)
        } else if path_lower.contains("fish") {
            Some(ShellType::Fish)
        } else if path_lower.contains("bash") {
            Some(ShellType::Bash)
        } else {
            None
        }
    }
    
    /// Detect shell from parent process (Unix only)
    #[cfg(unix)]
    fn detect_parent_shell() -> Option<ShellType> {
        use std::process::Command;
        
        // Get parent process ID
        let ppid = unsafe { libc::getppid() };
        
        // Try to get process name using ps
        if let Ok(output) = Command::new("ps")
            .args(&["-p", &ppid.to_string(), "-o", "comm="])
            .output()
        {
            if let Ok(comm) = String::from_utf8(output.stdout) {
                let comm = comm.trim();
                return Self::parse_shell_from_path(comm);
            }
        }
        
        None
    }
    
    /// Get the appropriate command format for a shell type
    pub fn get_shell_command_format(shell_type: &ShellType) -> ShellCommandFormat {
        match shell_type {
            ShellType::Zsh | ShellType::Bash => ShellCommandFormat::Export,
            ShellType::Fish => ShellCommandFormat::Set,
            ShellType::Unknown(_) => ShellCommandFormat::Export,
        }
    }
    
    /// Generate shell commands to set environment variables
    pub fn generate_env_commands(
        shell_type: &ShellType,
        variables: &HashMap<String, String>,
    ) -> EnvResult<String> {
        if variables.is_empty() {
            return Ok(String::new());
        }
        
        let format = Self::get_shell_command_format(shell_type);
        let mut commands = Vec::new();
        
        for (key, value) in variables {
            // Validate environment variable name
            crate::error::validate_env_var_name(key)?;
            
            let command = match format {
                ShellCommandFormat::Export => {
                    // For bash/zsh: export KEY='value'
                    format!("export {}='{}'", key, Self::escape_value_for_export(value))
                }
                ShellCommandFormat::Set => {
                    // For fish: set -x KEY 'value'
                    format!("set -x {} '{}'", key, Self::escape_value_for_fish(value))
                }
            };
            commands.push(command);
        }
        
        Ok(commands.join("\n"))
    }
    
    /// Generate shell commands to unset environment variables
    pub fn generate_unset_commands(
        shell_type: &ShellType,
        variable_names: &[String],
    ) -> EnvResult<String> {
        if variable_names.is_empty() {
            return Ok(String::new());
        }
        
        let mut commands = Vec::new();
        
        for name in variable_names {
            crate::error::validate_env_var_name(name)?;
            
            let command = match shell_type {
                ShellType::Fish => format!("set -e {}", name),
                _ => format!("unset {}", name),
            };
            commands.push(command);
        }
        
        Ok(commands.join("\n"))
    }
    
    /// Escape value for export command (bash/zsh)
    fn escape_value_for_export(value: &str) -> String {
        // Escape single quotes by ending the quoted string, adding an escaped quote, and starting a new quoted string
        value.replace('\'', "'\"'\"'")
    }
    
    /// Escape value for fish set command
    fn escape_value_for_fish(value: &str) -> String {
        // Fish uses the same escaping as bash for single quotes
        value.replace('\'', "'\"'\"'")
    }
    
    /// Get shell-specific configuration instructions
    pub fn get_shell_integration_instructions(shell_type: &ShellType) -> String {
        match shell_type {
            ShellType::Zsh => {
                r#"# Add to your ~/.zshrc:
alias envswitch-use='eval "$(envswitch use $1)"'

# Usage:
# envswitch-use deepseek
# envswitch-use kimi"#.to_string()
            }
            ShellType::Fish => {
                r#"# Add to your ~/.config/fish/config.fish:
function envswitch-use
    eval (envswitch use $argv[1])
end

# Usage:
# envswitch-use deepseek
# envswitch-use kimi"#.to_string()
            }
            ShellType::Bash => {
                r#"# Add to your ~/.bashrc:
alias envswitch-use='eval "$(envswitch use $1)"'

# Usage:
# envswitch-use deepseek
# envswitch-use kimi"#.to_string()
            }
            ShellType::Unknown(name) => {
                format!(r#"# Shell '{}' is not fully supported.
# Try using the generic approach:
eval "$(envswitch use <config-name>)"

# Or add to your shell's configuration file:
alias envswitch-use='eval "$(envswitch use $1)"'"#, name)
            }
        }
    }
}

impl std::fmt::Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellType::Zsh => write!(f, "zsh"),
            ShellType::Fish => write!(f, "fish"),
            ShellType::Bash => write!(f, "bash"),
            ShellType::Unknown(name) => write!(f, "unknown({})", name),
        }
    }
}#[cfg(test
)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_shell_from_path() {
        assert_eq!(
            ShellDetector::parse_shell_from_path("/bin/zsh"),
            Some(ShellType::Zsh)
        );
        assert_eq!(
            ShellDetector::parse_shell_from_path("/usr/local/bin/fish"),
            Some(ShellType::Fish)
        );
        assert_eq!(
            ShellDetector::parse_shell_from_path("/bin/bash"),
            Some(ShellType::Bash)
        );
        assert_eq!(
            ShellDetector::parse_shell_from_path("/bin/sh"),
            None
        );
    }

    #[test]
    fn test_shell_command_format() {
        assert!(matches!(
            ShellDetector::get_shell_command_format(&ShellType::Zsh),
            ShellCommandFormat::Export
        ));
        assert!(matches!(
            ShellDetector::get_shell_command_format(&ShellType::Bash),
            ShellCommandFormat::Export
        ));
        assert!(matches!(
            ShellDetector::get_shell_command_format(&ShellType::Fish),
            ShellCommandFormat::Set
        ));
        assert!(matches!(
            ShellDetector::get_shell_command_format(&ShellType::Unknown("test".to_string())),
            ShellCommandFormat::Export
        ));
    }

    #[test]
    fn test_generate_env_commands_bash_zsh() {
        let mut vars = HashMap::new();
        vars.insert("TEST_VAR".to_string(), "test_value".to_string());
        vars.insert("ANOTHER_VAR".to_string(), "another_value".to_string());

        let commands = ShellDetector::generate_env_commands(&ShellType::Zsh, &vars).unwrap();
        
        assert!(commands.contains("export TEST_VAR='test_value'"));
        assert!(commands.contains("export ANOTHER_VAR='another_value'"));
    }

    #[test]
    fn test_generate_env_commands_fish() {
        let mut vars = HashMap::new();
        vars.insert("TEST_VAR".to_string(), "test_value".to_string());

        let commands = ShellDetector::generate_env_commands(&ShellType::Fish, &vars).unwrap();
        
        assert_eq!(commands, "set -x TEST_VAR 'test_value'");
    }

    #[test]
    fn test_generate_env_commands_empty() {
        let vars = HashMap::new();
        let commands = ShellDetector::generate_env_commands(&ShellType::Zsh, &vars).unwrap();
        assert!(commands.is_empty());
    }

    #[test]
    fn test_generate_env_commands_invalid_var_name() {
        let mut vars = HashMap::new();
        vars.insert("123INVALID".to_string(), "value".to_string());

        let result = ShellDetector::generate_env_commands(&ShellType::Zsh, &vars);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_unset_commands_bash_zsh() {
        let vars = vec!["TEST_VAR".to_string(), "ANOTHER_VAR".to_string()];
        
        let commands = ShellDetector::generate_unset_commands(&ShellType::Zsh, &vars).unwrap();
        
        assert!(commands.contains("unset TEST_VAR"));
        assert!(commands.contains("unset ANOTHER_VAR"));
    }

    #[test]
    fn test_generate_unset_commands_fish() {
        let vars = vec!["TEST_VAR".to_string()];
        
        let commands = ShellDetector::generate_unset_commands(&ShellType::Fish, &vars).unwrap();
        
        assert_eq!(commands, "set -e TEST_VAR");
    }

    #[test]
    fn test_escape_value_for_export() {
        assert_eq!(
            ShellDetector::escape_value_for_export("simple"),
            "simple"
        );
        assert_eq!(
            ShellDetector::escape_value_for_export("with'quote"),
            "with'\"'\"'quote"
        );
        assert_eq!(
            ShellDetector::escape_value_for_export("multiple'quotes'here"),
            "multiple'\"'\"'quotes'\"'\"'here"
        );
    }

    #[test]
    fn test_escape_value_for_fish() {
        assert_eq!(
            ShellDetector::escape_value_for_fish("simple"),
            "simple"
        );
        assert_eq!(
            ShellDetector::escape_value_for_fish("with'quote"),
            "with'\"'\"'quote"
        );
    }

    #[test]
    fn test_shell_type_display() {
        assert_eq!(format!("{}", ShellType::Zsh), "zsh");
        assert_eq!(format!("{}", ShellType::Fish), "fish");
        assert_eq!(format!("{}", ShellType::Bash), "bash");
        assert_eq!(format!("{}", ShellType::Unknown("custom".to_string())), "unknown(custom)");
    }

    #[test]
    fn test_shell_integration_instructions() {
        let zsh_instructions = ShellDetector::get_shell_integration_instructions(&ShellType::Zsh);
        assert!(zsh_instructions.contains("~/.zshrc"));
        assert!(zsh_instructions.contains("envswitch-use"));

        let fish_instructions = ShellDetector::get_shell_integration_instructions(&ShellType::Fish);
        assert!(fish_instructions.contains("~/.config/fish/config.fish"));
        assert!(fish_instructions.contains("function envswitch-use"));

        let bash_instructions = ShellDetector::get_shell_integration_instructions(&ShellType::Bash);
        assert!(bash_instructions.contains("~/.bashrc"));
        assert!(bash_instructions.contains("envswitch-use"));

        let unknown_instructions = ShellDetector::get_shell_integration_instructions(&ShellType::Unknown("custom".to_string()));
        assert!(unknown_instructions.contains("custom"));
        assert!(unknown_instructions.contains("not fully supported"));
    }

    #[test]
    fn test_detect_shell_with_env_vars() {
        // This test sets environment variables to simulate different shells
        // Note: These tests might interfere with each other if run in parallel
        
        // Test ZSH_VERSION detection
        env::set_var("ZSH_VERSION", "5.8");
        // We can't easily test this without affecting the actual detection
        env::remove_var("ZSH_VERSION");
        
        // Test FISH_VERSION detection  
        env::set_var("FISH_VERSION", "3.1.0");
        env::remove_var("FISH_VERSION");
        
        // Test BASH_VERSION detection
        env::set_var("BASH_VERSION", "5.0.0");
        env::remove_var("BASH_VERSION");
    }
}