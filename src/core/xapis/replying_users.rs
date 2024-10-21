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
        sleep(Duration::from_secs(3)).await;
        println!("start");

        // Construct the request URL
        let endpoint_url = format!(
            "https://{}/comments?pid={}&count=20&rankingMode=Recency&cursor={}",
            vars.rapid_davethebeast_api_host, tweet_id, cursor
        );
        
        // Set up headers
        let mut headers = HeaderMap::new();
        headers.insert("x-rapidapi-key", HeaderValue::from_str(&vars.rapid_api_key).unwrap());
        headers.insert("x-rapidapi-host", HeaderValue::from_str(&vars.rapid_davethebeast_api_host).unwrap());

        let client = reqwest::Client::new();
        match client.get(&endpoint_url).headers(headers).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Value>().await {
                        Ok(json_data) => {
                            // Handle the cursor for pagination
                            if let Some(new_cursor) = json_data
                                .get("cursor")
                                .and_then(|cursor| cursor.get("bottom"))
                                .and_then(|bottom| bottom.as_str())
                            {
                                cursor = new_cursor.to_string();
                                println!("Next cursor: {}", cursor);
                            } else {
                                has_next_cursor = false;
                            }

                            // Process the entries for user IDs
                            if let Some(entries) = json_data
                                .get("result")
                                .and_then(|result| result.get("instructions"))
                                .and_then(|instructions| instructions.get(0))
                                .and_then(|instruction| instruction.get("entries"))
                            {
                                if let Some(entries_array) = entries.as_array() {
                                    for entry in entries_array {
                                        if let Some(entry_id) =
                                            entry.get("entryId").and_then(|e| e.as_str())
                                        {
                                            if entry_id.starts_with("conversationthread-") {
                                                if let Some(user_id) = entry
                                                    .get("content")
                                                    .and_then(|content| content.get("items"))
                                                    .and_then(|items| items.get(0))
                                                    .and_then(|item| item.get("item"))
                                                    .and_then(|item| item.get("itemContent"))
                                                    .and_then(|item_content| {
                                                        item_content.get("tweet_results")
                                                    })
                                                    .and_then(|tweet_result| {
                                                        tweet_result.get("result")
                                                    })
                                                    .and_then(|result| result.get("legacy"))
                                                    .and_then(|legacy| legacy.get("user_id_str"))
                                                    .and_then(|user_id_str| user_id_str.as_str())
                                                {
                                                    user_ids.push(user_id.to_string());
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    println!("No valid entries found.");
                                    break;
                                }
                            } else {
                                println!("Expected fields not found in the response.");
                                break;
                            }
                        }
                        Err(_) => {
                            println!("Failed to parse the JSON response.");
                            break;
                        }
                    }
                } else {
                    println!("Request unsuccessful with status: {}", response.status());
                    break;
                }
            }
            Err(_) => {
                println!("Failed to send request.");
                break;
            }
        }
    }

    user_ids
}
