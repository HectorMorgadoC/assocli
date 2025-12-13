use crate::utils::common::{
    add_dependency::add_dependency,
    create_dir::create_dir,
    create_file::create_file,
    file::{load_template, load_template_arg, modify_file, overwrite_file},
    remove_directory::delete_folder,
};
use console::style;

pub struct NewTemplate {
    pub module_path: std::path::PathBuf,
    pub project_path: std::path::PathBuf,
    pub name_module: String,
}

impl NewTemplate {
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

    pub fn check_project_path(&self) {
        if !self.project_path.exists() {
            eprintln!(
                "{}",
                style("  Error creating module, project does not exist.")
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }
    }

    pub fn add_dependency(&self) {
        let project_path = self.project_path.as_os_str().to_str();

        if let Some(path) = project_path {
            add_dependency("tera", None, path);
            std::thread::sleep(std::time::Duration::from_secs(1));
        } else {
            eprintln!(
                "{}",
                style(format!(
                    "  Error reading project path {}",
                    self.project_path.display()
                ))
                .red()
                .bold()
            );
            std::process::exit(1)
        }
    }

    pub fn create_folder_module(&self) {
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

    pub fn create_dir_templates(&self) {
        let path_dir_template = self.project_path.join("templates").join(&self.name_module);

        let path_template_shared = self.project_path.join("src/app/shared/template");

        if path_dir_template.exists() {
            println!(
                "{}",
                style("  Template directory already exists. Continue...")
                    .yellow()
                    .bold()
            );
            return;
        }

        create_dir(&path_dir_template);

        if path_template_shared.exists() {
            println!(
                "{}",
                style("  Template directory already exists. Continue...")
                    .yellow()
                    .bold()
            );
            return;
        }

        create_dir(&path_template_shared);
    }

    pub fn create_dir_static_file(&self) {
        let path_dir_static = self.project_path.join("templates/static");

        if path_dir_static.exists() {
            println!(
                "{}",
                style("  Static directory already exists. Continue...")
                    .yellow()
                    .bold()
            );
            return;
        }

        create_dir(&path_dir_static);
    }

    pub fn create_templates_files(&self) {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let path_dir_template = self.project_path.join("templates");
        let path_dir_template_module = path_dir_template.join(&self.name_module);
        let path_dir_static_files = self.project_path.join("templates/static");
        let path_template_shared = self.project_path.join("src/app/shared/template");

        load_template_arg(
            "engine.rs",
            &path_template_shared.join("engine.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));

        create_file(path_template_shared.join("mod.rs"), Some("pub mod engine;"));

        load_template_arg(
            "html/index.html",
            &path_dir_template_module.join("index.html"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));

        load_template_arg(
            "html/create.html",
            &path_dir_template_module.join("create.html"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));

        load_template_arg(
            "html/delete.html",
            &path_dir_template_module.join("delete.html"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));

        load_template_arg(
            "html/search.html",
            &path_dir_template_module.join("search.html"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));

        load_template_arg(
            "html/update.html",
            &path_dir_template_module.join("update.html"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));

        if !path_dir_template.join("error.html").exists() {
            load_template("html/error.html", &path_dir_template.join("error.html"));
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        if !path_dir_static_files.join("style.css").exists() {
            load_template(
                "static/styles.css",
                &path_dir_static_files.join("style.css"),
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    pub fn create_module_files(&self) {
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
            "handlers_template.rs",
            &self.module_path.join("handlers.rs"),
            &self.name_module,
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        load_template_arg(
            "config_mod_template.rs",
            &self.module_path.join("mod.rs"),
            &self.name_module,
        );

        std::thread::sleep(std::time::Duration::from_secs(1));
        //load_template("main.rs", path_module);

        println!("{}", style(" module created successfully").green().bold());
    }

    pub fn reconfigure_file_handler_error(&self) {
        println!("{}", style("  Configuring file errors...").blue().bold());
        std::thread::sleep(std::time::Duration::from_secs(2));

        let path_file_error = std::path::PathBuf::new().join(format!(
            "{}/src/app/shared/common/error.rs",
            &self.project_path.display()
        ));

        let add_dependency: (String, String) = (
            r###"    response::{IntoResponse, Response},
};
use serde::{Serialize, Serializer};"###
                .to_string(),
            r###"    response::{Html, IntoResponse, Response},
};
use serde::{Serialize, Serializer};
use tera::{Context, Tera};"###
                .to_string(),
        );

        let add_content = r###"pub fn err_render(
        state: std::sync::Arc<Tera>,
        status: StatusCode,
        detail: impl Into<String>,
        ) -> (StatusCode, Html<String>) {
        let mut context = Context::new();

        context.insert("status", &status.as_u16());
        context.insert("title", &format!("Error {}", status));
        context.insert("detail", &detail.into());

        let rendered = state.render("error.html", &context);

        match rendered {
        Ok(value) => (status, Html(value)),
        Err(err) => {
            tracing::error!("Error loading error template:  \n");
            panic!("{err}")
            }
        }
        }"###;

        if let Ok(content) = std::fs::read_to_string(&path_file_error) {
            if !content.contains(r###"use tera::{Context, Tera};"###) {
                modify_file(&path_file_error, &add_dependency.0, &add_dependency.1);
            }

            if !content.contains(r###"pub fn err_render"###) {
                overwrite_file(&path_file_error, add_content);
            }
        } else {
            eprintln!(
                "{}",
                style(format!(
                    "  Error reading content {}",
                    &path_file_error.display()
                ))
                .red()
                .bold()
            );

            std::process::exit(1)
        }
    }

    pub fn reconfigure_file_state(&self) {
        println!("{}", style("  Configuring file state...").blue().bold());
        std::thread::sleep(std::time::Duration::from_secs(2));

        let path_file_state = std::path::PathBuf::new().join(format!(
            "{}/src/app/shared/state/state.rs",
            &self.project_path.display()
        ));

        let add_dependency: (String, String) = (
            r###"#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        tracing::info!("🔧 Initializing AppState...");
        Self {}
    }
}"###
                .to_string(),
            r###"use crate::app::shared::template::engine::TemplateEngine;

#[derive(Clone)]
pub struct AppState {
    pub templates: TemplateEngine,
}

impl AppState {
    pub fn new() -> Self {
        tracing::info!("🔧 Initializing AppState...");
        Self {
            templates: TemplateEngine::new(),
        }
    }
}"###
                .to_string(),
        );

        if let Ok(content) = std::fs::read_to_string(&path_file_state) {
            if !content.contains(r###"use crate::app::shared::template::engine::TemplateEngine;"###)
            {
                modify_file(&path_file_state, &add_dependency.0, &add_dependency.1);
            }
        } else {
            eprintln!(
                "{}",
                style(format!(
                    "  Error reading content {}",
                    &path_file_state.display()
                ))
                .red()
                .bold()
            );

            std::process::exit(1)
        }
    }

    pub fn reconfigure_module_shared(&self) {
        println!("{}", style("  Configuring module shared...").blue().bold());
        std::thread::sleep(std::time::Duration::from_secs(2));

        let path_mod_shared = std::path::PathBuf::new().join(format!(
            "{}/src/app/shared/mod.rs",
            &self.project_path.display()
        ));

        let add_content: (String, String) = (
            r###"pub mod state;"###.to_string(),
            r###"pub mod state;
pub mod template;"###
                .to_string(),
        );

        if let Ok(content) = std::fs::read_to_string(&path_mod_shared) {
            if !content.contains(r###"pub mod template;"###) {
                modify_file(&path_mod_shared, &add_content.0, &add_content.1);
            }
        } else {
            eprintln!(
                "{}",
                style(format!(
                    "  Error reading content {}",
                    &path_mod_shared.display()
                ))
                .red()
                .bold()
            );

            std::process::exit(1)
        }
    }

    pub fn inject_module_main(&self) {
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
