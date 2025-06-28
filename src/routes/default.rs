use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use tera::{Context, Tera};

pub fn routes() -> Router<Arc<Tera>> {
    Router::new().route("/", get(homepage))
}
// 7. Fixed state extraction using proper State extractor
async fn homepage(State(tera): State<Arc<Tera>>) -> impl IntoResponse {
    let ctx = Context::new();
    // 8. Added proper error handling for template rendering
    match tera.render("index.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            Html(format!("Template error: {}", e).into())
        }
    }
}
