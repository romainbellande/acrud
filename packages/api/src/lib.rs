mod api_doc;
mod auth;
mod config;
mod db;
pub mod fixtures;
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

pub async fn start() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let database = Database::new().await;

    let conn = database.connection;

    Migrator::up(&conn, None)
        .await
        .expect("could not migrate database");

    acrud::tracing::init();

    let api_routes = Router::new()
        .nest("/todos", todo::router())
        .nest("/auth", auth::router());

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
