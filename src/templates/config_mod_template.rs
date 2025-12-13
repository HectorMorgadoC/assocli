mod handlers;
mod models;
mod repositories;
mod services;

use super::generic::{
    handlers::handler, repositories::repository::Repository, services::service::Service,
};
use crate::app::shared::state::state::AppState;
use axum::{Router, routing::get};
use tera::Tera;

#[derive(Clone)]
pub(crate) struct BlockState {
    service: std::sync::Arc<Service>,
    template: std::sync::Arc<Tera>,
}

pub fn configure(_state: std::sync::Arc<AppState>) -> Router {
    let repositories: Repository = Repository::new();
    let services: std::sync::Arc<Service> = std::sync::Arc::new(Service::new(repositories));

    let templates = std::sync::Arc::new(_state.templates.tera.clone());

    let blockstate: BlockState = BlockState {
        service: std::sync::Arc::clone(&services),
        template: templates,
    };

    Router::new()
        .route("/", get(handler::get_all))
        .route("/search", get(handler::search_by_id))
        .route("/create", get(handler::create_form).post(handler::create))
        .route("/update", get(handler::update_form).post(handler::update))
        .route("/delete", get(handler::delete))
        .with_state(blockstate)
}
