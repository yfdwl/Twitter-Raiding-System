use actix_web::{
    web::{self, Data},
    Error as ActixError, HttpResponse,
};
use apistos::{api_operation, ApiComponent};
use bigdecimal::BigDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    config::Action,
    core::{
        db::PgDb,
        discord::apis::{send_msg_channel, send_msg_to_owner},
        points_calc::calc::get_bonus_points,
        staking_raids::raids::{
            create_new_raid, get_leaderboard_by_project_id, get_raids_by_project_and_user_id,
            get_user_by_id, get_user_points_by_raid, update_pt_by_following, update_pt_by_replying,
            update_pt_by_retweet,
        },
        users::beta_users::get_all_beta_users_twitter_ids,
        xapis::{
            following_users::get_following_users, replying_users::get_replying_users,
            retweeted_users::get_retweeted_users,
        },
    },
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
pub struct ReplyPath {
    project_id: String,
    tweet_id: String,
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

#[api_operation(summary = "track leaderboard by project id", tag = "user")]
pub async fn track_leaderboard(
    db: Data<PgDb>,
    path: web::Path<String>,
) -> Result<HttpResponse, ActixError> {
    let project_id = &path.into_inner();

    match get_leaderboard_by_project_id(db.pool(), project_id.clone()).await {
        Ok(leaderboard) => Ok(HttpResponse::Ok().json(leaderboard)), // Send leaderboard as JSON
        Err(e) => {
            eprintln!(
                "Error retrieving leaderboard for project ID: {}: {:?}",
                project_id, e
            );
            Ok(HttpResponse::InternalServerError().json("Error retrieving leaderboard"))
        }
    }
}

#[api_operation(summary = "tweet replying", tag = "user")]
pub async fn tweet_replying(
    db: web::Data<PgDb>,
    path: web::Path<ReplyPath>,
) -> Result<HttpResponse, ActixError> {
    let project_id = &path.project_id;
    let tweet_id = &path.tweet_id;
    let replying_users = get_replying_users(tweet_id).await;
    let beta_users = get_all_beta_users_twitter_ids(db.pool()).await;

    for beta_user in &beta_users {
        if replying_users.contains(&beta_user) {
            let user_id = get_user_by_id(
                db.pool(),
                project_id.clone(),
                tweet_id.clone(),
                beta_user.clone(),
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to get user ID for Project ID: {}, Tweet ID: {}, Beta User: {}. Error: {:?}", project_id, tweet_id, beta_user, e);
                actix_web::error::ErrorInternalServerError("Error in getting user by ID.")
            })?;
            match user_id {
                Some(_) => update_pt_by_replying(
                    db.pool(),
                    project_id.clone(),
                    tweet_id.clone(),
                    beta_user.clone(),
                    get_bonus_points(&Action::Reply),
                )
                .await
                .map_err(|e| {
                    eprintln!("Error updating reply points for User: {} on Project: {} with Tweet: {}. Error: {:?}", beta_user, project_id, tweet_id, e);
                    actix_web::error::ErrorInternalServerError("Error updating reply points.")
                })?,
                None => create_new_raid(
                    db.pool(),
                    project_id.clone(),
                    tweet_id.clone(),
                    beta_user.clone(),
                    BigDecimal::from(0),
                    BigDecimal::from(0),
                    BigDecimal::from(0),
                    get_bonus_points(&Action::Reply),
                    get_bonus_points(&Action::Reply),
                )
                .await
                .map_err(|e| {
                    eprintln!("Failed to create new raid for Project ID: {}, Tweet ID: {}, User: {}. Error: {:?}", project_id, tweet_id, beta_user, e);
                    actix_web::error::ErrorInternalServerError("Error creating new raid.")
                })?,
            };
        }
    }

    Ok(HttpResponse::Ok().json("SUCCESS"))
}

