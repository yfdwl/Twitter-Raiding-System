use chrono::NaiveDate;
use sqlx::{PgPool, Result};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub wallet_address: Option<String>,
    pub twitter_id: Option<String>,
    pub twitter_username: Option<String>,
    pub username: Option<String>,
    pub profile_img: Option<String>,
    pub bio: Option<String>,
    pub discord_username: Option<String>,
    pub discord_follows: Option<bool>,
    pub telegram_id: Option<String>,
    pub telegram_username: Option<String>,
    pub linkedin_username: Option<String>,
    pub approved: Option<bool>,
    pub signature: Option<String>,
    pub birthday: Option<NaiveDate>
}

pub async fn get_all_beta_users(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, first_name, last_name, birthday, email, wallet_address, twitter_id, twitter_username, username, profile_img, bio, discord_username, discord_follows, linkedin_username, telegram_id, telegram_username, approved, signature
        FROM beta_users
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}