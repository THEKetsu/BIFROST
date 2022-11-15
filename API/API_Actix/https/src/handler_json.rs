use actix_web::{web,Result,get};
use serde_json::{Number, Value};
#[get("/log/{user_id}/{friend}")] // <- define path parameters
async fn extract(path: web::Path<(String, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id)) 
}