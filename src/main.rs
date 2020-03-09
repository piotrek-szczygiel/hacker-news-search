mod story;
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:3232")?
        .run()
        .await
}
