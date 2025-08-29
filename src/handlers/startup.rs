use std::path::PathBuf;

/// Check if this is the first time using envswitch
pub fn should_show_welcome() -> bool {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("envswitch");
    
    let config_file = config_dir.join("config.json");
    let welcome_file = config_dir.join(".welcome_shown");
    
    // Show welcome if config doesn't exist and welcome hasn't been shown
    !config_file.exists() && !welcome_file.exists()
}

/// Show welcome message for first-time users
pub fn show_welcome_message() {
    println!("🎉 Welcome to EnvSwitch!");
    println!("========================");
    println!();
    println!("EnvSwitch helps you manage and switch between different sets of environment variables.");
    println!("Perfect for managing API keys, database connections, and development environments!");
    println!();
    println!("🚀 Quick Start:");
    println!("  1. Create your first configuration:");
    println!("     envswitch set my-config -e API_KEY=your-key -e API_URL=https://api.example.com");
    println!();
    println!("  2. Switch to it:");
    println!("     eval \"$(envswitch use my-config)\"");
    println!();
    println!("  3. Check status:");
    println!("     envswitch status");
    println!();
    println!("📚 For a complete tutorial, run: envswitch tutorial");
    println!("❓ For help with any command, use: envswitch <command> --help");
    println!();
    
    // Create welcome marker file
    if let Some(config_dir) = dirs::config_dir() {
        let envswitch_dir = config_dir.join("envswitch");
        if let Ok(()) = std::fs::create_dir_all(&envswitch_dir) {
            let welcome_file = envswitch_dir.join(".welcome_shown");
            let _ = std::fs::write(welcome_file, "");
        }
    }
}