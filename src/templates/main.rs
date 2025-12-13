mod app;
use crate::app::config::env::env::Env;
use crate::app::shared::state::state::AppState;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true),
        )
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .init();

    let var_env = Env::init();
    let env_port: u16 = var_env.get_parsed("PORT").unwrap_or_else(|| 3000);
    let env_address: &str = &var_env.get_or("ADDRESS", "127.0.0.1");

    let shared_state: std::sync::Arc<AppState> = std::sync::Arc::new(AppState::new());
    let app = Router::new()
        .nest_service("/static", ServeDir::new("templates/static"))
        .merge(app::module::configure(std::sync::Arc::clone(&shared_state)))
        .route_service("/", ServeFile::new("templates/static/index.html"));

    let listener = TcpListener::bind(format!("{env_address}:{env_port}"))
        .await
        .unwrap();

    tracing::info!("󰒋 Server starting on {}:{}", env_address, env_port);
    axum::serve(listener, app).await.unwrap();
}
