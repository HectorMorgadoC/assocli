use console::style;
use once_cell::sync::Lazy;

// Variable global de solo lectura
pub static PROJECT_PATH: Lazy<Option<std::path::PathBuf>> = Lazy::new(|| {
    let home_path = std::env::var("HOME");

    if let Ok(_path) = home_path {
        let new_path = format!("{_path}/Asso");
        Some(std::path::Path::new(&new_path).to_path_buf())
    } else {
        eprintln!(
            "{}",
            style("  Error in home enviroment variable").red().bold()
        );
        None
    }
});
