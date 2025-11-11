use crate::utils::command::new_project::new_project;
use crate::utils::others::utils_command::*;
use console::style;
use dialoguer::{Confirm, Input};
use std::{process, thread, time::Duration};

pub fn handle_new(name: &Option<String>) {
    let project_name = match name {
        Some(_name) => _name.clone(),
        None => Input::new()
            .with_prompt("Project name")
            .default("myapp".into())
            .interact_text()
            .unwrap_or_else(|_| {
                eprintln!(
                    "{}",
                    style("  Error generating project name.").red().bold()
                );
                process::exit(1)
            }),
    };

    let confirmed = Confirm::new()
        .with_prompt(
            style(format!(
                "󰺴 ¿Do you want to create the project with the name: '{project_name}' ?."
            ))
            .blue()
            .bold()
            .to_string(),
        )
        .default(true)
        .interact()
        .unwrap_or_else(|err| {
            eprintln!(
                "{}",
                style(format!("  Error creating project: {err}."))
                    .red()
                    .bold()
            );
            process::exit(1)
        });

    if confirmed {
        println!(
            "{}",
            style(format!(" Creating the project '{project_name}'. "))
                .on_white()
                .bold()
        );

        let status = new_project::new(&project_name);

        thread::sleep(Duration::from_secs(1));

        if let Some(path) = status {
            create_actix(&path);
            create_app_structure(&path);
            create_env_file(&path);
            create_env_rs(&path);
            create_main_rs(&path);
            println!("{}", style("  Project created.").on_bright().bold());
        } else {
            println!("{}", style("  Project not created.").red().bold());
        }
    } else {
        println!("{}", style("❌ Cancelled by the user.").red().bold());
    }
}