#[api_operation(summary = "tweet retweet", tag = "user")]
pub async fn tweet_retweet(
    db: web::Data<PgDb>,
    path: web::Path<ReplyPath>,
) -> Result<HttpResponse, ActixError> {
    let project_id = &path.project_id;
    let tweet_id = &path.tweet_id;
    let retweet_users = get_retweeted_users(tweet_id).await;
    let beta_users = get_all_beta_users_twitter_ids(db.pool()).await;

    for beta_user in &beta_users {
        if retweet_users.contains(&beta_user) {
            let user_id = get_user_by_id(
                db.pool(),
                project_id.clone(),
                tweet_id.clone(),
                beta_user.clone(),
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to get user ID for retweet. Project ID: {}, Tweet ID: {}, User: {}. Error: {:?}", project_id, tweet_id, beta_user, e);
                actix_web::error::ErrorInternalServerError("Error retrieving user ID for retweet.")
            })?;
            match user_id {
                Some(_) => update_pt_by_retweet(
                    db.pool(),
                    project_id.clone(),
                    tweet_id.clone(),
                    beta_user.clone(),
                    get_bonus_points(&Action::Retweet),
                )
                .await
                .map_err(|e| {
                    eprintln!("Error updating retweet points for Project: {}, Tweet: {}, User: {}. Error: {:?}", project_id, tweet_id, beta_user, e);
                    actix_web::error::ErrorInternalServerError("Error updating retweet points.")
                })?,
                None => create_new_raid(
                    db.pool(),
                    project_id.clone(),
                    tweet_id.clone(),
                    beta_user.clone(),
                    BigDecimal::from(0),
                    BigDecimal::from(0),
                    get_bonus_points(&Action::Retweet),
                    BigDecimal::from(0),
                    get_bonus_points(&Action::Retweet),
                )
                .await
                .map_err(|e| {
                    eprintln!("Failed to create new raid for retweet. Project ID: {}, Tweet ID: {}, User: {}. Error: {:?}", project_id, tweet_id, beta_user, e);
                    actix_web::error::ErrorInternalServerError("Error creating new raid for retweet.")
                })?,
            };
        }
    }

    Ok(HttpResponse::Ok().json("SUCCESS"))
}

#[api_operation(summary = "project followers", tag = "user")]
pub async fn project_following(
    db: web::Data<PgDb>,
    path: web::Path<String>,
) -> Result<HttpResponse, ActixError> {
    let project_id = &path.into_inner();
    let followers = get_following_users(project_id).await;
    let beta_users = get_all_beta_users_twitter_ids(db.pool()).await;

    for beta_user in &beta_users {
        if followers.contains(&beta_user) {
            let raids =
                get_raids_by_project_and_user_id(db.pool(), project_id.clone(), beta_user.clone())
                    .await
                    .map_err(|e| {
                        eprintln!(
                    "Error fetching raids by user ID for Project ID: {}. User: {}. Error: {:?}",
                    project_id, beta_user, e
                );
                        actix_web::error::ErrorInternalServerError(
                            "Error retrieving raids by user ID.",
                        )
                    })?;

            if (raids.len() != 0) {
                for raid in raids {
                    update_pt_by_following(
                        db.pool(),
                        project_id.clone(),
                        raid.user_id.unwrap(),
                        get_bonus_points(&Action::Following),
                    )
                    .await
                    .map_err(|e| {
                        eprintln!("Error updating points for following. Project ID: {}, User: {}. Error: {:?}", project_id, beta_user, e);
                        actix_web::error::ErrorInternalServerError("Error updating points for following.")
                    })?;
                }
            } else {
                create_new_raid(
                    db.pool(),
                    project_id.clone(),
                    "".to_string(),
                    beta_user.clone(),
                    get_bonus_points(&Action::Following),
                    BigDecimal::from(0),
                    BigDecimal::from(0),
                    BigDecimal::from(0),
                    get_bonus_points(&Action::Following),
                )
                .await
                .map_err(|e| {
                    eprintln!("Failed to create new raid for following. Project ID: {}, User: {}. Error: {:?}", project_id, beta_user, e);
                    actix_web::error::ErrorInternalServerError("Error creating new raid for following.")
                })?;
            }
        }
    }

    Ok(HttpResponse::Ok().json("SUCCESS"))
}
#[api_operation(summary = "fetch how many points a user has", tag = "user")]
pub async fn get_user_points(
    db: Data<PgDb>,
    path: web::Path<UserPointsPath>,
) -> Result<HttpResponse, ActixError> {
    let user_id = &path.user_id;
    let project_id = &path.raid_id;

    match get_user_points_by_raid(db.pool(), project_id.clone(), user_id.clone()).await {
        Ok(res) => {
            let user_points = res.unwrap();
            Ok(HttpResponse::Ok().json(user_points))
        }
        Err(e) => {
            eprintln!(
                "Error retrieving user's points of project id is and user id is: {} {}: {:?}",
                project_id, user_id, e
            );
            Ok(HttpResponse::InternalServerError().json("Error retrieving user points"))
        }
    }
}
