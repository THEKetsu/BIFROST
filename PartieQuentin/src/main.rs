use axum::routing::get;
use axum::handler::Handler;
pub mod handler_file;

// Création d'un routeur pour notre API
 #[tokio::main]
pub async fn main() {
    let API = axum::Router::new()
    .fallback(
        handler_file::fallback.into_service()
    )
    .route("/demo.html",
    get(handler_file::hello_html)
    );


// http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(API.into_make_service())
        .with_graceful_shutdown( handler_file::shutdown_signal())
        .await
        .unwrap();
}