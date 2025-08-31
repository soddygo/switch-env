use crate::cli::Commands;
use crate::config::FileConfigManager;
use crate::env::ShellEnvironmentManager;
use crate::commands::*;
use std::error::Error;

/// Route commands to their respective handlers
pub fn run_command(command: Commands, verbose: bool) -> Result<(), Box<dyn Error>> {
    let config_manager = FileConfigManager::new()?;
    let env_manager = ShellEnvironmentManager::new();
    
    match command {
        Commands::Set { alias, env, description, file, replace, interactive } => {
            handle_set_command(&config_manager, alias, env, description, file, replace, interactive, verbose)?;
        }
        Commands::Use { alias, dry_run } => {
            handle_use_command(&config_manager, &env_manager, alias, dry_run, verbose)?;
        }
        Commands::List { verbose: list_verbose, table, active } => {
            handle_list_command(&config_manager, list_verbose || verbose, table, active)?;
        }
        Commands::Status { claude, table, mismatched } => {
            handle_status_command(&config_manager, &env_manager, claude, table, mismatched, verbose)?;
        }
        Commands::Edit { alias } => {
            handle_edit_command(&config_manager, alias, verbose)?;
        }
        Commands::Delete { alias, force, verbose: cmd_verbose } => {
            handle_delete_command(&config_manager, alias, force, verbose || cmd_verbose)?;
        }
        Commands::Export { output, configs, format, metadata, pretty } => {
            handle_export_command(&config_manager, output, configs, format, metadata, pretty, verbose)?;
        }
        Commands::Import { file, force, merge, dry_run, skip_validation, backup } => {
            handle_import_command(&config_manager, file, force, merge, dry_run, skip_validation, backup, verbose)?;
        }
        Commands::Setup { shell, generate, output, install, wrapper } => {
            handle_setup_command(&env_manager, shell, generate, output, install, wrapper, verbose)?;
        }
        Commands::Init { shell, completions } => {
            handle_init_command(&env_manager, shell, completions, verbose)?;
        }
        Commands::Tutorial { advanced, use_case } => {
            handle_tutorial_command(advanced, use_case, verbose)?;
        }
    }
    
    Ok(())
}