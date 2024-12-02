use crate::env::Vars;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use tokio::time::{sleep, Duration};

pub async fn get_retweeted_users(tweet_id: &str) -> Vec<String> {
    // Load Env Variables
    let vars = match Vars::load() {
        Ok(vars) => vars,
        Err(_) => {
            println!("Failed to load environment variables");
            return Vec::new();
        }
    };

    let count = 50;
    let mut user_ids: Vec<String> = Vec::new();
    let mut cursor = String::new();
    let mut has_next_cursor = true;

    while has_next_cursor {
        /*
        please contact the author of this project.

        Contact info

        Telegram: https://t.me/idioRusty
        */
    }

    println!("Length: {}", user_ids.len());
    user_ids
}
