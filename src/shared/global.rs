use console::style;
use once_cell::sync::Lazy;
use std::{
    env,
    path::{Path, PathBuf},
};

// Variable global de solo lectura
pub static PROJECT_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
    let home_path = env::var("HOME");

    let mut path: Option<PathBuf> = None;

    if let Ok(_path) = home_path {
        let new_path = format!("{_path}/Asso");
        path = Some(Path::new(&new_path).to_path_buf());
        path
    } else {
        eprintln!(
            "{}",
            style("  ERROR IN HOME ENVIRONMENT VARIABLE").red().bold()
        );
        None
    }
});
