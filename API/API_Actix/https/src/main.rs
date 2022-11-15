use actix_files as fs;
use actix_web::{
    middleware, web, App, HttpServer,
};
/* Tous les fichiers lib qu'on a besoin  */
pub mod tlsconfig;
pub mod handler;
pub mod handler_json;
pub mod static_file;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let config = tlsconfig::load_rustls_config(); //Load TLS 
    log::info!("starting HTTPS server at https://localhost:8443");
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/website/img", ".").show_files_listing())
            .service(handler_json::extract)
            .route("/", web::get().to(static_file::load))
            .route("/log", web::get().to(static_file::load2))
        
            .service(web::resource("/index.html").to(handler::index))
    })
    .bind_rustls("127.0.0.1:8443", config)? // check if the config is good
    .run()
    .await
}
