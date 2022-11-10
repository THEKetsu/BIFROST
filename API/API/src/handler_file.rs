/* fichier qui fait la gestion de requete HTTP comme GET */

use std::include_str;
use std::collections::HashMap;

/// We use this in our hyper `Server` method `with_graceful_shutdown`.
pub async fn api_down() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
/// Nous indique si la route existe ou pas et nous retourne une erreur si il ne trouve pas 
/// Parameter Validation #1 
pub async fn routing_error(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("ErrorDocument 404 not found {}", uri))
}

///  Créer une fonction qui fait la gestion de fichier html 
pub async fn sitehtml() -> axum::response::Html<&'static str> {
    include_str!("SiteHtml/web.html").into()
}

//Fonction qui retourn un url
pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

/// axum handler for "GET /items/:id" which uses `axum::extract::Path`.
/// This extracts a path parameter then deserializes it as needed.
pub async fn get_items_id(axum::extract::Path(id):axum::extract::Path<String>) -> String {
    format!("Get items with path id: {:?}", id)
}

/// axum handler for "GET /items" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub async fn get_items(axum::extract::Query(params):axum::extract::Query<HashMap<String, String>>) -> String {
    format!("Get items with query params: {:?}", params)
}