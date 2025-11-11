use crate::models::command::command_model::*;
use crate::utils::handlers::{new::handle_new, release::handler_release};
use clap::Parser;
use console::style;

pub fn commands() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            handle_new(name);
        }

        Commands::Release => {
            handler_release();
        }

        Commands::Info => {
            println!(
                "{}",
                style("AssoCLI v0.1.0 — tu asistente para construir apps modulares 🦀")
                    .on_blue()
                    .bold()
            );
        }
    }
}
