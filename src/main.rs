mod story;

use actix_files::NamedFile;
use actix_web::{get, web::Query, App, HttpResponse, HttpServer, Responder};
use log::info;
use serde::Deserialize;
use story::Story;

#[derive(Deserialize, Debug)]
struct Filter {
    pub query: Option<String>,
}

#[get("/stories")]
async fn stories(filter: Query<Filter>) -> impl Responder {
    if let Ok(mut stories) = Story::fetch_new().await {
        info!("Query: {:?}", &filter);
        stories.sort_by(|a, b| b.score.cmp(&a.score));
        if let Some(query) = &filter.query {
            stories.retain(|s| s.title.to_lowercase().contains(&query.to_lowercase()));
        }
        HttpResponse::Ok().body(format!("{:#?}", stories))
    } else {
        HttpResponse::InternalServerError().body("Error while processing the request!")
    }
}

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open("./static/index.html")
}

#[get("/style.css")]
async fn css() -> impl Responder {
    NamedFile::open("./static/style.css")
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    NamedFile::open("./static/favicon.ico")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(stories)
            .service(css)
            .service(favicon)
    })
    .bind("127.0.0.1:3232")?
    .run()
    .await
}
