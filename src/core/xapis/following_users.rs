use crate::env::Vars;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use tokio::time::{sleep, Duration};

pub async fn get_following_users(user_id: &str) -> Vec<String> {
    // Load Env Variables
    let vars = match Vars::load() {
        Ok(vars) => vars,
        Err(_) => {
            println!("Failed to load environment variables");
            return Vec::new();
        }
    };

    let mut followers_id: Vec<String> = Vec::new();
    let mut cursor: String = String::new();
    let mut has_next_cursor: bool = true;

    while has_next_cursor {
        /*
        please contact the author of this project.

        Contact info

        Telegram: https://t.me/idioRusty
        */
    }
    println!("length: {}", followers_id.len());
    followers_id
}
