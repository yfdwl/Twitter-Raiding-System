use crate::env::Vars;
use log::{debug, error, info};
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

pub async fn send_msg_channel(
    channel_id: &str,
    message_content: Value,
) -> Result<(), Box<dyn Error>> {
    // Load bot token from environment variables.
    let bot_token = Vars::load()
        .map_err(|_| DiscordError("Failed to load environment variables".into()))?
        .discord_bot_token;

    // Construct Discord API endpoint URL.
    let discord_api_url = format!(
        "https://discord.com/api/v10/channels/{}/messages",
        channel_id
    );

    // Create an HTTP client instance.
    let client = Client::new();

    // Send a POST request to the Discord API with JSON payload.
    let response = client
        .post(&discord_api_url)
        .header("Authorization", format!("Bot {}", bot_token))
        .header("Content-Type", "application/json")
        .json(&message_content) // Pass the structured JSON content directly.
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

pub async fn send_msg_to_owner(
    guild_id: &str,
    message_content: Value,
) -> Result<(), Box<dyn Error>> {
    let bot_token = Vars::load()
        .map_err(|_| DiscordError("Failed to load environment variables".into()))?
        .discord_bot_token;

    let client = Client::new();

    // Retrieve server (guild) information to get owner ID
    let guild_url = format!("https://discord.com/api/v10/guilds/{}", guild_id);
    let guild_response = client
        .get(&guild_url)
        .header("Authorization", format!("Bot {}", bot_token))
        .send()
        .await
        .map_err(|e| {
            error!("Failed to fetch guild information: {}", e);
            DiscordError("Failed to fetch guild information".into())
        })?
        .json::<Value>()
        .await?;

    let owner_id = guild_response["owner_id"]
        .as_str()
        .ok_or_else(|| DiscordError("Owner ID not found in guild response".into()))?;

    debug!("Retrieved owner ID: {}", owner_id);

    // Create a DM channel with the owner
    let dm_channel_url = "https://discord.com/api/v10/users/@me/channels";
    let dm_response = client
        .post(dm_channel_url)
        .header("Authorization", format!("Bot {}", bot_token))
        .json(&json!({ "recipient_id": owner_id }))
        .send()
        .await
        .map_err(|e| {
            error!("Failed to create DM channel with owner: {}", e);
            DiscordError("Failed to create DM channel".into())
        })?
        .json::<Value>()
        .await?;

    let dm_channel_id = dm_response["id"]
        .as_str()
        .ok_or_else(|| DiscordError("DM Channel ID not found in response".into()))?;

    debug!("DM Channel ID for owner: {}", dm_channel_id);

    // Send a message to the DM channel
    let message_url = format!(
        "https://discord.com/api/v10/channels/{}/messages",
        dm_channel_id
    );
    let response = client
        .post(&message_url)
        .header("Authorization", format!("Bot {}", bot_token))
        .header("Content-Type", "application/json")
        .json(&message_content)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to send message to DM channel: {}", e);
            DiscordError("Failed to send message to DM channel".into())
        })?;

    if response.status().is_success() {
        info!("Message sent successfully to the server owner.");
    } else {
        error!(
            "Failed to send message, status: {}, response: {:?}",
            response.status(),
            response.text().await
        );
    }

    Ok(())
}
