pub mod utils_command {

    use crate::utils::progress_bar::progress::*;

    use super::super::file::file_management::copy_template;
    use console::style;
    use indicatif::ProgressBar;
    use std::{
        env, fs,
        io::Write,
        path::{Path, PathBuf},
        process::{Command, Stdio},
    };

    pub fn find_cargo_root() -> Option<PathBuf> {
        let path = path_home().unwrap();
        let path_str = path.as_str();
        let base_path = Path::new(path_str);

        let mut current = env::current_dir().unwrap();
        //let mut current = from_path.to_path_buf();

        loop {
            let cargo_toml = current.join("Cargo.toml");

            if cargo_toml.exists() {
                return Some(current);
            }

            // Si llegamos a Asso, detenemos la búsqueda
            if current == base_path {
                break;
            }

            // Si no se puede subir más (ya estamos en `/`), salir
            if !current.pop() {
                break;
            }
        }

        None
    }

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

    fn add_dependency(arg: &str, features: Option<&str>, path: &str) {
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
                    style(format!("  {arg} agregado correctamente!"))
                        .green()
                        .bold()
                );
            }
            Ok(_) => {
                eprintln!(
                    "{}",
                    style(format!(
                        "   Falló la instalación de {arg}. ¿Tienes instalado 'cargo-edit'?"
                    ))
                    .yellow()
                    .bold()
                );
            }
            Err(err) => {
                eprintln!(
                    "{}",
                    style(format!("  Error al ejecutar 'cargo add {arg}': {err}",))
                        .red()
                        .bold()
                );
            }
        }
    }

    pub fn run_cargo_command(arg: &str, optional_arg: Option<&str>, path: PathBuf) {
        let cargo_toml = path.join("Cargo.toml");
        let progress = ProgressBar::new(100);
        progress_bar::start_progress(progress.clone(), "Compilando proyecto...".to_string());

        // 1️⃣ Verificar si el proyecto es un proyecto Cargo válido
        if !cargo_toml.exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  No se encontró un Cargo.toml en '{path:?}'. No parece un proyecto de Cargo.",
                ))
                .red()
                .bold()
            );
            return;
        }

        // 2️⃣ Ejecutar el comando Cargo solicitado
        let mut output = Command::new("cargo");

        output.arg(arg);

        if let Some(arg) = optional_arg {
            output.arg(arg);
        }

        let child_result = output.current_dir(path).stdout(Stdio::null()).spawn();

        if let Ok(child) = child_result {
            progress_bar::progressing(progress.clone(), child);

            let status = output.output().unwrap();

            progress_bar::progress_message_finish(progress, status.clone());
            if !status.status.success() {
                println!("{}", String::from_utf8_lossy(&status.stderr));
            }
        }
    }

    pub fn lift_release_service(path: PathBuf) {
        // 1️⃣ Verificar que Cargo.toml exista
        let cargo_toml_path = path.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            eprintln!(
                "{}",
                style(format!("  No se encontró Cargo.toml en {path:?}")).red()
            );
            return;
        }

        // 2️⃣ Leer el contenido de Cargo.toml y extraer el nombre del proyecto
        let cargo_toml_content =
            fs::read_to_string(&cargo_toml_path).expect("No se pudo leer el archivo Cargo.toml");

        let project_name = cargo_toml_content
            .lines()
            .find(|line| line.trim_start().starts_with("name"))
            .and_then(|line| line.split('=').nth(1))
            .map(|name| name.trim().trim_matches('"').to_string());

        let Some(project_name) = project_name else {
            eprintln!(
                "{}",
                style("  No se pudo determinar el nombre del proyecto desde Cargo.toml").red()
            );
            return;
        };

        println!(
            "{}",
            style(format!(
                "󰍦  Iniciando servicio del proyecto: {project_name}"
            ))
            .blue()
        );

        // 3️⃣ Construir la ruta al binario
        let binary_path = path.join("target").join("release").join(&project_name);

        if !binary_path.exists() {
            eprintln!("{}",style(format!(
            "  El binario no existe en {:?}. Asegúrate de compilar con `cargo build --release` primero.",
            binary_path
        )).red());
            return;
        }

        // 4️⃣ Ejecutar el binario
        let mut child = Command::new(&binary_path)
            .current_dir(&path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("No se pudo ejecutar el binario");

        println!(
            "{}",
            style(format!(
                "  Servicio '{project_name}' en ejecución (Ctrl+C para detener)"
            ))
            .cyan()
        );

        // 5️⃣ Esperar a que el proceso termine
        let status = child.wait().expect("Error al esperar el proceso");
        println!("🔚 Proceso finalizado con estado: {}", status);
    }

    pub fn path_home() -> Option<String> {
        let home_path = env::var("HOME");
        let mut destination = String::new();

        if let Ok(path) = home_path {
            let new_path = format!("{path}/Asso");
            destination.push_str(&new_path);
            Some(destination)
        } else {
            eprintln!(
                "{}",
                style("  ERROR IN HOME ENVIRONMENT VARIABLE").red().bold()
            );
            None
        }
    }

    pub fn cargo_new(name: &str) -> Option<String> {
        /*
        let home_path = env::var("HOME");
        let mut destination = String::new();

        if let Ok(path) = home_path {
            let new_path = format!("{path}/Asso");
            destination.push_str(&new_path);
        } else {
            eprintln!(
                "{}",
                style("  ERROR IN HOME ENVIRONMENT VARIABLE").red().bold()
            );
            return None;
        }
        */

        let destination = path_home().unwrap();
        if !Path::new(&destination).exists() {
            println!("📁 Carpeta 'proyectos' no encontrada. Creándola...");

            if fs::create_dir_all(&destination).is_err() {
                eprintln!(
                    "{}",
                    style("  ERROR CREATING PROJECT DIRECTORY").red().bold()
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
            eprintln!("{}", style("  ERROR EXECUTING CARGO NEW").red().bold());
            None
        }
    }

    pub fn create_actix(project_path: &str) {
        let cargo_toml_path = Path::new(project_path).join("Cargo.toml");

        if !cargo_toml_path.exists() {
            eprintln!(
                "{}",
                style(format!("  'Cargo.toml' WAS NOT FOUND IN '{project_path}'",))
                    .red()
                    .bold()
            );
            return;
        }

        println!(
            "{}",
            style("🔍 Verificando proyecto Cargo...").cyan().bold()
        );

        println!(
            "{}",
            style("  Agregando 'actix-web' al proyecto...")
                .blue()
                .bold()
        );

        add_dependency("actix-web", None, project_path);
        add_dependency("dotenvy", None, project_path);
        add_dependency("tracing", None, project_path);
        add_dependency(
            "tracing-subscriber",
            Some("env-filter,fmt,ansi"),
            project_path,
        );
    }

    pub fn create_app_structure(project_path: &str) {
        let src_path = Path::new(project_path).join("src");
        let app_path = src_path.join("app");

        if !src_path.exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  No se encontró el directorio src en '{project_path}'",
                ))
                .red()
                .bold()
            );
            return;
        }

        let subdirs = ["config", "module", "shared"];
        for dir in &subdirs {
            let path = app_path.join(dir);
            let mod_rs_path = path.join("mod.rs");
            let mod_rs_path_str = mod_rs_path.to_str().unwrap();

            if !path.exists() {
                if fs::create_dir_all(&path).is_err() {
                    eprintln!(
                        "{}",
                        style("  ERROR CREATING PROJECT SUBDIRECTORY").red().bold()
                    );
                    return;
                }
                println!(
                    "{}",
                    style(format!("  Creado: {}", path.display()))
                        .green()
                        .bold()
                );
            } else {
                println!(
                    "{}",
                    style(format!("  Ya existe: {}", path.display()))
                        .yellow()
                        .bold()
                );
            }

            create_file(mod_rs_path_str, None);
        }

        let mod_file_path = app_path.join("mod.rs");
        let mod_content = "pub mod module;\npub mod shared;\npub mod config;\n";

        let file = fs::File::create(&mod_file_path);
        if let Ok(mut file_created) = file {
            if file_created.write_all(mod_content.as_bytes()).is_err() {
                println!("{}", style("  No se pudo escribir en mod.rs").red().bold());
                return;
            }
        } else {
            println!(
                "{}",
                style("  No se pudo crear el archivo mod.rs").red().bold()
            );
            return;
        }

        println!(
            "{}",
            style(format!(
                "  Estructura 'app' creada correctamente en '{}'",
                app_path.display()
            ))
            .cyan()
            .bold()
        );
    }

    pub fn create_env_file(project_path: &str) {
        let env_path = Path::new(project_path).join(".env");

        if env_path.exists() {
            println!(
                "{}",
                style("  .env ya existe, omitiendo...").yellow().bold()
            );
            return;
        }

        let content = r#"ADDRESS="127.0.0.1"
        PORT=3000
        "#;

        if fs::write(&env_path, content).is_err() {
            eprintln!(
                "{}",
                style("  error al escribir variables de entorno")
                    .red()
                    .bold()
            );
            return;
        }

        println!(
            "{}",
            style(format!("  Archivo .env creado en {env_path:?}"))
                .green()
                .bold()
        );
    }

    pub fn create_env_rs(project_path: &str) {
        let config_dir = Path::new(project_path).join("src/app/config");
        let env_rs_path = config_dir.join("env.rs");
        let mod_rs_path = config_dir.join("mod.rs");

        if !config_dir.exists() {
            eprintln!(
                "{}",
                style(format!(
                    "  No se encontró la carpeta config en {config_dir:?}"
                ))
                .red()
                .bold()
            );
            return;
        }

        let content = "\npub mod env;";

        if mod_rs_path.exists() {
            if fs::write(&mod_rs_path, content).is_err() {
                eprintln!(
                    "{}",
                    style(format!("  error al escribir {mod_rs_path:?}"))
                        .red()
                        .bold()
                );
                return;
            }
        } else {
            let mod_rs_path_str = mod_rs_path.to_str().unwrap();
            create_file(mod_rs_path_str, Some(content));
        }

        let template = copy_template("env.rs", &env_rs_path);

        if template.is_err() {
            eprintln!(
                "{}",
                style("  Error de al cargar la plantilla").red().bold()
            );
        }
    }

    pub fn create_main_rs(project_path: &str) {
        let main_path = Path::new(project_path).join("src/main.rs");

        if main_path.exists() {
            let _ = fs::remove_file(&main_path);
            println!(
                "{}",
                style("  main.rs ya existe, omitiendo...").yellow().bold()
            );
        }

        let template = copy_template("main.rs", &main_path);

        if template.is_err() {
            eprintln!(
                "{}",
                style("  Error de al cargar la plantilla").red().bold()
            );
        }

        println!(
            "{}",
            style("  main.rs creado correctamente").green().bold()
        );
    }
}

fn compile_release() {}
