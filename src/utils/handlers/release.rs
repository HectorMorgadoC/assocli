use crate::utils::others::utils_command::*;
use console::style;

pub fn handler_release() {
    let path_cargo_toml = find_cargo_root();

    if let Some(path) = path_cargo_toml {
        run_cargo_command("build", Some("--release"), path.clone());
        println!("{}", style("  Compilado exitoso").green().bold());
        lift_release_service(path);
    } else {
        println!("{}", style("  Compilado fallido").red().bold());
    }
}
