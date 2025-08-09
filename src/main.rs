use axum::{
    Router,
    extract::DefaultBodyLimit,
};
use std::{net::SocketAddr, sync::Arc};
use tera::Tera;
use tower_http::services::ServeDir;
use tracing_subscriber;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

mod routes;
mod models;

const CONTENT_LIMIT_LENGTH: usize = 20 * 1024 * 1024;

pub struct AppState {
    pub tera: Arc<Tera>,
    pub db: PgPool,
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
    println!("{}",database_url);
    
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url).await
    .expect("cant connect to the DB");

    println!("connected to the db");
    tracing_subscriber::fmt::init();

    let tera = Tera::new("templates/**/*.html").expect("Failed to init Tera");
    let shared_tera = Arc::new(tera);

    let app_state = Arc::new(AppState{tera: shared_tera, db: pool});
    
    let app = Router::new()
        .merge(routes::default::routes())
        .merge(routes::images::routes())
        .layer(DefaultBodyLimit::max(CONTENT_LIMIT_LENGTH))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Listening on http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

