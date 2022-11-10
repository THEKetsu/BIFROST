use axum::routing::get;
use axum::handler::Handler;
use std::collections::HashMap;

use axum::{
    response::Html,
    http::StatusCode,
};

/// Use Serde JSON to serialize/deserialize JSON, such as in a request.
/// axum creates JSON or extracts it by using `axum::extract::Json`.
/// For this demo, see functions `get_demo_json` and `post_demo_json`.
use serde_json::{json, Value};

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World!".into()
 }

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

/// axum handler for "GET /demo.html" which responds with HTML text.
/// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}

/// axum handler that responds with typical HTML coming from a file.
/// This uses the Rust macro `std::include_str` to include a UTF-8 file
/// path, relative to `main.rs`, as a `&'static str` at compile time.
async fn hello_html() -> axum::response::Html<&'static str> {
    include_str!("hello.html").into()
}

/// axum handler for "GET /demo-status" which returns a HTTP status
/// code, such as OK (200), and a custom user-visible string message.
pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}

/// axum handler for "GET /demo-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
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

/// axum handler for "GET /items/:id" which uses `axum::extract::Path`.
/// This extracts a path parameter then deserializes it as needed.
pub async fn get_items_id(
    axum::extract::Path(id):
        axum::extract::Path<String>
) -> String {
    format!("Get items with path id: {:?}", id)
}

/// axum handler for "GET /items" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub async fn get_items(
    axum::extract::Query(params):
        axum::extract::Query<HashMap<String, String>>
) -> String {
    format!("Get items with query params: {:?}", params)
}

/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it bu using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a":"b"}).into()
}

/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>
) -> String{
    format!("Put demo JSON data: {:?}", data)
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
    )
    .route("/items",
    get(get_items)
    )
    .route("/demo.json",
        get(get_demo_json)
    )
    .route("/demo.json",
    get(get_demo_json)
    .put(put_demo_json)
    );


    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
