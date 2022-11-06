use axum::routing::get;
use axum::handler::Handler;

use axum::{
    response::Html,
    http::StatusCode,
};

pub async fn hello() -> String {
    "Hello, World!".into()
 }

 pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

// si j'ajoute /demo.html a l'url il y a marqué Hello
pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}

//Pour appeler le code html
async fn hello_html() -> axum::response::Html<&'static str> {
    include_str!("hello.html").into()
}

//http://localhost:3000/demo-status 
pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}

//http://localhost:3000/demo-uri
pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}


/// axum handler for "GET /foo" which returns a string message.
/// This shows our naming convention for HTTP GET handlers.
pub async fn get_foo() -> String {
    "GET foo".to_string()
 }
 
 /// axum handler for "PUT /foo" which returns a string message.
 /// This shows our naming convention for HTTP PUT handlers.
 pub async fn put_foo() -> String {
    "PUT foo".to_string()
 }
 
 /// axum handler for "PATCH /foo" which returns a string message.
 /// This shows our naming convention for HTTP PATCH handlers.
 pub async fn patch_foo() -> String {
    "PATCH foo".to_string()
 }
 
 /// axum handler for "POST /foo" which returns a string message.
 /// This shows our naming convention for HTTP POST handlers.
 pub async fn post_foo() -> String {
    "POST foo".to_string()
 }
 
 /// axum handler for "DELETE /foo" which returns a string message.
 /// This shows our naming convention for HTTP DELETE handlers.
 pub async fn delete_foo() -> String {
    "DELETE foo".to_string()
 }
 
 pub async fn get_items_id(
    axum::extract::Path(id):
        axum::extract::Path<String>
) -> String {
    format!("Get items with path id: {:?}", id)
}

#[tokio::main]
pub async fn main() {
     // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(
            fallback.into_service()
        )
        .route("/",
            get(hello)
        )
        .route("/demo.html",
            get(get_demo_html)
        )
        .route("/hello.html",
        get(hello_html)
        )
        .route("/demo-status",
        get(demo_status)
        )
        .route("/demo-uri",
        get(demo_uri)
        )
        .route("/foo",
        get(get_foo)
        .put(put_foo)
        .patch(patch_foo)
        .post(post_foo)
        .delete(delete_foo),
        )
        .route("/items/:id",
        get(get_items_id)
        );

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}