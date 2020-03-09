mod story;

use actix_files as fs;
use actix_web::{get, web::Query, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;
use serde::Deserialize;
use std::path::PathBuf;
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
    fs::NamedFile::open("./static/index.html")
}

#[get("/{filename:.*}")]
async fn files(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let mut path = PathBuf::from("./static");
    path.push(req.match_info().query("filename"));
    Ok(fs::NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| App::new().service(index).service(stories).service(files))
        .bind("127.0.0.1:3232")?
        .run()
        .await
}
