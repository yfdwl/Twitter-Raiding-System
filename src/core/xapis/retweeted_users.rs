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
        sleep(Duration::from_secs(3)).await;
        println!("Start");
        println!("Cursor: {}", cursor);

        // Construct the request URL
        let endpoint_url = format!(
            "https://{}/v2/Retweeters/?id={}&count={}&cursor={}",
            vars.rapid_galvier_api_host, tweet_id, count, cursor
        );

        // Construct the request URL
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-rapidapi-key",
            HeaderValue::from_str(&vars.rapid_api_key).unwrap(),
        );
        headers.insert(
            "x-rapidapi-host",
            HeaderValue::from_str(&vars.rapid_galvier_api_host).unwrap(),
        );

        let client = reqwest::Client::new();
        match client.get(&endpoint_url).headers(headers).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<Value>().await {
                        Ok(json_data) => {
                            if let Some(entries) = json_data
                                .get("data")
                                .and_then(|data| data.get("retweeters_timeline"))
                                .and_then(|timeline| timeline.get("timeline"))
                                .and_then(|timeline| timeline.get("instructions"))
                                .and_then(|instructions| instructions.get(0))
                                .and_then(|instruction| instruction.get("entries"))
                            {
                                if let Some(entries_array) = entries.as_array() {
                                    if entries_array.len() == 2 {
                                        has_next_cursor = false;
                                    }
                                    else {
                                        if let Some(last_entry) = entries_array.last() {
                                            if let Some(cursor_bottom) = last_entry
                                                .get("content")
                                                .and_then(|content| content.get("value"))
                                                .and_then(Value::as_str)
                                            {
                                                cursor = cursor_bottom.to_string();
                                            } else {
                                                println!("Failed to extract cursor from the last entry.");
                                                break;
                                            }
                                        } else {
                                            println!("Entries array is empty, stopping pagination.");
                                            break;
                                        }
                                    }

                                    for entry in entries_array {
                                        if let Some(id_value) =
                                            entry.get("entryId").and_then(Value::as_str)
                                        {
                                            if id_value.starts_with("user-") {
                                                user_ids.push(id_value.to_string());
                                            }
                                        }
                                    }
                                } else {
                                    println!("Expected entries to be an array.");
                                    break;
                                }
                            } else {
                                println!("Could not find the expected fields in the JSON data.");
                                break;
                            }
                        }
                        Err(_) => {
                            println!("Failed to parse JSON");
                            break;
                        }
                    }
                } else {
                    println!("Unsuccessful request");
                    break;
                }
            }
            Err(_) => {
                println!("Failed to send request");
                break;
            }
        }
    }

    println!("Length: {}", user_ids.len());
    user_ids
}
