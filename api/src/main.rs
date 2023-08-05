use actix::{Actor, Addr, AsyncContext, StreamHandler};
use actix_files::NamedFile;
use actix_http::ws::Item;
use actix_web::{
    get,
    web::{self, Bytes},
    App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_actors::ws;
use std::{collections::HashMap, env, panic, path::Path};

/// Define HTTP actor
struct MyWs {
    memcache: HashMap<Addr<Self>, Vec<Bytes>>,
}

impl Default for MyWs {
    fn default() -> Self {
        MyWs {
            memcache: HashMap::new(),
        }
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

fn make_class_diagram(src: String) -> String {
    let result = panic::catch_unwind(|| nereus::transform(src).unwrap());
    result.unwrap_or(String::new())
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("received message of length {}", text.len());
                ctx.text(make_class_diagram(text.to_string()));
            }
            Ok(ws::Message::Continuation(item)) => match item {
                Item::FirstText(bytes) => {
                    self.memcache.insert(ctx.address(), vec![bytes]);
                }
                Item::Continue(bytes) => {
                    self.memcache.get_mut(&ctx.address()).unwrap().push(bytes);
                }
                Item::Last(bytes) => {
                    let cached = self.memcache.get_mut(&ctx.address()).unwrap();
                    cached.push(bytes);
                    let text = cached.concat();
                    dbg!("{}", String::from_utf8(text.clone()));
                    cached.clear();
                    ctx.text(make_class_diagram(String::from_utf8(text).unwrap()));
                }
                _ => (),
            },
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
    Ok(NamedFile::open(path)?)
}

async fn index() -> Result<NamedFile> {
    let curr_dir = env::current_dir()?;
    let path = curr_dir.as_path().join(Path::new("static/index.html"));
    Ok(NamedFile::open(path)?)
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs::default(), &req, stream);
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
