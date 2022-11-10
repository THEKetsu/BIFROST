use serde_json::{json, Value};
use std::include_str;
pub async fn get_demo_json() -> axum::response::Json<&'static str>{
    include_str!("id.json").into()
}
/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(axum::extract::Json(data): axum::extract::Json<serde_json::Value>) -> String{
    format!("Put demo JSON data: {:?}", data)
}