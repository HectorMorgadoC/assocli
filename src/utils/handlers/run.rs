use crate::shared::global::PROJECT_PATH;
use console::style;

pub fn handler_run(name_project: &str) {
    run_cargo_command("run", None, name_project.to_string());
}

fn run_cargo_command(arg: &str, optional_arg: Option<&str>, name_project: String) {
    let path = PROJECT_PATH.as_deref();

    if let Some(_path) = path {
        let dir_project = _path.to_path_buf().join(&name_project);
        let cargo_toml = dir_project.join("Cargo.toml");

        if !cargo_toml.exists() {
            eprintln!("{}", style("  The project does not exist").red().bold());
            std::process::exit(1);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut output = std::process::Command::new("cargo");

        output.arg(arg);

        if let Some(arg) = optional_arg {
            output.arg(arg);
        }

        let command_result = output.current_dir(&dir_project).status();

        if let Ok(_status) = command_result {
            if !_status.success() {
                eprintln!("{}", style(_status.to_string()).red().bold());
            }
        } else {
            eprintln!(
                "{}",
                style("  Error trying to start the process").red().bold()
            )
        }
    } else {
        eprintln!(
            "{}",
            style("  Error searching for project path").red().bold()
        )
    }
}
