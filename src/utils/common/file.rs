pub mod file_management {
    use std::{fs, path::Path};

    pub fn copy_template(from: &str, to: &Path) -> std::io::Result<()> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let ruta_raiz = Path::new(manifest_dir);
        let new_path = format!("src/templates/{from}");
        let from_path = ruta_raiz.join(new_path);
        fs::copy(from_path, to)?;
        Ok(())
    }
}
