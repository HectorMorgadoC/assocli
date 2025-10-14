use clap::{Parser, Subcommand};
use console::style;
use dialoguer::{Confirm, Input};
mod utils;
use crate::utils::command::utils_command::*;

#[derive(Parser)]
#[command(
    name = "asso",
    version,
    about = "AssoCLI - Crea proyectos modulares en Rust"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Crea un nuevo proyecto modular
    New {
        /// Nombre del proyecto
        name: Option<String>,
    },
    /// Muestra información del proyecto
    Info,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            handle_new(name);
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

fn handle_new(name: &Option<String>) {
    let project_name = match name {
        Some(_name) => _name.clone(),
        None => Input::new()
            .with_prompt("Nombre del proyecto")
            .default("mi_app".into())
            .interact_text()
            .unwrap(),
    };

    let confirmed = Confirm::new()
        .with_prompt(
            style(format!("¿Deseas crear el proyecto '{project_name}' ?"))
                .blue()
                .bold()
                .to_string(),
        )
        .default(true)
        .interact()
        .unwrap();

    if confirmed {
        println!(
            "{}",
            style(format!(" Creando el proyecto '{project_name}' "))
                .on_white()
                .bold()
        );
        let status = cargo_new(&project_name);

        if status.is_some() {
            println!("{}", style("  Proyecto creado").on_bright().bold());
        } else {
            println!("{}", style("  Proyecto no creado").red().bold());
        }
    } else {
        println!("{}", style("❌ Cancelado por el usuario.").red().bold());
    }
}
