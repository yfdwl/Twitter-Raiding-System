use actix_web::{web, HttpResponse, Responder, Error};
use apistos::ApiComponent;
use apistos::api_operation;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use serde_json::Value;
use crate::core::discord::apis::send_msg_channel;
use crate::core::discord::apis::send_msg_to_owner;

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SendMsgRequest {
    guild_id: String,
    message_content: Value,
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ChannelMsgRequest {
    channel_id: String,
    message_content: Value,
}

#[api_operation(summary = "Get Raids Points", tag = "user")]
pub async fn get_raids_points() -> impl Responder {
    HttpResponse::Ok().body("Raids Points")
}

#[api_operation(summary = "Send message to server owner", tag = "user")]
pub async fn post_send_msg_to_owner(
    payload: web::Json<SendMsgRequest>,
) -> Result<HttpResponse, Error> {
    // Extract guild_id and message_content from the request payload
    let guild_id = &payload.guild_id;
    let message_content = payload.message_content.clone();

    // Call the function to send the message to the server owner
    match send_msg_to_owner(guild_id, message_content).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Message sent successfully to the server owner.")),
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to send message to the server owner."))
        }
    }
}

#[api_operation(summary = "Send message to channel", tag = "user")]
pub async fn post_send_msg_to_channel(
    payload: web::Json<ChannelMsgRequest>,
) -> Result<HttpResponse, Error> {
    // Extract guild_id and message_content from the request payload
    let channel_id = &payload.channel_id;
    let message_content = payload.message_content.clone();

    // Call the function to send the message to the server owner
    match send_msg_channel(channel_id, message_content).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Message sent successfully to the channel.")),
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to send message to the channel."))
        }
    }
}