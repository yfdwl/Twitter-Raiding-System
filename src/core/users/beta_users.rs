use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{PgPool, Result};
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
    pub birthday: Option<NaiveDate>,
}

pub async fn get_all_beta_users_twitter_ids(pool: &PgPool) -> Vec<String> {
    let twitter_ids = sqlx::query_scalar!(
        r#"
        SELECT twitter_id
        FROM beta_users
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| Vec::new());

    twitter_ids.into_iter().filter_map(|id| id).collect()
}
