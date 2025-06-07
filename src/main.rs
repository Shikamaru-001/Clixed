use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router
};

use std::{net::SocketAddr, sync::Arc};
use tera:: {Context,Tera};
use tower_http::services::ServeDir;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let tera = Tera::new("templates/**/*.html").expect("Failed to init Tera");
    let shared_tera = Arc::new(tera);

    let app = Router::new()
        .route("/", get(homepage))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(shared_tera);

    let  addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("Listining on http://{}", addr);


    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn homepage(tera: axum::extract::State<Arc<Tera>>) -> impl IntoResponse{
    let ctx = Context::new();
    let rendered = tera.render("index.html",&ctx).unwrap();
    Html(rendered)
}
