use actix_web::{middleware, web, get, App, HttpRequest, HttpResponse, HttpServer,Error,Responder,FromRequest};
use actix_web::web::{Bytes,Payload};
//use awc::ws;
use futures_util::{sink::SinkExt as _, stream::StreamExt as _};
use tokio::{select, sync::mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use serde::{Serialize};
use futures::{StreamExt};
//websocket protocol
use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web_actors::ws;
//use actix_send_websocket::{Message, WebSocket};
use actix_web::rt::net::Ready;
use actix_web::error::ErrorBadRequest;
use std::future::ready;

use mysql::*;
use mysql::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt::Debug;
use std::fmt;

pub mod tls;
 
 /* Définition */
 #[derive(Serialize)]
 struct RequestData {
     method: String,
     uri: String,
     body: String,
     headers: Vec<(String, String)>,
 }
 
 #[derive(Debug, PartialEq, Eq)]
struct Autorisation{
    Nom: String,
    auto: String,

}
 
fn bdd_research(base : String)->actix_web::Result<String,Box<dyn std::error::Error>>{
    let url = "mysql://aimasu:BIFROST@localhost:3306/autorisation";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let selected_auto = conn
    	.query_map(
            "SELECT * FROM credential where Nom='Mindflow' ",
            |(Nom, auto)| {
                Autorisation { Nom, auto}
            },
        )?;

        let mut log_extrait = base.to_string();
        log_extrait.push_str("Authorization: ");

        for burnout in selected_auto {
        log_extrait.push_str(&burnout.auto);
        }
        log_extrait.push_str("\r\n");

        Ok(log_extrait)


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



/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}


/// Handler for `ws::Message`
impl StreamHandler<actix_web::Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: actix_web::Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let mut text2=text.clone();
                ctx.text(text);
                let mut test = bdd_research(text2.to_string());
                println!("{:?}",test);
                

            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    
    }
    
}


/// WebSocket handshake and start `MyWebSocket` actor.
/*async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket::new(), &req, stream);
    let (_httresp,cmd)=extract_http_request(req).await;
    println!("{}",cmd);
    resp
}*/

async fn ws_route(req: HttpRequest, stream: web::Payload) -> actix_web::Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket::new(), &req, stream);
    //let (_httresp,cmd)=extract_http_request(req).await;
    //println!("{}",cmd);
    //ws::Message::Text(cmd.into());
    resp
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at https://localhost:8443");
    //let config = tls::load_rustls_config();
    
    HttpServer::new(|| {
        App::new()
            // WebSocket UI HTML file
            // websocket route
            //.service(extract_http_request)
            //.service(web::resource("/").route(web::get().to(ws_route)))
            // enable logger
            .service(web::resource("/").to(ws_route))
            .wrap(middleware::Logger::default())
    })
    //.bind_rustls("127.0.0.1:8443", config)?
    .bind(("127.0.0.1", 8443))?
    .workers(2)
    .run()
    .await
} 