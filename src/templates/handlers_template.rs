pub(crate) mod handler {

    use super::super::{BlockState, models::model::dto::CreateUpdateGENERIC};
    use crate::app::shared::common::error::err_render;

    use axum::{
        Form,
        extract::{
            Query, State,
            rejection::{FormRejection, QueryRejection},
        },
        http::StatusCode,
        response::{Html, IntoResponse},
    };
    use serde::Deserialize;
    use tera::{Context, Tera};

    #[derive(Deserialize, Debug)]
    pub(crate) struct Params {
        id: Option<i32>,
        #[serde(default)]
        confirm: Option<String>,
    }

    pub async fn get_all(State(state): State<BlockState>) -> impl IntoResponse {
        let mut context = Context::new();

        match state.service.get().await {
            Ok(items) => {
                context.insert("items", &items);
            }
            Err(err) => {
                return err_render(
                    std::sync::Arc::clone(&state.template),
                    err.status,
                    err.detail,
                )
                .into_response();
            }
        };

        render_template(state.template, "generic/index.html", context).into_response()
    }

    pub async fn search_by_id(
        query: Result<Query<Params>, QueryRejection>,
        State(state): State<BlockState>,
    ) -> impl IntoResponse {
        let mut context = Context::new();
        let state_template = std::sync::Arc::clone(&state.template);

        let params = match query {
            Ok(Query(p)) => p,
            Err(err) => {
                return err_render(state_template, err.status(), err.body_text()).into_response();
            }
        };

        if let Some(id) = params.id {
            context.insert("search_id", &id);

            match state.service.get_by_id(id).await {
                Ok(item) => {
                    if let Some(value) = item {
                        context.insert("item", &value);
                    } else {
                        context.insert("error", &true);
                    }
                }
                Err(err) => {
                    if err.status == 404 {
                        return err_render(state_template, err.status, err.detail).into_response();
                    } else {
                        return err_render(state_template, err.status, "").into_response();
                    }
                }
            }
        }

        render_template(state.template, "generic/search.html", context).into_response()
    }

    pub async fn create_form(State(state): State<BlockState>) -> impl IntoResponse {
        let context = Context::new();

        render_template(state.template, "generic/create.html", context).into_response()
    }

    pub async fn create(
        State(state): State<BlockState>,
        form: Result<Form<CreateUpdateGENERIC>, FormRejection>,
    ) -> impl IntoResponse {
        let mut context = Context::new();

        let data = match form {
            Ok(Form(f)) => f,
            Err(err) => {
                return err_render(state.template.clone(), err.status(), err.body_text())
                    .into_response();
            }
        };

        if data.id.is_none() && data.name.is_none() {
            return render_template(
                state.template.clone(),
                "generic/create.html",
                context.clone(),
            )
            .into_response();
        };

        let new_item = CreateUpdateGENERIC {
            id: Some(data.id.unwrap_or(0)),
            name: Some(data.name.unwrap_or_default()),
            state: Some(data.state.unwrap_or("off".to_string())), // true si el checkbox está marcado
        };

        match state.service.create(new_item).await {
            Ok(_) => {
                context.insert("success", &true);
                render_template(
                    state.template.clone(),
                    "generic/create.html",
                    context.clone(),
                )
                .into_response()
            }
            Err(err) => err_render(state.template, err.status, err.detail).into_response(),
        }
    }

    pub async fn update_form(
        query: Result<Query<Params>, QueryRejection>,
        State(state): State<BlockState>,
    ) -> impl IntoResponse {
        let mut context = Context::new();

        let query_params = match query {
            Ok(Query(value)) => value,
            Err(err) => {
                return err_render(state.template.clone(), err.status(), err.body_text())
                    .into_response();
            }
        };

        if let Some(id) = query_params.id {
            match state.service.get_by_id(id).await {
                Ok(item) => {
                    if let Some(value) = item {
                        context.insert("item", &value);
                    } else {
                        context.insert("error", &true);
                    }
                }
                Err(err) => {
                    return err_render(state.template.clone(), err.status, err.detail)
                        .into_response();
                }
            }
        }

        render_template(state.template, "generic/update.html", context).into_response()
    }

    pub async fn update(
        State(state): State<BlockState>,
        Form(data): Form<CreateUpdateGENERIC>,
    ) -> impl IntoResponse {
        let mut context = Context::new();

        // Si hay ID en query, buscar y mostrar form
        let item_id = data.id.unwrap_or(0);

        let updated_item = CreateUpdateGENERIC {
            id: Some(item_id),
            name: Some(data.name.unwrap_or_default()),
            state: Some(data.state.unwrap_or("off".to_string())),
        };

        if state.service.update(updated_item, item_id).await.is_ok() {
            context.insert("success", &true);
            if let Ok(Some(item)) = state.service.get_by_id(item_id).await {
                context.insert("item", &item);
            }
        } else {
            return err_render(
                state.template.clone(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "",
            )
            .into_response();
        }

        render_template(state.template, "generic/update.html", context).into_response()
    }

    pub async fn delete(
        Query(params): Query<Params>,
        State(state): State<BlockState>,
    ) -> impl IntoResponse {
        let mut context = Context::new();
        let id = params.id;

        if id.is_none() {
            return render_template(state.template.clone(), "generic/delete.html", context)
                .into_response();
        };

        if let Some(value) = id {
            if params.confirm.is_some() {
                match state.service.delete(value).await {
                    Ok(_) => {
                        context.insert("success", &true);
                    }
                    Err(err) => {
                        return err_render(state.template.clone(), err.status, err.detail)
                            .into_response();
                    }
                }
            } else {
                match state.service.get_by_id(value).await {
                    Ok(Some(item)) => {
                        context.insert("item", &item);
                    }
                    Ok(None) => {
                        return err_render(
                            state.template.clone(),
                            StatusCode::NOT_FOUND,
                            "Not Found id",
                        )
                        .into_response();
                    }
                    Err(err) => {
                        return err_render(state.template.clone(), err.status, err.detail)
                            .into_response();
                    }
                }
            }
        }
        render_template(state.template, "generic/delete.html", context).into_response()
    }

    fn render_template(
        state: std::sync::Arc<Tera>,
        name_template: &str,
        context: Context,
    ) -> (StatusCode, Html<String>) {
        match state.render(name_template, &context) {
            Ok(value) => (StatusCode::OK, Html(value.to_string())),
            Err(err) => {
                tracing::error!(
                    "Error rendering template: {}: {}",
                    name_template,
                    err.to_string()
                );
                err_render(
                    std::sync::Arc::clone(&state),
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "",
                )
            }
        }
    }
}
