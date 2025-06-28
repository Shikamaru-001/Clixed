use axum::{
    Router,
    extract::{DefaultBodyLimit},
};
use std::{net::SocketAddr, sync::Arc};
use tera::{Tera};
use tower_http::services::ServeDir;
use tracing_subscriber;
mod routes;

const CONTENT_LIMIT_LENGTH: usize = 20 * 1024 * 1024;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let tera = Tera::new("templates/**/*.html").expect("Failed to init Tera");
    let shared_tera = Arc::new(tera);

    let app = Router::new()
        .merge(routes::default::routes())
        .merge(routes::images::routes())
        .layer(DefaultBodyLimit::max(CONTENT_LIMIT_LENGTH))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(shared_tera);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

