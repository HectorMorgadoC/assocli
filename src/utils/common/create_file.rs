use console::style;
use std::{fs, path::PathBuf};

pub fn create_file(path: PathBuf, content: Option<&str>) {
    /*
        if path.exists() {
            println!(
                "{}",
                style(format!(
                    "  {} It already exists, omitting....",
                    &path.display()
                ))
                .yellow()
                .bold()
            );
            std::process::exit(1)
        }
    */
    if let Some(affair) = content {
        if fs::write(&path, affair).is_err() {
            eprintln!(
                "{}",
                style(format!("  Error when writing {}", &path.display()))
                    .red()
                    .bold()
            );
            std::process::exit(1)
        }
    } else if fs::File::create(&path).is_err() {
        eprintln!(
            "{}",
            style(format!("  Error creating file {}", &path.display()))
                .red()
                .bold()
        );
        std::process::exit(1)
    }

    println!("{}", style(format!("  Created: {path:?}")).green().bold());
}
