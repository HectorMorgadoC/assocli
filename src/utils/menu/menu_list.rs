use crate::models::command::command_model::*;
use crate::utils::handlers::{new::handle_new, release::handler_release};
use clap::Parser;
use console::style;

pub fn commands() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            let _name = name.clone().unwrap_or("app".to_string());
            handle_new(&_name);
        }

        Commands::Release { name_proyect } => {
            handler_release(name_proyect);
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
