use crate::utils::common::{
    add_dependency::add_dependency, create_file::create_file, file::file_management::copy_template,
};

use console::style;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct NewProject {
    pub path: std::path::PathBuf,
    pub project_path: std::path::PathBuf,
}

impl NewProject {
    pub fn new(path: std::path::PathBuf, project_path: std::path::PathBuf) -> Self {
        Self { path, project_path }
    }

    pub fn create_project(&mut self, name: &str) -> bool {
        println!("📁 Creating project...");

        std::thread::sleep(std::time::Duration::from_secs(1));
        if std::fs::create_dir_all(self.path.as_path()).is_err() {
            eprintln!(
                "{}",
                style("  Error creating project directory").red().bold()
            );
            return false;
        }

        let path = self.path.to_str().unwrap_or("");
        let path_project = format!("{path}/{name}");

        if std::path::Path::new(&path_project).exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  The project '{name}' already exists in '{path}'"
                ))
                .yellow()
                .bold()
            );
            return false;
        }

        let status = std::process::Command::new("cargo")
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
            return false;
        }

        self.project_path = std::path::PathBuf::from(&path_project);

        true
    }

    pub fn create_actix(&self) {
        let cargo_toml_path = self.project_path.join("Cargo.toml");

        if let Some(project_path) = self.project_path.to_str() {
            if !cargo_toml_path.exists() {
                eprintln!(
                    "{}",
                    style(format!("  'Cargo.toml' was bot found in '{project_path}'",))
                        .red()
                        .bold()
                );
                std::process::exit(1)
            }

            println!("{}", style("🔍 Checking project Cargo...").cyan().bold());

            println!(
                "{}",
                style("  Adding 'actix-web' to the project...")
                    .blue()
                    .bold()
            );

            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("actix-web", None, project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("dotenvy", None, project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));
            add_dependency("tracing", None, project_path);
            std::thread::sleep(std::time::Duration::from_secs(1));

            add_dependency(
                "tracing-subscriber",
                Some("env-filter,fmt,ansi"),
                project_path,
            );
        } else {
            eprintln!(
                "{}",
                style("  Error creating actix, problems with the project path.",)
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    }

    pub fn create_app_structure(&self) {
        let src_path = self.project_path.join("src");
        let app_path = src_path.join("app");

        if let Some(project_path) = self.project_path.to_str() {
            if !src_path.exists() {
                eprintln!(
                    "{}",
                    style(format!(
                        "  The src directory was not found in '{project_path}'",
                    ))
                    .red()
                    .bold()
                );
                std::process::exit(1);
            }

            let subdirs = ["config", "module", "shared"];

            for dir in subdirs {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let path = app_path.join(dir);
                let mod_rs_path = path.join("mod.rs");
                let mod_rs_path_str = mod_rs_path.to_str().unwrap();

                if !path.exists() {
                    if std::fs::create_dir_all(&path).is_err() {
                        eprintln!(
                            "{}",
                            style("  Error creating project subdirectory").red().bold()
                        );
                        std::process::exit(1)
                    }
                    println!(
                        "{}",
                        style(format!("  Created: {}", path.display()))
                            .green()
                            .bold()
                    );
                } else {
                    println!(
                        "{}",
                        style(format!("  It already exists: {}", path.display()))
                            .yellow()
                            .bold()
                    );
                }

                create_file(mod_rs_path_str, None);
            }

            let mod_file_path = app_path.join("mod.rs");
            let mod_content = "pub mod module;\npub mod shared;\npub mod config;\n";

            std::thread::sleep(std::time::Duration::from_secs(1));
            let file = std::fs::File::create(&mod_file_path);
            if let Ok(mut file_created) = file {
                if file_created.write_all(mod_content.as_bytes()).is_err() {
                    println!("{}", style("  Could not write to mod.rs").red().bold());
                    std::process::exit(1)
                }
            } else {
                println!(
                    "{}",
                    style("  The mod.rs file could not be created")
                        .red()
                        .bold()
                );
                std::process::exit(1)
            }

            println!(
                "{}",
                style(format!(
                    "  App structure created correctly in {}'",
                    app_path.display()
                ))
                .cyan()
                .bold()
            );
        } else {
            eprintln!(
                "{}",
                style("  Error creating app structure, problems with the project path.",)
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    }

    pub fn create_env_rs(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating env rs,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        let config_dir = self.project_path.join("src/app/config");
        let env_rs_path = config_dir.join("env.rs");
        let mod_rs_path = config_dir.join("mod.rs");

        if !config_dir.exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  The config folder was not found in {config_dir:?}"
                ))
                .red()
                .bold()
            );
            std::process::exit(1)
        }

        let content = "\npub mod env;";

        std::thread::sleep(std::time::Duration::from_secs(1));
        if mod_rs_path.exists() {
            if std::fs::write(&mod_rs_path, content).is_err() {
                eprintln!(
                    "{}",
                    style(format!("  Write error {mod_rs_path:?}"))
                        .red()
                        .bold()
                );
                std::process::exit(1)
            }
        } else if let Some(path) = mod_rs_path.to_str() {
            create_file(path, Some(content));
        } else {
            eprintln!(
                "{}",
                style("  Error: mod.rs path does not exist").red().bold()
            );
            std::process::exit(1)
        }

        let template = copy_template("env.rs", &env_rs_path);

        if template.is_err() {
            eprintln!("{}", style("  Error loading template").red().bold());
            std::process::exit(1)
        }
    }

    pub fn create_env_file(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating env file,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        let env_path = self.project_path.join(".env");

        std::thread::sleep(std::time::Duration::from_secs(1));
        if env_path.exists() {
            println!(
                "{}",
                style("  .env already exists, omitting...").yellow().bold()
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
            std::process::exit(1)
        }

        let content = r#"ADDRESS="127.0.0.1"
        PORT=3000
        "#;

        if std::fs::write(&env_path, content).is_err() {
            eprintln!(
                "{}",
                style("  Error writing environment variables").red().bold()
            );
            std::process::exit(1)
        }

        println!(
            "{}",
            style(format!("  .env file created in {env_path:?}"))
                .green()
                .bold()
        );
    }

    pub fn create_main_rs(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating main.rs file,problems with the project path")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        let main_path = self.project_path.join("src/main.rs");

        if main_path.exists() {
            let _ = std::fs::remove_file(&main_path);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        let template = copy_template("main.rs", &main_path);

        if template.is_err() {
            eprintln!("{}", style("  Error loading template").red().bold());
            std::process::exit(1)
        }

        println!(
            "{}",
            style("  main.rs created successfully").green().bold()
        );
    }
}
