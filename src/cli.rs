use clap::{Parser, Subcommand};
use std::io::{self, Write};
use crate::calculator::Calculator;

#[derive(Parser)]
#[command(name = "calculator-app")]
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
            // Interactive mode
            println!("Calculator CLI - Interactive Mode");
            println!("Enter expressions like '2 + 3' or 'quit' to exit");
            
            loop {
                print!("> ");
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                let input = input.trim();
                
                if input.is_empty() {
                    continue;
                }
                
                if input == "quit" || input == "exit" {
                    println!("Goodbye!");
                    break;
                }
                
                match calculator.evaluate(input) {
                    Ok(result) => println!("= {}", result),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
    
    Ok(())
}