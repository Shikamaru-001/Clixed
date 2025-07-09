use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use tera::{Context, Tera};

pub fn routes() -> Router<Arc<Tera>> {
    Router::new()
        .route("/", get(homepage))
        .route("/about", get(about))
}

async fn about(State(tera): State<Arc<Tera>>) -> impl IntoResponse {
    let ctx = Context::new();
    match tera.render("about.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("About.Html Template error: {}", e);
            Html(format!("About Template error: {}", e).into())
        }
    }
}
// 7. Fixed state extraction using proper State extractor
async fn homepage(State(tera): State<Arc<Tera>>) -> impl IntoResponse {
    let ctx = Context::new();
    // 8. Added proper error handling for template rendering
    match tera.render("home.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            Html(format!("Template error: {}", e).into())
        }
    }
}
