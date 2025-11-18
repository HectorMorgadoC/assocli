pub mod progress_bar {
    use console::style;
    use indicatif::{ProgressBar, ProgressStyle};

    pub fn start_progress(progress_bar: ProgressBar, message: String) {
        progress_bar.set_style(
            ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos:>3}% {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        progress_bar.set_message(message);
    }

    pub fn progressing(progress_bar: ProgressBar, mut child: std::process::Child) {
        let mut progress = 0;
        while let Ok(None) = child.try_wait() {
            progress = (progress + 1).min(100);
            progress_bar.set_position(progress);
            std::thread::sleep(std::time::Duration::from_millis(80));
        }
    }

    pub fn progress_message_finish(progress_bar: ProgressBar, output: std::process::Output) {
        if output.status.success() {
            progress_bar.finish_with_message("\nProgress completed....");
            println!(
                "{}",
                style("  Compilation completed successfully")
                    .on_green()
                    .bold()
            )
        } else {
            progress_bar.finish_with_message("\nProgress fault....");
            std::thread::sleep(std::time::Duration::from_secs(2));
            eprintln!("{}", style("  Error during compilation.").red().bold());
            std::process::exit(1)
        }
    }
}
