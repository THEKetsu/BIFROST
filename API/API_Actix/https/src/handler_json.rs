use actix_web::{web,Result,get};
use serde::Deserialize;
#[derive(Deserialize)]
struct User {
    compagnie:String,
    email: String,
    pwd: String,
}

#[get("/log/{email}/{pwd}/{compagnie}")] 
async fn extract(user:web::Path<User>) -> Result<String> {
    Ok(format!("email:{}, pwd:{}, compagnie:{}!",user.email,user.pwd,user.compagnie)) 
}

/* LOAD A PICTURE IN HTML  */
