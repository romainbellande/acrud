use crate::modules::todo;
use crate::auth::{self, credentials::Credentials, body::AuthBody};
use acrud::errors::WebError;
use axum::{extract::Path, response::IntoResponse, routing, Extension, Json, Router};
use entity::todo::{CreateTodo, Model as TodoModel};
use hyper::StatusCode as HyperStatusCode;
use std::sync::Arc;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi, Component
};
use utoipa_swagger_ui::Config;

#[derive(Component)]
pub struct StatusCode(u8);

#[derive(OpenApi)]
#[openapi(
    handlers(
        todo::find,
        todo::create,
        auth::authorize
    ),
    components(TodoModel, CreateTodo, WebError, Credentials, AuthBody, StatusCode),
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
            .route(
                "/openapi.json",
                routing::get({
                    let doc = api_doc;
                    move || async { Json(doc) }
                }),
            )
            .route(
                "/ui/*tail",
                routing::get(serve_swagger_ui).layer(Extension(config)),
            )
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
                    HyperStatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| HyperStatusCode::NOT_FOUND.into_response()),
        Err(error) => (HyperStatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
