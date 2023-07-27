use actix::{Actor, StreamHandler};
use actix_files::NamedFile;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use std::{env, panic, path::Path};

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let m = msg.as_ref().unwrap();
        println!("{m:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let result = panic::catch_unwind(|| nereus::transform(text.to_string()).unwrap());
                if result.is_ok() {
                    return ctx.text(result.unwrap_or(String::new()));
                }
            }

            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("/static/{filename}")]
async fn get_image(info: web::Path<String>) -> Result<NamedFile> {
    let curr_dir = env::current_dir()?;
    let path = curr_dir
        .as_path()
        .join(Path::new(&format!("static/{}", info)));
    // println!("{path:?}");
    Ok(NamedFile::open(path)?)
}

async fn index() -> Result<NamedFile> {
    let curr_dir = env::current_dir()?;
    let path = curr_dir.as_path().join(Path::new("static/index.html"));
    // println!("{path:?}");
    Ok(NamedFile::open(path)?)
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_image)
            .route("/", web::get().to(index))
            .route("/index.html", web::get().to(index))
            .route("/ws/", web::get().to(ws_index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
