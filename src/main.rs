mod story;

use actix_files::NamedFile;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use story::Story;

#[get("/")]
async fn index() -> impl Responder {
    if let Ok(ip) = Story::fetch_new().await {
        HttpResponse::Ok().body(format!("{:#?}", ip))
    } else {
        HttpResponse::InternalServerError().body("Error while processing the request!")
    }
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    NamedFile::open("./favicon.ico")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| App::new().service(index).service(favicon))
        .bind("127.0.0.1:3232")?
        .run()
        .await
}
