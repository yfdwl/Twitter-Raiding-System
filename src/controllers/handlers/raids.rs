use actix_web::{
    web::{self, Data},
    Error as ActixError, HttpResponse,
};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::{
    db::PgDb,
    discord::apis::{send_msg_channel, send_msg_to_owner},
    staking_raids::queries::add_new_project,
    staking_raids_user::queries::{add_new_user, get_points_by_user_and_project, get_leaderboard_by_project},
};

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

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct NewProject {
    project_id: String,
    tweet_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UserJoinPath {
    user_id: String,
    project_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UserPointsPath {
    user_id: String,
    raid_id: String,
}

#[api_operation(summary = "Send message to server owner", tag = "user")]
pub async fn post_send_msg_to_owner(
    payload: web::Json<SendMsgRequest>,
) -> Result<HttpResponse, ActixError> {
    let guild_id = &payload.guild_id;
    let message_content = payload.message_content.clone();

    match send_msg_to_owner(guild_id, message_content).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Message sent successfully to the server owner.")),
        Err(e) => {
            eprintln!(
                "Failed to send message to the server owner. Guild ID: {}, Error: {}",
                guild_id, e
            );
            Ok(HttpResponse::InternalServerError()
                .json("Failed to send message to the server owner."))
        }
    }
}

#[api_operation(summary = "Send message to channel", tag = "user")]
pub async fn post_send_msg_to_channel(
    payload: web::Json<ChannelMsgRequest>,
) -> Result<HttpResponse, ActixError> {
    let channel_id = &payload.channel_id;
    let message_content = payload.message_content.clone();

    match send_msg_channel(channel_id, message_content).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Message sent successfully to the channel.")),
        Err(e) => {
            eprintln!(
                "Failed to send message to channel. Channel ID: {}, Error: {}",
                channel_id, e
            );
            Ok(HttpResponse::InternalServerError().json("Failed to send message to the channel."))
        }
    }
}

#[api_operation(summary = "create new project", tag = "user")]
pub async fn create_new_project(
    db: Data<PgDb>,
    payload: web::Json<NewProject>,
) -> Result<HttpResponse, ActixError> {
    let project_id = &payload.project_id;
    let tw_ids = &payload.tweet_ids;

    match add_new_project(db.pool(), project_id.clone(), tw_ids.clone()).await {
        Ok(raid) => Ok(HttpResponse::Ok().json(raid)),
        Err(e) => {
            eprintln!(
                "Failed to add new project to staking_raids table, Error: {}",
                e
            );
            Ok(HttpResponse::InternalServerError()
                .json("Failed to add new project to staking_raids table"))
        }
    }
}

#[api_operation(summary = "join new project", tag = "user")]
pub async fn join_project(
    db: Data<PgDb>,
    path: web::Path<UserPointsPath>,
) -> Result<HttpResponse, ActixError> {
    let user_id = &path.user_id;
    let project_id = &path.raid_id;

    match add_new_user(db.pool(), user_id.clone(), project_id.clone()).await {
        Ok(raid_user) => Ok(HttpResponse::Ok().json(raid_user)),
        Err(e) => {
            eprintln!(
                "Failed to add new user to staking_raids_user table, Error: {}",
                e
            );
            Ok(HttpResponse::InternalServerError()
                .json("Failed to add new user to staking_raids_user"))
        }
    }
}

#[api_operation(summary = "get points by project and user", tag = "user")]
pub async fn get_points(
    db: Data<PgDb>,
    path: web::Path<UserJoinPath>,
) -> Result<HttpResponse, ActixError> {
    let user_id = &path.user_id;
    let project_id = &path.project_id;

    match get_points_by_user_and_project(db.pool(), user_id.clone(), project_id.clone()).await {
        Ok(raid_user) => Ok(HttpResponse::Ok().json(raid_user)),
        Err(e) => {
            eprintln!("Failed to get user's points, Error: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to get user's points"))
        }
    }
}

#[api_operation(summary = "track leaderboard by project", tag = "user")]
pub async fn track_leaderboard(
    db: Data<PgDb>,
    path: web::Path<String>,
) -> Result<HttpResponse, ActixError> {
    let project_id = &path.into_inner();

    match get_leaderboard_by_project(db.pool(), project_id.clone()).await {
        Ok(leaderboard) => Ok(HttpResponse::Ok().json(leaderboard)),
        Err(e) => {
            eprintln!(
                "Error retrieving leaderboard for project ID: {}: {:?}",
                project_id, e
            );
            Ok(HttpResponse::InternalServerError().json("Error retrieving leaderboard"))
        }
    }
}