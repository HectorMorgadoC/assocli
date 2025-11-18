use console::style;
use std::{fs, path::Path};

pub fn create_file(path: &str, content: Option<&str>) {
    let file_path = Path::new(path);

    if file_path.exists() {
        println!(
            "{}",
            style(format!("  {file_path:?} ya existe, omitiendo..."))
                .yellow()
                .bold()
        );
        return;
    }

    if let Some(affair) = content {
        if fs::write(file_path, affair).is_err() {
            eprintln!(
                "{}",
                style(format!("  error al escribir {file_path:?}"))
                    .red()
                    .bold()
            );
            return;
        }
    } else if fs::File::create(file_path).is_err() {
        eprintln!(
            "{}",
            style(format!("  error al crear archivo {file_path:?}"))
                .red()
                .bold()
        );
        return;
    }

    println!(
        "{}",
        style(format!("  Creado: {file_path:?}")).green().bold()
    );
}
