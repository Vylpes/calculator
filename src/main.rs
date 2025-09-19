mod calculator;
mod cli;
mod gui;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Cli { expression }) => {
            cli::run_cli_mode(expression)?;
        }
        Some(Commands::Gui) => {
            std::process::exit(gui::run_gui_mode().into());
        }
        None => {
            // Default to GUI mode
            std::process::exit(gui::run_gui_mode().into());
        }
    }

    Ok(())
}
