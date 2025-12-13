use crate::utils::common::{
    create_dir::create_dir, file::load_template_arg, remove_directory::delete_folder,
};
use console::style;

pub struct NewModule {
    pub module_path: std::path::PathBuf,
    pub project_path: std::path::PathBuf,
    pub name_module: String,
}

impl NewModule {
    pub fn new(
        module_path: std::path::PathBuf,
        project_path: std::path::PathBuf,
        name_module: String,
    ) -> Self {
        Self {
            module_path,
            project_path,
            name_module,
        }
    }

    pub fn create_folder_module(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating module, project does not exist.")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }

        if self.module_path.exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  Error creating directory, a module with the name already exists {}",
                    self.name_module
                ))
                .red()
                .bold()
            );
            std::process::exit(1)
        }
        create_dir(&self.module_path);
    }

    pub fn create_module_files(&self) {
        if !self.module_path.exists() {
            eprintln!(
                "{}",
                style("  Error, the module path does not exist")
                    .red()
                    .bold()
            );

            std::process::exit(1)
        }

        std::thread::sleep(std::time::Duration::from_secs(1));

        load_template_arg(
            "models.rs",
            &self.module_path.join("models.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "repositories.rs",
            &self.module_path.join("repositories.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "services.rs",
            &self.module_path.join("services.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "handlers.rs",
            &self.module_path.join("handlers.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "config_mod.rs",
            &self.module_path.join("mod.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        //load_template("main.rs", path_module);

        println!("{}", style(" module created successfully").green().bold());
        self.inject_module_main();
    }

    fn inject_module_main(&self) {
        if !self.module_path.exists() {
            eprintln!(
                "{}",
                style("  Error, the module path does not exist")
                    .red()
                    .bold()
            );

            std::process::exit(1)
        }

        println!(
            "{}",
            style(format!(
                "  Configuring the implementation of the {} module...",
                &self.name_module
            ))
            .blue()
            .bold()
        );
        std::thread::sleep(std::time::Duration::from_secs(2));

        let path_module_main = std::path::PathBuf::new().join(format!(
            "{}/src/app/module/mod.rs",
            &self.project_path.display()
        ));

        let content_new_module: (String, String) = (
            format!(
                "Router::new()\n\t\t.nest(\"/{}\", {{
            tracing::info!(\"  ├─ 󰕳 Starting module configuration {}....\");
            {}::configure(std::sync::Arc::clone(&_state))
        }})",
                self.name_module, self.name_module, self.name_module
            ),
            format!("mod {};\nuse axum::Router;", self.name_module),
        );

        let content_module_main = std::fs::read_to_string(&path_module_main);

        if let Ok(mut content) = content_module_main {
            if content.contains(&self.name_module) {
                eprintln!(
                    "{}",
                    style(format!(
                        "  The module files were deleted, but it is still configured in the {}.",
                        self.project_path.join("src/app/module/mod.rs").display()
                    ))
                    .red()
                    .bold()
                );
                delete_folder(&self.project_path.join("src/app/module"), &self.name_module);
                std::process::exit(1);
            }

            content = content.replace("Router::new()", &content_new_module.0);

            content = content.replace("use axum::Router;", &content_new_module.1);

            if std::fs::write(&path_module_main, content).is_err() {
                eprintln!(
                    "{}",
                    style(format!(
                        "  Error loading new content in main module {}",
                        path_module_main.display()
                    ))
                    .red()
                    .bold()
                );
                std::process::exit(1);
            }
        } else {
            eprintln!(
                "{}",
                style("  Error injecting module into main configuration")
                    .red()
                    .bold()
            );

            std::process::exit(1)
        }
    }
}
