use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, NaiveDateTime, Utc};
use futures::future;
use lazy_static::lazy_static;
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Story {
    id: u64,
    pub title: String,
    pub score: i64,
    url: String,
    #[serde(deserialize_with = "from_ts")]
    time: DateTime<Utc>,
}

impl Default for Story {
    fn default() -> Self {
        Story {
            id: 0,
            title: "".into(),
            score: 0,
            url: "".into(),
            time: DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
        }
    }
}

impl Story {
    pub async fn fetch_new() -> Result<Vec<Story>, Box<dyn std::error::Error>> {
        lazy_static! {
            static ref CLIENT: Client = Client::new();
        }

        const STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/newstories.json";
        info!("Fetching {}", STORIES_URL);

        let ids = CLIENT
            .get(STORIES_URL)
            .send()
            .await?
            .json::<Vec<u64>>()
            .await?;

        Ok(future::join_all(ids.into_iter().map(|id| Story::fetch(id)))
            .await
            .into_iter()
            .filter_map(|x| x.ok())
            .collect())
    }

    async fn fetch(id: u64) -> Result<Story, reqwest::Error> {
        lazy_static! {
            static ref CLIENT: Client = Client::new();
            static ref CACHE: Mutex<HashMap<u64, Story>> = Mutex::new(HashMap::new());
        }

        if let Some(story) = CACHE.lock().unwrap().get(&id) {
            info!("Using cached story {}", id);
            return Ok(story.clone());
        }

        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        info!("Fetching {}", url);
        let story = CLIENT.get(&url).send().await?.json::<Story>().await;
        if let Ok(story) = story.as_ref().map(|s| s.clone()) {
            CACHE.lock().unwrap().insert(id, story);
        }
        story
    }
}
