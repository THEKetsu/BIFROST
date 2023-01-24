use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::Bytes;
use awc::ws;
use futures_util::{sink::SinkExt as _, stream::StreamExt as _};
use tokio::{select, sync::mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use serde::{Serialize};
use futures::{StreamExt};
//pour le protocol websocket 
use std::time::{Duration, Instant};
use actix::prelude::*;
//use actix_web_actors::ws;
//use std::{io, thread};

pub mod tls;

   /* Définition */
     #[derive(Serialize)]
     struct RequestData {
         method: String,
         uri: String,
         body: String,
         headers: Vec<(String, String)>,
     }
     
     
     
    
    pub async fn extract_http_request(req: HttpRequest) -> (HttpResponse,String) {
         /* extraction du body,header,method,url */
         let mut headers = Vec::new();
         for (key, value) in req.headers() {
             headers.push((key.to_string(), value.to_str().unwrap().to_string()));
         }
          /* Init*/
         let _request_json = RequestData{
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
         return (HttpResponse::Ok().body(result.clone()),result.clone());
     }
     
     

async fn wsclient(req1: HttpRequest)-> HttpResponse {
    log::info!("starting echo WebSocket client");

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    
    let (_httresp,cmd)=extract_http_request(req1).await;
    cmd_tx.send(cmd.clone()).unwrap();
    

    let (res, mut ws) = awc::Client::new()
        .ws("http://127.0.0.1:8443")
        .connect()
        .await
        .unwrap();

    log::debug!("response: {res:?}");
    log::info!("connected; server will echo messages sent");

    loop {
        select! {
            Some(msg) = ws.next() => {
                match msg {
                    Ok(ws::Frame::Text(txt)) => {
                        // log echoed messages from server
                        log::info!("Server: {txt:?}")
                    }

                    Ok(ws::Frame::Ping(_)) => {
                        // respond to ping probes
                        ws.send(ws::Message::Pong(Bytes::new())).await.unwrap();
                    }

                    _ => {}
                }
            }

            Some(cmd) = cmd_rx.next() => {
                if cmd.is_empty() {
                    continue;
                }

                ws.send(ws::Message::Text(cmd.into())).await.unwrap();
            }

            else => break
        }
    }
    HttpResponse::Ok().body(cmd.clone())

}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP client at https://localhost:8080");
    //let config = tls::load_rustls_config();

    HttpServer::new(|| {
        App::new()
            // WebSocket UI HTML file
            // websocket route
            .service(web::resource("/").to(wsclient))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    //.bind_rustls("127.0.0.1:8080", config)?
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
} 
