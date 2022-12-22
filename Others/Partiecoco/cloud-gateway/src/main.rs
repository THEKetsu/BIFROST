use actix_web::{HttpServer, App, middleware, web, HttpResponse, Result};
use actix_web_lab::web::redirect;
use actix_web::http::{StatusCode};


pub mod tls;

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = tls::load_rustls_config();

    log::info!("starting HTTPS server at https://localhost:8443");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // redirection vers l'URL renseigner
            .service(redirect("/", "http://127.0.0.2:8080"))
            .default_service(web::route().to(not_found)) //en cas d'errreur

    })
    .bind_rustls("127.0.0.1:8443", config)?
    .run()
    .await
}