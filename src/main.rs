mod story;

use actix_files as fs;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use story::Story;

#[derive(Serialize)]
struct QueryResult {
    stories: Option<Vec<Story>>,
    error: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Filter {
    pub query: Option<String>,
}

#[get("/stories")]
async fn stories(filter: web::Query<Filter>) -> impl Responder {
    match Story::fetch_new().await {
        Ok(mut stories) => {
            info!("Query: {:?}", &filter);
            stories.sort_by(|a, b| b.score.cmp(&a.score));
            if let Some(query) = &filter.query {
                stories.retain(|s| s.title.to_lowercase().contains(&query.to_lowercase()));
            }

            HttpResponse::Ok().json(QueryResult {
                stories: Some(stories),
                error: None,
            })
        }
        Err(error) => HttpResponse::Ok().json(QueryResult {
            stories: None,
            error: Some(format!("Error: {:#?}", error)),
        }),
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

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| App::new().service(index).service(stories).service(files))
        .bind(("0.0.0.0", port))
        .expect(&format!("Can not bind to port {}", port))
        .run()
        .await
}
