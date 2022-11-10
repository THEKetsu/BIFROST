/* fichier qui nous permet de déterminer les routes que peuvent prendre notre API  */
use axum::routing::get;
use axum::handler::Handler;
pub mod handler_file;
pub mod handler_json;

// Création d'un routeur pour notre API
 #[tokio::main]
pub async fn main() {
    let api = axum::Router::new()
    .fallback(handler_file::routing_error.into_service())
    .route("/API",get(handler_file::sitehtml))
    .route("/",get(handler_file::demo_uri))
    .route("/API/items",get(handler_file::get_items))
    .route("/API/demo.json",get(handler_json::get_demo_json).put(handler_json::put_demo_json))
    .route("/API/replace.json",get(handler_json::collect));

// http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(api.into_make_service())
        .with_graceful_shutdown( handler_file::api_down())
        .await
        .unwrap();
}

