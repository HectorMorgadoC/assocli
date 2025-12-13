use crate::models::command::command_model::*;
use crate::utils::handlers::*;
use clap::Parser;
use console::style;

pub fn commands() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            let _name = name.clone().unwrap_or("app".to_string());
            new::handle_new(&_name);
        }

        Commands::Run {
            project: name_project,
        } => {
            run::handler_run(name_project);
        }

        Commands::Release {
            project: name_project,
        } => {
            release::handler_release(name_project);
        }

        Commands::Module { name, project } => {
            module::handler_module(name, project);
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
