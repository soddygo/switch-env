use std::error::Error;

/// Enhanced error handling with user-friendly messages and suggestions
pub fn handle_error(error: &Box<dyn Error>, verbose: bool) {
    // Check if this is a known error type that we can provide better messages for
    if let Some(config_error) = error.downcast_ref::<crate::error::ConfigError>() {
        eprintln!("❌ {}", config_error.user_message());
        
        // Provide additional context based on error type
        match config_error {
            crate::error::ConfigError::ConfigNotFound(_) => {
                eprintln!("💡 Tip: Use 'envswitch list' to see all available configurations");
                eprintln!("   Or use 'envswitch set <name>' to create a new configuration");
            }
            crate::error::ConfigError::FileError(_) => {
                eprintln!("💡 Tip: Check that you have write permissions to ~/.config/envswitch/");
                eprintln!("   You can also try running: mkdir -p ~/.config/envswitch");
            }
            crate::error::ConfigError::JsonError(_) => {
                eprintln!("💡 Tip: Your configuration file may be corrupted");
                eprintln!("   You can backup and recreate it, or restore from a backup");
                eprintln!("   Use 'envswitch export' to backup current configurations");
            }
            crate::error::ConfigError::InvalidConfigName(_) => {
                eprintln!("💡 Tip: Configuration names should contain only letters, numbers, hyphens, and underscores");
                eprintln!("   Examples: 'my-config', 'dev_env', 'production123'");
            }
            _ => {}
        }
    } else if let Some(env_error) = error.downcast_ref::<crate::error::EnvError>() {
        eprintln!("❌ {}", env_error.user_message());
        
        match env_error {
            crate::error::EnvError::ShellDetectionFailed => {
                eprintln!("💡 Tip: Try setting your SHELL environment variable:");
                eprintln!("   export SHELL=/bin/zsh  # or /bin/bash, /usr/bin/fish");
            }
            crate::error::EnvError::UnsupportedShell(_) => {
                eprintln!("💡 Tip: envswitch works best with zsh, bash, or fish");
                eprintln!("   Generic export commands will be used for your shell");
            }
            _ => {}
        }
    } else {
        // Generic error handling
        eprintln!("❌ Error: {}", error);
        
        // Check for common error patterns and provide suggestions
        let error_msg = error.to_string().to_lowercase();
        if error_msg.contains("permission denied") {
            eprintln!("💡 Tip: Check file permissions and try running with appropriate privileges");
        } else if error_msg.contains("not found") {
            eprintln!("💡 Tip: Make sure the file or configuration exists");
            eprintln!("   Use 'envswitch list' to see available configurations");
        } else if error_msg.contains("already exists") {
            eprintln!("💡 Tip: Use a different name or use 'envswitch edit' to modify existing configuration");
        }
    }
    
    if verbose {
        eprintln!("\n🔍 Debug information:");
        eprintln!("Error type: {}", std::any::type_name_of_val(&**error));
        eprintln!("Full error chain:");
        let mut current_error: &dyn Error = &**error;
        let mut level = 0;
        loop {
            eprintln!("  {}: {}", level, current_error);
            match current_error.source() {
                Some(source) => {
                    current_error = source;
                    level += 1;
                }
                None => break,
            }
        }
    }
    
    eprintln!("\n📚 For more help, use 'envswitch --help' or 'envswitch <command> --help'");
}