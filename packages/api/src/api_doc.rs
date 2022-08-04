use hyper::StatusCode;
use utoipa::{OpenApi, Modify, openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}};
use entity::todo::{Model as TodoModel, CreateTodo};
use utoipa_swagger_ui::Config;
use std::sync::Arc;
use axum::{Router, routing, Json, extract::Path, Extension, response::IntoResponse};
use acrud::errors::WebError;
use crate::modules::todo;

#[derive(OpenApi)]
#[openapi(
    handlers(
        todo::find,
        todo::create
    ),
    components(TodoModel, CreateTodo, WebError),
    modifiers(&SecurityAddon),
    tags(
        (name = "todo", description = "Todo items management API")
    )
)]
pub struct ApiDoc;


struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("apikey"))),
            )
        }
    }
}

impl ApiDoc {
    pub fn router() -> Router {
        let api_doc = Self::openapi();
        let config = Arc::new(Config::from("/swagger/openapi.json"));

        Router::new()
            .route("/openapi.json", routing::get({
                let doc = api_doc.clone();
                move || async { Json(doc) }
            }))
            .route("/ui/*tail", routing::get(serve_swagger_ui).layer(Extension(config)))
    }
}

async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}