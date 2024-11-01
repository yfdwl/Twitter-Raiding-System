use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::{PgPool, Result};
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
    pub total_pt: Option<BigDecimal>,
}

#[derive(Debug, Serialize)]
pub struct LeaderBoard {
    user_id: Option<String>,
    total_pt: Option<BigDecimal>,
    rank: Option<i64>,
}

pub async fn create_new_raid(
    pool: &PgPool,
    project_id: String,
    tw_id: String,
    user_id: String,
    pt_by_following: BigDecimal,
    pt_by_liking: BigDecimal,
    pt_by_retweet: BigDecimal,
    pt_by_replying: BigDecimal,
    total_pt: BigDecimal,
) -> Result<Raids> {
    let id = Uuid::new_v4();

    let inserted_row = sqlx::query_as!(
        Raids,
        r#"
        INSERT INTO staking_raids (id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt
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
    .fetch_one(pool)
    .await?;

    Ok(inserted_row)
}

pub async fn get_all_raids(
    pool: &PgPool
) -> Result<Vec<Raids>> {
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

pub async fn get_user_by_id(
    pool: &PgPool,
    project_id: String,
    tw_id: String,
    user_id: String,
) -> Result<Option<Raids>> {
    let users = sqlx::query_as!(
        Raids,
        r#"
        SELECT id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt
        FROM staking_raids
        WHERE project_id = $1 AND tw_id = $2 AND user_id = $3
        "#,
        project_id,
        tw_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(users)
}

pub async fn get_raids_by_project_and_user_id(
    pool: &PgPool,
    project_id: String,
    user_id: String
) -> Result<Vec<Raids>> {
    let users = sqlx::query_as!(
        Raids,
        r#"
        SELECT id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt
        FROM staking_raids
        WHERE project_id = $1 AND user_id = $2
        "#,
        project_id,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn get_leaderboard_by_project_id(
    pool: &PgPool,
    project_id: String,
) -> Result<Vec<LeaderBoard>> {
    let leader_board = sqlx::query_as!(
        LeaderBoard,
        r#"
        SELECT 
            user_id, 
            total_pt, 
            RANK() OVER (ORDER BY total_pt DESC) AS rank
        FROM 
            staking_raids
        WHERE 
            project_id = $1
        ORDER BY 
            total_pt DESC
        "#,
        project_id
    )
    .fetch_all(pool)
    .await?;

    Ok(leader_board)
}

pub async fn update_pt_by_following(
    pool: &PgPool,
    project_id: String,
    user_id: String,
    new_pt_by_following: BigDecimal,
) -> Result<Raids> {
    let updated_row = sqlx::query_as!(
        Raids,
        r#"
        UPDATE staking_raids
        SET pt_by_following = $1
        WHERE project_id = $2 AND user_id = $3
        RETURNING id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt
        "#,
        new_pt_by_following,
        project_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_row)
}

pub async fn update_pt_by_retweet(
    pool: &PgPool,
    project_id: String,
    tw_id: String,
    user_id: String,
    new_pt_by_retweet: BigDecimal,
) -> Result<Raids> {
    let updated_row: Raids = sqlx::query_as!(
        Raids,
        r#"
        UPDATE staking_raids
        SET pt_by_replying = $1
        WHERE project_id = $2 AND tw_id = $3 AND user_id = $4
        RETURNING id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt
        "#,
        new_pt_by_retweet,
        project_id,
        tw_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_row)
}

pub async fn update_pt_by_replying(
    pool: &PgPool,
    project_id: String,
    tw_id: String,
    user_id: String,
    new_pt_by_replying: BigDecimal,
) -> Result<Raids> {
    let updated_row = sqlx::query_as!(
        Raids,
        r#"
        UPDATE staking_raids
        SET pt_by_replying = $1
        WHERE project_id = $2 AND tw_id = $3 AND user_id = $4
        RETURNING id, project_id, tw_id, user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt
        "#,
        new_pt_by_replying,
        project_id,
        tw_id,
        user_id
    )
    .fetch_one(pool) 
    .await?;

    Ok(updated_row)
}

pub async fn get_user_points_by_raid(
    pool: &PgPool,
    project_id: String,
    user_id: String
) -> Result<Option<BigDecimal>> {
    let user_points = sqlx::query_scalar!(
        r#"
        SELECT SUM(total_pt)
        FROM staking_raids
        WHERE project_id = $1 AND user_id = $2
        "#,
        project_id,
        user_id
    )
    .fetch_one(pool) 
    .await?;

    Ok(user_points)
}