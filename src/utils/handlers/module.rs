use crate::{
    shared::global::PROJECT_PATH,
    utils::{command::api::type_api::NewModule, common::selection_module_type::choose_module_type},
};
use console::style;

pub fn handler_module(name_module: &str, name_project: &str) {
    let mut path_project = std::path::PathBuf::new();

    if let Some(_path) = PROJECT_PATH.as_deref() {
        path_project = std::path::PathBuf::from(_path).join(name_project);

        println!("{}", path_project.display());
        if !path_project.exists() {
            eprintln!(
                "{}",
                style(format!(
                    "The project {} does not exist to create module {}",
                    name_project, name_module
                ))
                .on_white()
                .bold()
            );
            std::process::exit(1)
        }
    } else {
        eprintln!(
            "{}",
            style("  Error creating project path to query")
                .red()
                .bold()
        )
    }

    match choose_module_type(name_module).as_str() {
        "API" => {
            let new_module: NewModule = NewModule::new(
                path_project
                    .clone()
                    .join(format!("src/app/module/{}", name_module)),
                path_project.clone(),
                name_module.to_string(),
            );

            new_module.create_folder_module();
            new_module.create_module_files();
        }
        "Template" => println!("template"),
        _ => println!("No pasa nada"),
    }

    /*
    *
    fn handle_api_module(name: &str) {
    println!("🔧 Generating API module: {}", name);
    }

    fn handle_template_module(name: &str) {
    println!("🧩 Generating Template module: {}", name);
    }

    */
}
