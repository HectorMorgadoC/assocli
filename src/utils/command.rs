pub mod utils_command {

    use console::style;
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use yansi::Paint;

    pub fn cargo_new(name: &str) -> Option<String> {
        let home_path = env::var("HOME");
        let mut destination = String::new();
        //let mut project_path = String::new();

        if let Ok(path) = home_path {
            let new_path = format!("{path}/Asso");
            destination.push_str(&new_path);
        } else {
            eprintln!(
                "{}",
                style(" ERROR IN HOME ENVIRONMENT VARIABLE").red().bold()
            );
            return None;
        }

        if !Path::new(&destination).exists() {
            println!("📁 Carpeta 'proyectos' no encontrada. Creándola...");

            if fs::create_dir_all(&destination).is_err() {
                eprintln!(
                    "{}",
                    style(" ERROR CREATING PROJECT DIRECTORY").red().bold()
                );
                return None;
            }
        }

        let project_path = format!("{destination}/{name}");

        if Path::new(&project_path).exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  El proyecto '{name}' ya existe en '{destination}'"
                ))
                .yellow()
                .bold()
            );
            return None;
        }

        let status = Command::new("cargo")
            .arg("new")
            .arg(name)
            .current_dir(destination)
            .status();

        if status.is_ok() {
            Some(project_path)
        } else {
            eprintln!("{}", style(" ERROR EXECUTING CARGO NEW").red().bold());
            None
        }
    }
}
