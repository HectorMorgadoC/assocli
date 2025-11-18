mod app;
use crate::app::config::env::env::Env;
use actix_web::{App, HttpServer};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(true)
        .with_thread_ids(true)
        .init();

    let var_env = Env::init();
    let env_port: u16 = var_env.get_parsed("PORT").unwrap_or_else(|| 3000);
    let env_address: &str = &var_env.get_or("ADDRESS", "127.0.0.1");

    let server = HttpServer::new(move || App::new());
    tracing::info!("󰒋 Server starting on {}:{}", env_address, env_port);
    server.bind((env_address, env_port)).unwrap().run().await
}
