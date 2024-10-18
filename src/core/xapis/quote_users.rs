use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use tokio::time::{sleep, Duration};
use crate::env::Vars;

pub async fn get_replying_users(tweet_id: &str) -> Vec<String> {
    let vars = Vars::load();
    let mut user_ids: Vec<String> = Vec::new();
    let mut is_next_cursor_existed: bool = true;
    let mut cursor: String = "".to_string();
    match vars {
        Ok(vars) => {
            while is_next_cursor_existed {
                sleep(Duration::from_secs(3)).await;
                let api_host = vars.rapid_davethebeast_api_host.as_str();
                let api_key = vars.rapid_api_key.as_str();
                let endpoint_url = format!(
                    "https:/{}/comments?pid={}&count=20&rankingMode=Recency&cursor={}",
                    api_host, tweet_id, cursor
                );
    
                let mut headers = HeaderMap::new();
                headers.insert("x-rapidapi-key", HeaderValue::from_str(api_key).unwrap());
                headers.insert("x-rapidapi-host", HeaderValue::from_str(api_host).unwrap());
    
                let client = reqwest::Client::new();

                match client.get(&endpoint_url).headers(headers).send().await {
                    Ok(res) => {
                        if res.status().is_success() {
                            match res.json::<Value>().await {
                                Ok(json_data) => {
                                    if let Some(bottom) = json_data
                                        .get("cursor")
                                        .and_then(|cursor| cursor.get("bottom"))
                                        .and_then(|bottom| bottom.as_str())
                                    {
                                        cursor = bottom.to_string();
                                        println!("cursor:{}:", cursor)
                                    } else {
                                        is_next_cursor_existed = false;
                                    }
                                    if let Some(entries) = json_data
                                        .get("result")
                                        .and_then(|result| result.get("instructions"))
                                        .and_then(|instructions| instructions.get(0))
                                        .and_then(|instruction| instruction.get("entries"))
                                    {
                                        if let Some(entries_array) = entries.as_array() {
                                            for entry in entries_array {
                                                // Loop over entries_array
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
                                                            .and_then(|legacy| {
                                                                legacy.get("user_id_str")
                                                            })
                                                            .and_then(|user_id_str| {
                                                                user_id_str.as_str()
                                                            })
                                                        {
                                                            user_ids.push(user_id.to_string());
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            is_next_cursor_existed = false;
                                        }
                                    } else {
                                        is_next_cursor_existed = false;
                                    }
                                }
                                Err(_) => {
                                    println!("Failed to parse JSON");
                                    is_next_cursor_existed = false;
                                }
                            }
                        } else {
                            println!("Unsuccessful request");
                            is_next_cursor_existed = false;
                        }
                    }
                    Err(_) => {
                        println!("Failed to send request");
                        is_next_cursor_existed = false;
                    }

                }
            }

            println!("{:#?}", user_ids);
            user_ids
        }
        Err(_) => {
            println!("Failed to send request");
            Vec::new()
        }
    }
}
