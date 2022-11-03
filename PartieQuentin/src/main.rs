use axum::routing::get;
use axum::handler::Handler;
use std::include_str;
use serde_json::{json, Value};
use std::collections::HashMap;
use axum::{
    http::StatusCode,
    response::Html,
};
// Crates search parameters


/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
/// Nous indique si la route existe ou pas et nous retourne une erreur 
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

///  Créer une fonction qui fait la gestion de fichier html 
async fn hello_html() -> axum::response::Html<&'static str> {
    include_str!("SiteHtml/web.html").into()
}


 #[tokio::main]
pub async fn main() {
     // Build our application by creating our router.
    let app = axum::Router::new()

    .fallback(
        fallback.into_service()
    )
    .route("/demo.html",
    get(hello_html)
    );


    
    


    




    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}