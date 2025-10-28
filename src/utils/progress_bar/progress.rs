pub mod progress_bar {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::{
        process::{Child, Output},
        time::Duration,
    };

    pub fn start_progress(progress_bar: ProgressBar, message: String) {
        progress_bar.set_style(
            ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos:>3}% {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        progress_bar.set_message(message);
    }

    pub fn progressing(progress_bar: ProgressBar, mut child: Child) {
        let mut progress = 0;
        while let Ok(None) = child.try_wait() {
            progress = (progress + 1).min(100);
            progress_bar.set_position(progress);
            std::thread::sleep(Duration::from_millis(80));
        }
    }

    pub fn progress_message_finish(progress_bar: ProgressBar, output: Output) {
        if output.status.success() {
            progress_bar.finish_with_message("  Compilación completada con éxito");
        } else {
            progress_bar.finish_with_message("  Error durante la compilación");
        }
    }
}
