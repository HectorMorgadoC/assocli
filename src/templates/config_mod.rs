mod handlers;
mod models;
mod repositories;
mod services;

use super::generic::{
    handlers::handler, repositories::repository::Repository, services::service::Service,
};
use crate::app::shared::state::state::AppState;
use axum::{Router, routing::get};

pub fn configure(state: std::sync::Arc<AppState>) -> Router {
    let repositories: Repository = Repository::new();
    let services: std::sync::Arc<Service> = std::sync::Arc::new(Service::new(repositories));

    Router::new()
        .route("/", get(handler::get_all).post(handler::create))
        .route(
            "/{id}",
            get(handler::get_by_id)
                .patch(handler::update)
                .delete(handler::delete),
        )
        .with_state(services)
}
