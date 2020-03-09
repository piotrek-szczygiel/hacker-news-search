use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use futures::future;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[get("/")]
async fn index() -> impl Responder {
    if let Ok(ip) = get_new_stories().await {
        HttpResponse::Ok().body(format!("{:#?}", ip))
    } else {
        HttpResponse::InternalServerError().body("Error while processing the request!")
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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
        static ref CACHE: Mutex<HashMap<u64, Story>> = Mutex::new(HashMap::new());
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
            if let Some(story) = CACHE.lock().unwrap().get(&id) {
                println!("Using cached result for {}", id);
                return Ok(story.clone());
            }

            println!("Fetching result for {}", id);
            let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            let story = client.get(&url).send().await?.json::<Story>().await;
            if let Ok(story) = story.as_ref().map(|s| s.clone()) {
                CACHE.lock().unwrap().insert(id, story);
            }
            story
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
