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
        sleep(Duration::from_secs(3)).await;
        println!("Start");
        // Construct the request URL
        let endpoint_url = format!(
            "https://{}/v1.1/FollowersIds/?id={}&count=5000&cursor={}",
            vars.rapid_galvier_api_host, user_id, cursor
        );

        // Set up headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-rapidapi-key",
            HeaderValue::from_str(&vars.rapid_api_key).unwrap(),
        );
        headers.insert(
            "x-rapidapi-host",
            HeaderValue::from_str(&vars.rapid_galvier_api_host).unwrap(),
        );

        // Create HTTP client
        let client = reqwest::Client::new();

        match client.get(&endpoint_url).headers(headers).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Value>().await {
                        Ok(json_data) => {
                            // Update cursor for the next page if available
                            match json_data.get("next_cursor_str").and_then(|c| c.as_str()) {
                                Some(next_cursor) => {
                                    if next_cursor == "0" {
                                        has_next_cursor = false;
                                    } else {
                                        cursor = next_cursor.to_string();
                                    }
                                }
                                None => {
                                    println!("Failed to load next cursor.");
                                    break;
                                }
                            }

                            // Process followers' IDs
                            if let Some(ids_array) =
                                json_data.get("ids").and_then(|ids| ids.as_array())
                            {
                                for id in ids_array {
                                    if let Some(id_str) = id.as_str() {
                                        followers_id.push(id_str.to_string());
                                    }
                                }
                            } else {
                                println!("Expected 'ids' field to be an array.");
                                break;
                            }
                        }
                        Err(_) => {
                            println!("Failed to parse JSON response.");
                            break;
                        }
                    }
                } else {
                    println!(
                        "Received an unsuccessful HTTP response: {}",
                        response.status()
                    );
                    break;
                }
            }
            Err(e) => {
                println!("Request failed: {:?}", e);
                break;
            }
        }
    }
    println!("length: {}", followers_id.len());
    followers_id
}
