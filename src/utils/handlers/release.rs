use crate::shared::global::PROJECT_PATH;
use crate::utils::common::{clear_terminal::clear_terminal, progress::progress_bar};
use console::style;
use indicatif::ProgressBar;
use std::path::PathBuf;
use std::process;

pub fn handler_release(name_project: &str) {
    run_cargo_command("build", Some("--release"), name_project.to_string());
}

fn run_cargo_command(arg: &str, optional_arg: Option<&str>, name_project: String) {
    let path = PROJECT_PATH.as_deref();

    if let Some(_path) = path {
        let dir_project = _path.to_path_buf().join(&name_project);
        let cargo_toml = dir_project.join("Cargo.toml");

        if !cargo_toml.exists() {
            eprintln!(
                "{}",
                style("  Error: Cargo.toml file is not present in the project")
                    .red()
                    .bold()
            );
            process::exit(1);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        let progress = ProgressBar::new(100);
        progress_bar::start_progress(progress.clone(), "\nCompiling project...".to_string());

        let mut output = std::process::Command::new("cargo");

        output.arg(arg);

        if let Some(arg) = optional_arg {
            output.arg(arg);
        }

        let child_result = output
            .current_dir(&dir_project)
            .stdout(std::process::Stdio::null())
            .spawn();

        if let Ok(child) = child_result {
            progress_bar::progressing(progress.clone(), child);

            let status = output.output().unwrap();

            progress_bar::progress_message_finish(progress, status.clone());
            if !status.status.success() {
                eprintln!(
                    "{}",
                    style(String::from_utf8_lossy(&status.stderr)).red().bold()
                );
            }
        } else {
            eprintln!(
                "{}",
                style("  Error trying to start the process").red().bold()
            )
        }

        lift_release_service(cargo_toml, dir_project);
    } else {
        eprintln!(
            "{}",
            style("  Error searching for project path").red().bold()
        )
    }
}

pub fn lift_release_service(cargo_toml_path: PathBuf, path: PathBuf) {
    if let Ok(cargo_toml_content) = std::fs::read_to_string(&cargo_toml_path) {
        let project_name = cargo_toml_content
            .lines()
            .find(|line| line.trim_start().starts_with("name"))
            .and_then(|line| line.split('=').nth(1))
            .map(|name| name.trim().trim_matches('"').to_string());

        let Some(project_name) = project_name else {
            eprintln!(
                "{}",
                style(
                    "  The project name could not be determined from Cargo.toml
                "
                )
                .red()
            );
            std::process::exit(1);
        };

        std::thread::sleep(std::time::Duration::from_secs(1));
        println!(
            "{}",
            style(format!("󰍦  Starting project service: {project_name}")).blue()
        );

        let binary_path = path.join("target").join("release").join(&project_name);

        if !binary_path.exists() {
            eprintln!("{}",style(format!(
                "  Binary does not exist in {}. Make sure to compile with `cargo build --release` first.",binary_path.display() 
            )).red());
            std::process::exit(1);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        let child = std::process::Command::new(&binary_path)
            .current_dir(&path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn(); // Con esto pasa la aplicacion a segundo plano. 

        if let Ok(mut _child) = child {
            clear_terminal();
            println!(
                "{}",
                style(format!(
                    "  Service '{project_name}' running (Ctrl+C to stop)"
                ))
                .cyan()
            );

            let status = _child.wait();

            // Opcional: si el hijo falló, salir con su código de error
            if status.is_err() {
                eprintln!("{}", style("Failed to wait on child process").red().bold());
                std::process::exit(1);
            }
        } else {
            eprintln!(
                "{}",
                style(format!("  Error starting service {project_name}"))
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    } else {
        eprintln!(
            "{}",
            style("  Error reading content from Cargo.toml")
                .red()
                .bold()
        );
        std::process::exit(1);
    }
}
