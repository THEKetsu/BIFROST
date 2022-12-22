use actix_web::{
    middleware, web, App, HttpServer
};
use actix_web_lab::web as web_lab;

/* Tous les fichiers lib qu'on a besoin  */
pub mod tlsconfig;
pub mod static_file;
pub mod extractionhttp;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let config = tlsconfig::load_rustls_config(); //Load TLS 
    log::info!("starting HTTPS server at https://127.0.0.1:8000");
    HttpServer::new(|| {
        App::new()
            .service(extractionhttp::extract_http_request)
            .default_service(web::route().to(static_file::not_found))
            .wrap(middleware::Logger::default())
    })
    .bind_rustls("127.0.0.1:8000", config)? // check if the config is good
    .run()
    .await
}
