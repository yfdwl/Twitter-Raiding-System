use crate::env::Vars;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
};
use serde_json::Value;
use tokio::time::{sleep, Duration};

pub async fn get_replying_users(tweet_id: &str) -> Vec<String> {
    let vars = match Vars::load() {
        Ok(vars) => vars,
        Err(_) => {
            println!("Failed to load environment variables.");
            return Vec::new();
        }
    };

    let mut user_ids: Vec<String> = Vec::new();
    let mut has_next_cursor = true;
    let mut cursor = String::new();

    while has_next_cursor {
        /*
        please contact the author of this project.

        Contact info

        Telegram: https://t.me/idioRusty
        */
    }

    user_ids
}
