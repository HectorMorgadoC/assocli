use crate::shared::global::PROJECT_PATH;
use console::style;

pub fn check_toml_project(name_project: &str) -> bool {
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
            std::process::exit(1);
        }

        true
    } else {
        eprintln!(
            "{}",
            style("  Error searching for project in team").red().bold()
        );
        false
    }
}
