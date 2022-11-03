use std::include_str;
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
/// Nous indique si la route existe ou pas et nous retourne une erreur si il ne trouve pas 
/// Parameter Validation #1 
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("ErrorDocument 404 not found {}", uri))
}

///  Créer une fonction qui fait la gestion de fichier html 
pub async fn hello_html() -> axum::response::Html<&'static str> {
    include_str!("SiteHtml/web.html").into()
}
