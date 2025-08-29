mod cli;
mod commands;
mod config;
mod env;
mod error;
mod handlers;
pub mod shell;
mod types;
mod utils;

use clap::Parser;
use cli::Cli;
use std::process;

fn main() {
    let cli = Cli::parse();

    // Check for first-time usage and show welcome message
    if handlers::startup::should_show_welcome() {
        handlers::startup::show_welcome_message();
    }

    if let Err(e) = commands::router::run_command(cli.command, cli.verbose) {
        handlers::error_handling::handle_error(&e, cli.verbose);
        process::exit(1);
    }
}
