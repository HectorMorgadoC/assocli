use console::style;

pub fn clear_terminal() {
    if cfg!(windows) {
        let clear = std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status();

        if clear.is_err() {
            eprintln!("{}", style("Error process clear terminal").red().bold())
        }
    } else {
        let clear = std::process::Command::new("clear").status();

        if clear.is_err() {
            eprintln!("{}", style("Error process clear terminal").red().bold())
        }
    }
}
