mod command;
mod shell;

use clap::{Parser, Subcommand};

/// Shipping Instructions (SI) - a helper tool for Cargo
#[derive(Parser)]
#[command(arg_required_else_help = true, version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    /// Run cargo-update and then update dependencies in the manifest that are behind the versions recorded in the local lock file
    Update,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Update => {
            if let Err(error) = command::update() {
                shell::print_error("Error", error);
                std::process::exit(1)
            }
        },
    }
}
