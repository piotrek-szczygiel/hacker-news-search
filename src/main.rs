use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use futures::future;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> impl Responder {
    if let Ok(ip) = get_new_stories().await {
        HttpResponse::Ok().body(format!("{:#?}", ip))
    } else {
        HttpResponse::InternalServerError().body("Error while processing the request!")
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct Story {
    id: u64,
    title: String,
    score: i64,
    url: String,
}

async fn get_new_stories() -> Result<Vec<Story>, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref CLIENT: Client = Client::new();
    }

    let ids = CLIENT
        .get("https://hacker-news.firebaseio.com/v0/newstories.json")
        .send()
        .await?
        .json::<Vec<u64>>()
        .await?;

    let stories: Vec<Story> = future::join_all(ids.into_iter().map(|id| {
        let client = &CLIENT;
        async move {
            let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            client.get(&url).send().await?.json::<Story>().await
        }
    }))
    .await
    .into_iter()
    .filter_map(|x| x.ok())
    .collect();

    Ok(stories)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:3232")?
        .run()
        .await
}
