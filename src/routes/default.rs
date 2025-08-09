use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use tera::Context;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(homepage))
        .route("/about", get(about))
        .route("/settings", get(settings_page))
        .route("/contact", get(contact_page))
}

async fn about(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let ctx = Context::new();
    match state.tera.render("about.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("About.Html Template error: {}", e);
            Html(format!("About Template error: {}", e).into())
        }
    }
}

async fn homepage(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let ctx = Context::new();
    match state.tera.render("home.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            Html(format!("Template error: {}", e).into())
        }
    }
}

async fn settings_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let ctx = Context::new();
    match state.tera.render("settings.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            Html(format!("Template error: {}", e).into())
        }
    }
}   

async fn contact_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let ctx = Context::new();
    match state.tera.render("contact.html", &ctx) {
        Ok(rendered) => Html(rendered),
        Err(e) => {
            tracing::error!("Contact.Html Template error: {}", e);
            Html(format!("Contact Template error: {}", e).into())
        }
    }
}
