use crate::env::Vars;
use reqwest::Client;
use serde_json::{json, Value};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct DiscordError(String);

impl fmt::Display for DiscordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DiscordError {}

pub async fn send_msg_channel(channel_id: &str, message_content: Value) -> Result<(), Box<dyn Error>> {
    // Load bot token from environment variables.
    let bot_token = Vars::load()
        .map_err(|_| DiscordError("Failed to load environment variables".into()))?
        .discord_bot_token;

    // Construct Discord API endpoint URL.
    let discord_api_url = format!("https://discord.com/api/v10/channels/{}/messages", channel_id);

    // Create an HTTP client instance.
    let client = Client::new();

    // Send a POST request to the Discord API with JSON payload.
    let response = client
        .post(&discord_api_url)
        .header("Authorization", format!("Bot {}", bot_token))
        .header("Content-Type", "application/json")
        .json(&message_content)  // Pass the structured JSON content directly.
        .send()
        .await
        .map_err(|e| DiscordError(format!("Failed to send request: {}", e)))?;

    if response.status().is_success() {
        println!("Message sent successfully to channel ID: {}", channel_id);
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".into());
        return Err(Box::new(DiscordError(format!(
            "Failed to send message: {}",
            error_text
        ))));
    }

    Ok(())
}