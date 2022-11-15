use axum::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}
async fn create_user(payload: extract::Json<CreateUser>) {
    let payload: CreateUser = payload.0;

    // ...
}
