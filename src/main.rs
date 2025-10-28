use clap::Parser;
use console::style;
use dialoguer::{Confirm, Input};
mod utils;
use crate::utils::command::utils_command::*;
mod models;
use crate::models::command::command_model::*;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            handle_new(name);
        }

        Commands::Release => {
            command_release();
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

        if let Some(path) = status {
            create_actix(&path);
            create_app_structure(&path);
            create_env_file(&path);
            create_env_rs(&path);
            create_main_rs(&path);
            println!("{}", style("  Proyecto creado").on_bright().bold());
        } else {
            println!("{}", style("  Proyecto no creado").red().bold());
        }
    } else {
        println!("{}", style("❌ Cancelado por el usuario.").red().bold());
    }
}

fn command_release() {
    let path_cargo_toml = find_cargo_root();

    if let Some(path) = path_cargo_toml {
        run_cargo_command("build", Some("--release"), path.clone());
        println!("{}", style("  Compilado exitoso").green().bold());
        lift_release_service(path);
    } else {
        println!("{}", style("  Compilado fallido").red().bold());
    }
}
