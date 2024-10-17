use actix_web::HttpResponse;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

pub struct UserID {
    pub id: i32,
}

// pub async fn get_retweeted_users(tweet_id: web::Path<String>) -> HttpResponse {
pub async fn get_retweeted_users() -> HttpResponse {
    // let user_ids: Vec<UserID> = Vec::new();
    // Load the Bearer token from environment variables
    // let token = match env::var("BEARER_TOKEN") {
    //     Ok(val) => val,
    //     Err(_) => return HttpResponse::InternalServerError().body("Missing bearer token"),
    // };
    let api_host = "twitter135.p.rapidapi.com";
    let api_key = "6c46e6f106msh5d04b566bf506d5p196810jsn01cafc4d064b"; // Add your API key here

    let tweet_id = "1846574339434447299"; // Example Tweet ID
    let count = 20;

    let endpoint_url = format!(
        "https://{}/v2/Retweeters/?id={}&count={}",
        api_host, tweet_id, count
    );

    // let endpoint_url = format!("https://api.twitter.com/2/tweets/{}/retweeted_by", tweet_id);
    // let endpoint_url = format!("https://twitter135.p.rapidapi.com/v2/Retweeters/");

    // Create headers
    // let mut headers = HeaderMap::new();
    // headers.insert(USER_AGENT, HeaderValue::from_static("v2LikingUsersRust"));
    // headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

    let mut headers = HeaderMap::new();
    headers.insert("x-rapidapi-key", HeaderValue::from_str(api_key).unwrap());
    headers.insert("x-rapidapi-host", HeaderValue::from_str(api_host).unwrap());
    // Send request
    let client = reqwest::Client::new();
    println!("f{endpoint_url}");
    match client.get(&endpoint_url).headers(headers).send().await {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<Value>().await {
                    Ok(json_data) => {
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                        HttpResponse::Ok().json(json_data)
                    }
                    Err(_) => HttpResponse::InternalServerError().body("Failed to parse JSON"),
                }
            } else {
                HttpResponse::BadRequest().body("Unsuccessful request")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to send request"),
    }
}
