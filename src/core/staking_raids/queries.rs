use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct LeaderBoard {
    user_id: Option<String>,
    total_pt: Option<BigDecimal>,
    rank: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Raid {
    pub id: Uuid,
    pub project_id: String,
    pub tw_ids: Option<Vec<String>>,
}

pub async fn add_new_project(
    pool: &PgPool,
    project_id: String,
    tw_ids: Option<Vec<String>>,
) -> Result<Raid> {
    let inserted_row = sqlx::query_as!(
        Raid,
        r#"
        INSERT INTO staking_raids (project_id, tw_ids)
        VALUES ($1, $2)
        RETURNING id, project_id, tw_ids
        "#,
        project_id,
        tw_ids.as_deref()
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_row)
}

pub async fn get_all_raids(
    pool: &PgPool,
) -> Result<Vec<Raid>> {
    let raids = sqlx::query_as!(
        Raid,
        r#"
        SELECT *
        FROM staking_raids
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(raids)
}
