use bigdecimal::BigDecimal;
use sqlx::{PgPool, Result};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Raids {
    pub id: Uuid,
    pub project_id: Option<String>,
    pub tw_id: Option<String>,
    pub user_id: Option<String>,
    pub pt_by_following: Option<BigDecimal>,
    pub pt_by_liking: Option<BigDecimal>,
    pub pt_by_retweet: Option<BigDecimal>,
    pub pt_by_replying: Option<BigDecimal>,
    pub total_pt: Option<BigDecimal>
}

pub async fn get_all_raids(pool: &PgPool) -> Result<Vec<Raids>> {
    let users = sqlx::query_as!(
        Raids,
        r#"
        SELECT id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt
        FROM staking_raids
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn create_new_raid(
    pool: &PgPool,
    project_id: Option<String>,
    tw_id: Option<String>,
    user_id: Option<String>,
    pt_by_following: Option<BigDecimal>,
    pt_by_liking: Option<BigDecimal>,
    pt_by_retweet: Option<BigDecimal>,
    pt_by_replying: Option<BigDecimal>,
    total_pt: Option<BigDecimal>
) -> Result<Uuid> {
    // Generate a new UUID for the new raid entry
    let id = Uuid::new_v4();

    // Insert the new row into the `staking_raids` table
    sqlx::query!(
        r#"
        INSERT INTO staking_raids (id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        id,
        project_id,
        tw_id,
        user_id,
        pt_by_following,
        pt_by_liking,
        pt_by_retweet,
        pt_by_replying,
        total_pt
    )
    .execute(pool)
    .await?;

    // Return the UUID of the newly inserted row
    Ok(id)
}

pub async fn update_pt_by_following(
    pool: &PgPool,
    user_id: Uuid,
    new_pt_by_following: Option<BigDecimal>,
) -> Result<u64> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE staking_raids
        SET pt_by_following = $1
        WHERE id = $2
        "#,
        new_pt_by_following,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected(); // Get the number of rows affected by the update

    Ok(rows_affected) // Return the number of rows updated
}

pub async fn update_pt_by_liking(
    pool: &PgPool,
    user_id: Uuid,
    new_pt_by_liking: Option<BigDecimal>,
) -> Result<u64> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE staking_raids
        SET pt_by_liking = $1
        WHERE id = $2
        "#,
        new_pt_by_liking,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected(); // Get the number of rows affected by the update

    Ok(rows_affected) // Return the number of rows updated
}

pub async fn update_pt_by_retweet(
    pool: &PgPool,
    user_id: Uuid,
    new_pt_by_retweet: Option<BigDecimal>,
) -> Result<u64> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE staking_raids
        SET pt_by_retweet = $1
        WHERE id = $2
        "#,
        new_pt_by_retweet,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected(); // Get the number of rows affected by the update

    Ok(rows_affected) // Return the number of rows updated
}

pub async fn update_pt_by_replying(
    pool: &PgPool,
    user_id: Uuid,
    new_pt_by_replying: Option<BigDecimal>,
) -> Result<u64> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE staking_raids
        SET pt_by_replying = $1
        WHERE id = $2
        "#,
        new_pt_by_replying,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected(); // Get the number of rows affected by the update

    Ok(rows_affected) // Return the number of rows updated
}

pub async fn update_total_pt(
    pool: &PgPool,
    user_id: Uuid,
    total_pt: Option<BigDecimal>,
) -> Result<u64> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE staking_raids
        SET total_pt = $1
        WHERE id = $2
        "#,
        total_pt,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected(); // Get the number of rows affected by the update

    Ok(rows_affected) // Return the number of rows updated
}

pub async fn get_raids_by_id(pool: &PgPool, user_id: Uuid) -> Result<Raids> {
    
    
    let user = sqlx::query_as!(
        Raids,
        r#"
        SELECT id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt
        FROM staking_raids
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}