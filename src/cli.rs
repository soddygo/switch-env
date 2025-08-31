use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "envswitch")]
#[command(about = "A tool for managing and switching environment variable configurations")]
#[command(long_about = "EnvSwitch helps you manage different sets of environment variables and quickly switch between them. Perfect for switching between different AI model configurations, development environments, or any other environment-specific settings.")]
#[command(version = "0.1.0")]
#[command(author = "EnvSwitch Team")]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create or update a configuration
    #[command(alias = "add")]
    Set {
        /// Configuration alias name
        alias: String,
        /// Environment variables in KEY=VALUE format
        #[arg(short, long, value_parser = parse_env_var)]
        env: Vec<(String, String)>,
        /// Description for the configuration
        #[arg(short, long)]
        description: Option<String>,
        /// Read environment variables from a file
        #[arg(short, long, conflicts_with = "env")]
        file: Option<String>,
        /// Replace all variables instead of merging (only for updates)
        #[arg(short, long)]
        replace: bool,
        /// Interactive mode to add variables one by one
        #[arg(short, long, conflicts_with_all = ["env", "file"])]
        interactive: bool,
    },
    /// Switch to a configuration
    #[command(alias = "switch")]
    Use {
        /// Configuration alias to activate
        alias: String,
        /// Show commands without executing (dry run)
        #[arg(short, long)]
        dry_run: bool,
    },
    /// List all configurations
    #[command(alias = "ls")]
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
        /// Display in table format
        #[arg(short, long)]
        table: bool,
        /// Show only active configuration
        #[arg(short, long)]
        active: bool,
    },
    /// Show current active configuration and environment status
    #[command(alias = "info")]
    Status {
        /// Show only Claude-specific variables
        #[arg(short, long)]
        claude: bool,
        /// Display in table format
        #[arg(short, long)]
        table: bool,
        /// Show only mismatched variables
        #[arg(short, long)]
        mismatched: bool,
    },
    /// Edit a configuration interactively
    /// 
    /// Opens an interactive editor to modify environment variables.
    /// Allows adding, editing, and removing variables with real-time validation.
    /// 
    /// Example:
    ///   envswitch edit my-config
    Edit {
        /// Configuration alias to edit
        /// Creates a new configuration if it doesn't exist
        alias: String,
    },
    /// Delete a configuration
    /// 
    /// Removes a configuration permanently. Shows interactive confirmation
    /// unless --force is used. Cannot delete the currently active configuration.
    /// 
    /// Examples:
    ///   envswitch delete old-config
    ///   envswitch delete temp-config --force
    #[command(alias = "rm")]
    Delete {
        /// Configuration alias to delete
        alias: String,
        /// Skip confirmation prompt and delete immediately
        /// Use with caution as this action cannot be undone
        #[arg(short, long)]
        force: bool,
        /// Show verbose output during deletion
        #[arg(short, long)]
        verbose: bool,
    },
    /// Export configurations to a file
    /// 
    /// Examples:
    ///   envswitch export --output my-configs.json
    ///   envswitch export --configs dev,prod --format env --output configs.env
    ///   envswitch export --metadata --pretty --output detailed-configs.json
    Export {
        /// Output file path (default: envswitch_export.json)
        /// Supports .json, .env, and .yaml extensions for format detection
        #[arg(short, long)]
        output: Option<String>,
        /// Export only specific configurations (comma-separated)
        /// Example: --configs dev,staging,prod
        #[arg(short, long, value_delimiter = ',')]
        configs: Vec<String>,
        /// Export format: json (default), env, or yaml
        /// Format is auto-detected from file extension if not specified
        #[arg(short, long, default_value = "json")]
        format: String,
        /// Include metadata such as creation timestamps and descriptions
        #[arg(short, long)]
        metadata: bool,
        /// Pretty print JSON output for better readability
        #[arg(short, long)]
        pretty: bool,
    },
    /// Import configurations from a file
    /// 
    /// Supports JSON, ENV, and YAML formats with automatic format detection.
    /// Creates automatic backups when --backup is used.
    /// 
    /// Examples:
    ///   envswitch import configs.json
    ///   envswitch import --backup --merge team-configs.json
    ///   envswitch import --dry-run --verbose new-configs.yaml
    Import {
        /// Input file path (supports .json, .env, .yaml formats)
        /// Format is automatically detected from file content and extension
        file: String,
        /// Overwrite existing configurations without confirmation
        /// Use with caution as this will replace existing configs
        #[arg(short, long)]
        force: bool,
        /// Merge with existing configurations instead of replacing
        /// Combines variables from imported and existing configs
        #[arg(short, long)]
        merge: bool,
        /// Preview import changes without actually importing (dry run)
        /// Shows what configurations would be created or modified
        #[arg(short, long)]
        dry_run: bool,
        /// Skip validation of imported configurations for faster import
        /// Only recommended for trusted configuration files
        #[arg(short, long)]
        skip_validation: bool,
        /// Create backup of existing configurations before import
        /// Backup is saved to ~/.config/envswitch/backups/
        #[arg(short, long)]
        backup: bool,
    },
    /// Show shell integration instructions and generate setup scripts
    Setup {
        /// Target shell (auto-detected if not specified)
        #[arg(short, long)]
        shell: Option<String>,
        /// Generate shell integration script
        #[arg(short, long)]
        generate: bool,
        /// Output file for generated script
        #[arg(short, long)]
        output: Option<String>,
        /// Install shell integration automatically
        #[arg(short, long)]
        install: bool,
        /// Generate wrapper script with enhanced functionality
        #[arg(short, long)]
        wrapper: bool,
    },
    /// Generate shell initialization code for eval
    Init {
        /// Target shell (auto-detected if not specified)
        #[arg(short, long)]
        shell: Option<String>,
        /// Generate completion scripts
        #[arg(short, long)]
        completions: bool,
    },
    /// Show getting started guide and examples
    #[command(alias = "guide")]
    Tutorial {
        /// Show advanced usage examples
        #[arg(short, long)]
        advanced: bool,
        /// Show examples for specific use case
        #[arg(short, long)]
        use_case: Option<String>,
    },
}

/// Parse environment variable in KEY=VALUE format
fn parse_env_var(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid format '{}'. Expected KEY=VALUE", s));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}#[cfg(test
)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env_var_valid() {
        let result = parse_env_var("KEY=value");
        assert!(result.is_ok());
        let (key, value) = result.unwrap();
        assert_eq!(key, "KEY");
        assert_eq!(value, "value");
    }

    #[test]
    fn test_parse_env_var_with_equals_in_value() {
        let result = parse_env_var("URL=https://api.example.com/v1?key=value");
        assert!(result.is_ok());
        let (key, value) = result.unwrap();
        assert_eq!(key, "URL");
        assert_eq!(value, "https://api.example.com/v1?key=value");
    }

    #[test]
    fn test_parse_env_var_empty_value() {
        let result = parse_env_var("EMPTY=");
        assert!(result.is_ok());
        let (key, value) = result.unwrap();
        assert_eq!(key, "EMPTY");
        assert_eq!(value, "");
    }

    #[test]
    fn test_parse_env_var_invalid_no_equals() {
        let result = parse_env_var("INVALID");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid format"));
    }

    #[test]
    fn test_parse_env_var_invalid_empty() {
        let result = parse_env_var("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid format"));
    }

    #[test]
    fn test_parse_env_var_only_equals() {
        let result = parse_env_var("=");
        assert!(result.is_ok());
        let (key, value) = result.unwrap();
        assert_eq!(key, "");
        assert_eq!(value, "");
    }
}