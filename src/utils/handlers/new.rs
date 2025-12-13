use crate::shared::global::PROJECT_PATH;
use crate::utils::common::remove_directory::delete_folder;
use crate::utils::{
    command::new_project::add_project::NewProject, common::clear_terminal::clear_terminal,
};
use console::style;
use dialoguer::Confirm;
use std::{path::PathBuf, process, thread, time::Duration};

pub fn handle_new(project_name: &str) {
    let path_home = PROJECT_PATH.as_deref();

    if let Some(_path) = path_home {
        let mut new_project = NewProject::new(_path.to_path_buf(), PathBuf::new());

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

            if new_project.create_project(project_name) {
                new_project.create_actix();
                new_project.create_app_structure();
                new_project.create_mod_main();
                new_project.create_env_file();
                new_project.create_files_common();
                new_project.create_files_state();
                new_project.create_env_rs();
                new_project.create_main_rs();
                new_project.add_root_template();
                clear_terminal();
                println!("{}", style("  Project created.").on_bright().bold());
            } else {
                //clear_terminal();
                println!("{}", style("  Project not created.").red().bold());
                delete_folder(&new_project.project_path, project_name);
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
