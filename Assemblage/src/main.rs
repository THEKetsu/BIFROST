use std::{io};
use actix_files::{Files, NamedFile};
use serde::{Deserialize};
use askama::Template;
use std::path::{Path, PathBuf};
use actix_web::{
    get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,ResponseError
};


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
    thing_to_show: String,
    thing_to_show2: String,
    thing_to_show3: String
}

#[derive(Deserialize)] // pour adapter la donnée
struct FormData {
    thing_to_show:String,
    thing_to_show2:String,
    thing_to_show3: String
}


async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("templates/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

/* FONCTION EN PLUS DE THOMAS */
async fn showthis(form_data: web::Form<FormData>) -> Result<NamedFile> { //fonction pour afficher le 2éme rendu html
    let html = Show{ thing_to_show: form_data.thing_to_show.to_string(),thing_to_show2: form_data.thing_to_show2.to_string(),thing_to_show3: form_data.thing_to_show3.to_string()}.render().unwrap();
    println!("{}",html);
    let path: PathBuf = "templates/menushowthis.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}


#[get("templates/menu1")]
async fn menu1(req: HttpRequest) -> Result<HttpResponse> {
    println!("{req:?}");
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../templates/menu1.html")))
}

async fn index(data: web::Data<String>) -> HttpResponse {
    let html = format!(r#"
    <html>
        <body>
            <script src="/templates/sha256.js" type="text/javascript"></script>
            <link rel="stylesheet" type="text/css" href="/templates/SignUp.css" />
            <script src="/templates/lottie-player.js" type="text/javascript"></script>
            <script>
                var rust_variable = '{}';
                document.getElementById('rust_variable_value').innerHTML = rust_variable; 
                console.log(rust_variable)
                
            function Securisation() {{
                let str;
                str = document.getElementById("password").value;
                console.log(str.length);
                console.log(str);
                let mdphash = str+rust_variable;
                console.log (mdphash)
                let hash = sha256(mdphash)
		        console.log(hash);
            }}                  
            </script>
            <body class="body">     
      <div class="login-page">
        <div class="form">
		<div class="titre">
		<h1 align="center">SIGNUP BIFROST</h1>
	</div>
          <form>
            <lottie-player
              src="https://assets4.lottiefiles.com/datafiles/XRVoUu3IX4sGWtiC3MPpFnJvZNq7lVWDCa8LSqgS/profile.json"
              background="transparent"
              speed="1"
              style="justify-content: center"
              loop
              autoplay
            ></lottie-player>
            <input id="login" type="text" placeholder="&#xf007; Login" />
            <input id="password" type="password"  placeholder="&#xf023; Password" />          </form>
			
		<a><input id="signup" type="submit" value="SIGN UP"  style="display: none;" ><a>
          <a><input id="test" type="button" onclick="Securisation()"value="Verification du mot de passe"><a>
          <button onclick="window.location.href='templates/menu1.html'">ok</button>
        </div>
      </div>
        </body>
    </html>
"#, data.get_ref());
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(menu1) 
         // static files
        .service(Files::new("/templates", "templates").show_files_listing())
            // redirect
        .service(
                web::resource("/templates/login.html").route(web::get().to(|req: HttpRequest| async move {
                    //println!("{req:?}");
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "templates/login.html"))
                        .finish()
                })),
            )
            // default
        .default_service(web::to(default_handler))
        .route("/showthis", web::post().to(showthis))
        .data("Salt".to_owned()).route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 1010))?
    .workers(2)
    .run()
    .await
}