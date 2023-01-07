use actix_web::{HttpRequest,get, HttpResponse};
use serde::{Serialize};
use futures::{stream, StreamExt};

     /* Définition */
#[derive(Serialize)]
struct RequestData {
    method: String,
    uri: String,
    body: String,
    headers: Vec<(String, String)>,
}



#[get("/")]
pub async fn extract_http_request(req: HttpRequest) -> HttpResponse {
    /* extraction du body,header,method,url */
    let mut headers = Vec::new();
    for (key, value) in req.headers() {
        headers.push((key.to_string(), value.to_str().unwrap().to_string()));
    }
     /* Init*/
    let request_json = RequestData{
        uri:req.uri().to_string(),
        method:req.method().to_string(),
        body:String::from("NULL"),
        headers:headers
    };
      /* Réponse pour vérifier */
    let mut result = String::new();
    result.push_str(&format!("URL:{},METHOD:{}\n",req.uri(),req.method()));
    for (name, value) in req.headers().iter() {
        result.push_str(&format!("{}: {:?}\n", name, value));
    }
    //println!("{}", result);
    /* REDIRECTION DE LA REQUETE HTTP AVEC UN HEADER D'AUHTENTIFICATION*/
    HttpResponse::Ok()
    .body(result)
    .header("Authorization","Basic kfôzjfgzeôjr");
    
}

