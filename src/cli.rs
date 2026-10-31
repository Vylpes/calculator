use clap::{Parser, Subcommand};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::calculator::Calculator;
use crate::session_history::SessionHistory;

#[derive(Parser)]
#[command(name = "vyxcalc")]
#[command(about = "A calculator app with CLI and GUI modes")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run in CLI mode with expression
    Cli {
        /// Expression to evaluate (e.g., "2 + 3")
        expression: Option<String>,
    },
    /// Run in GUI mode (default)
    Gui,
}

pub fn run_cli_mode(expression: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let calculator = Calculator::new();

    match expression {
        Some(expr) => {
            // Single expression mode
            match calculator.evaluate(&expr) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("{}", e),
            }
        }
        None => {
            run_interactive_mode(&calculator)?;
        }
    }
    
    Ok(())
}

fn run_interactive_mode(calculator: &Calculator) -> Result<(), Box<dyn std::error::Error>> {
    println!("Calculator CLI - Interactive Mode");
    println!("Enter expressions like '2 + 3' or 'quit' to exit");
    println!("Use Up/Down arrow keys to cycle through this session's history");

    // In-memory history only: do not load or save a history file.
    let mut rl = DefaultEditor::new()?;
    let mut history = SessionHistory::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let input = line.trim();

                if input.is_empty() {
                    continue;
                }

                if input == "quit" || input == "exit" {
                    println!("Goodbye!");
                    break;
                }

                match calculator.evaluate(input) {
                    Ok(result) => {
                        // Only successful expressions are recalled (not blanks/errors).
                        if history.save(input) {
                            let _ = rl.add_history_entry(input);
                        }
                        println!("= {}", result);
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                return Err(Box::new(err));
            }
        }
    }

    Ok(())
}
