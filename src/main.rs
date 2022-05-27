use actix_files as fs;
use actix::{Actor, StreamHandler};
use actix_web::{get, web, App, HttpResponse, HttpServer, Error, HttpRequest};
use actix_web_actors::ws;
use rand::prelude::*;

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws_init(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}



#[get("/api/random/{min}/{max}")]
async fn random(minmax: web::Path<(u64, u64)>) -> HttpResponse {
    let mut rng = rand::thread_rng();
    let num: u64 = rng.gen_range(minmax.0..minmax.1);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("{}", num))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ws/", web::get().to(ws_init))
            .service(random)
            .service(fs::Files::new("/", "./www/public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .bind(("::1", 8080))?
    .run()
    .await
}