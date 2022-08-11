mod api_doc;
mod auth;
mod db;
mod fixtures;
mod modules;

use acrud::log::print_request_response;
use api_doc::ApiDoc;
use axum::{middleware, Extension, Router};
use db::Database;
#[cfg(debug_assertions)]
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use modules::todo;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn start() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let database = Database::new().await;

    let conn = database.connection;

    Migrator::up(&conn, None)
        .await
        .expect("could not migrate database");

    let json_format = tracing_subscriber::fmt::format::Format::default().json();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .event_format(json_format);
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=debug".into()),
        ))
        .with(fmt_layer)
        .init();

    let api_routes = Router::new().nest("/todos", todo::router());

    let app = Router::new()
        .nest("/swagger", ApiDoc::router())
        .nest("/api", api_routes)
        .layer(middleware::from_fn(print_request_response))
        .layer(Extension(conn));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
