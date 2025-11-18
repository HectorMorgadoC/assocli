use console::style;
use std::process::Command;

pub fn add_dependency(arg: &str, features: Option<&str>, path: &str) {
    let mut command = Command::new("cargo");

    command.arg("add").arg(arg);
    if let Some(value) = features {
        command.arg("--features").arg(value);
    }

    command.current_dir(path);

    let status = command.status();

    match status {
        Ok(status) if status.success() => {
            println!(
                "{}",
                style(format!("  {arg} successfully added!"))
                    .green()
                    .bold()
            );
        }
        Ok(_) => {
            eprintln!(
                "{}",
                style(format!(
                    "  The installation failed {arg}. ¿you have installed 'cargo-edit'?"
                ))
                .yellow()
                .bold()
            );
        }
        Err(err) => {
            eprintln!(
                "{}",
                style(format!("  Error executing 'cargo add {arg}': {err}",))
                    .red()
                    .bold()
            );
        }
    }
}
