use crate::shared::global::PROJECT_PATH;
use crate::utils::{
    command::new_project::new_project::NewProject, common::clear_terminal::clear_terminal,
};
use console::style;
use dialoguer::Confirm;
use std::{path::PathBuf, process, thread, time::Duration};

pub fn handle_new(project_name: &str) {
    let path_home = PROJECT_PATH.as_deref();

    if let Some(_path) = path_home {
        let mut path_project = NewProject::new(_path.to_path_buf(), PathBuf::new());

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

            thread::sleep(Duration::from_secs(1));

            if path_project.create_project(project_name) {
                path_project.create_actix();
                path_project.create_app_structure();
                path_project.create_env_file();
                path_project.create_env_rs();
                path_project.create_main_rs();
                clear_terminal();
                println!("{}", style("  Project created.").on_bright().bold());
            } else {
                clear_terminal();
                println!("{}", style("  Project not created.").red().bold());
            }
        } else {
            println!("{}", style("❌ Cancelled by the user.").red().bold());
        }
    } else {
        eprintln!(
            "{}",
            style("  Error obtaining the system's HOME path.")
                .red()
                .bold()
        );
        process::exit(1)
    }
}
