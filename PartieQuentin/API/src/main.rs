use axum::routing::get;
use axum::handler::Handler;
pub mod handler_file;

// Création d'un routeur pour notre API
 #[tokio::main]
pub async fn main() {
    let api = axum::Router::new()
    .fallback(handler_file::routing_error.into_service())
    .route("/web",get(handler_file::siteHTML));

// http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(api.into_make_service())
        .with_graceful_shutdown( handler_file::api_down())
        .await
        .unwrap();
}