use crate::env::Vars;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

pub async fn get_retweeted_users(tweet_id: &str) -> Vec<String> {
    let vars = Vars::load();
    match vars {
        Ok(vars) => {
            let api_host = vars.rapid_api_host.as_str();
            let api_key = vars.rapid_api_key.as_str();
            let count = 200;
            let endpoint_url = format!(
                "https://{}/v2/Retweeters/?id={}&count={}",
                api_host, tweet_id, count
            );

            // config headers info
            let mut headers = HeaderMap::new();
            headers.insert("x-rapidapi-key", HeaderValue::from_str(api_key).unwrap());
            headers.insert("x-rapidapi-host", HeaderValue::from_str(api_host).unwrap());

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
                                    let mut user_ids: Vec<String> = Vec::new();
                                    if let Some(entries_array) = entries.as_array() {
                                        for entry in entries_array {
                                            // Check if entry has an id field
                                            if let Some(id_value) =
                                                entry.get("entryId").and_then(Value::as_str)
                                            {
                                                // Check if the ID starts with "user-"
                                                if id_value.starts_with("user-") {
                                                    user_ids.push(id_value.to_string());
                                                }
                                            }
                                        }
                                        print!("{:#?}", user_ids);
                                        Vec::new()
                                    } else {
                                        println!("Expected entries to be an array.");
                                        Vec::new()
                                    }
                                } else {
                                    println!(
                                        "Could not find the expected fields in the JSON data."
                                    );
                                    Vec::new() // Return empty Vec
                                }
                            }
                            Err(_) => {
                                println!("Failed to parse JSON");
                                Vec::new()
                            }
                        }
                    } else {
                        println!("Unsuccessful request");
                        Vec::new()
                    }
                }
                Err(_) => {
                    println!("Failed to send request");
                    Vec::new()
                }
            }
        }
        Err(_) => {
            println!("Failed to send request");
            Vec::new()
        }
    }
}
