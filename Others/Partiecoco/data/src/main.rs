use actix_web::{web, App, HttpResponse, HttpServer, ResponseError, Responder};
use serde::{Deserialize};
use askama::Template;

#[derive(Debug)] // Macro qui implémente l'erreur
pub struct MyError(String); // <-- needs debug and display

 impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input.")
    }
} 

impl ResponseError for MyError {} // <-- key // je crée l'instance erreur avec la macro du dessus

#[derive(Template)] //déclaration de la 1ére template html (le formulaire)
#[template(path = "index.html")]
struct Index {}

#[derive(Template)] // déclaration de la 2éme template html (affichage de la variable)
#[template(path = "show.html")]
struct Show {
    thing_to_show: String
}

#[derive(Deserialize)] // pour adapter la donnée
struct FormData {
    thing_to_show: String,
}

async fn index() -> Result<HttpResponse, MyError>  { //fonction pour afficher le premier rendu html
    let html = Index{}.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn showthis(form_data: web::Form<FormData>) -> Result<HttpResponse, MyError>  { //fonction pour afficher le 2éme rendu html
    let html = Show{ thing_to_show: form_data.thing_to_show.to_string() }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/showthis", web::post().to(showthis))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
