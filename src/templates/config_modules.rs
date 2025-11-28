use crate::app::shared::state::state::AppState;
use axum::Router;

pub fn configure(state: std::sync::Arc<AppState>) -> Router {
    tracing::info!("🔧 Configuring application modules...");

    Router::new()
}
