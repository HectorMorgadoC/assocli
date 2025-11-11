use crate::shared::global::PROJECT_PATH;
use console::style;
use std::{fs, path::Path, process::Command, thread, time::Duration};

pub fn new(name: &str) -> Option<String> {
    let path_home = &PROJECT_PATH.as_deref();

    if let Some(_path) = path_home {
        println!("📁 Creating project...");

        thread::sleep(Duration::from_secs(1));
        if fs::create_dir_all(_path).is_err() {
            eprintln!(
                "{}",
                style("  Error creating project directory").red().bold()
            );
            return None;
        }

        let path = _path.to_str().unwrap_or("");
        let path_project = format!("{path}/{name}");

        if Path::new(&path_project).exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  The project '{name}' already exists in '{path}'"
                ))
                .yellow()
                .bold()
            );
            return None;
        }

        let status = Command::new("cargo")
            .arg("new")
            .arg(name)
            .current_dir(path)
            .status();

        if let Err(err) = &status {
            eprintln!(
                "{}",
                style(format!("  Error executing cargo new: {err}"))
                    .red()
                    .bold()
            );
            return None;
        }

        Some(path_project)
    } else {
        eprintln!(
            "{}",
            style("  Error finding the path in the system user")
                .red()
                .bold()
        );
        None
    }
}
