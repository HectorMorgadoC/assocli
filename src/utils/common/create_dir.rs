use console::style;

pub fn create_dir(path: &std::path::PathBuf) {
    if std::fs::create_dir_all(path).is_err() {
        eprintln!(
            "{}",
            style(format!("  Error creating directory {}", &path.display()))
                .red()
                .bold()
        );
        std::process::exit(1)
    }

    println!(
        "{}",
        style(format!("  Created {} directory", &path.display()))
            .cyan()
            .bold()
    );
}
