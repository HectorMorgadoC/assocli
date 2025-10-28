pub mod command_model {

    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(
        name = "asso",
        version,
        about = "AssoCLI - Crea proyectos modulares en Rust"
    )]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Commands,
    }

    #[derive(Subcommand)]
    pub enum Commands {
        New { name: Option<String> },
        Release,
        Info,
    }
}
