use console::style;

pub fn delete_folder(path: &std::path::PathBuf, name_directory: &str) {
    let command = std::process::Command::new("rm")
        .arg("-rf")
        .arg(path)
        .status();

    if command.is_err() {
        eprintln!(
            "{}",
            style(format!(
                "  Command execution failed when trying to delete directory {}.",
                name_directory
            ))
            .red()
            .bold()
        );
        std::process::exit(1);
    }

    println!(
        "{}",
        style(format!("  Directory {} deleted", name_directory))
            .yellow()
            .bold()
    );
}
