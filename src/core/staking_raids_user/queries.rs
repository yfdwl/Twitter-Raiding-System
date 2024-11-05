use bigdecimal::BigDecimal;
use sqlx::{PgPool, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RaidUser {
    pub user_id: String,
    pub pt_by_following: BigDecimal,
    pub pt_by_liking: BigDecimal,
    pub pt_by_retweet: BigDecimal,
    pub pt_by_replying: BigDecimal,
    pub total_pt: BigDecimal,
    pub project_id: String,
}

pub async fn add_new_user( 
    pool: &PgPool,
    user_id: String,
    project_id: String,
) -> Result<RaidUser, sqlx::Error> {
    let inserted_raid_user = sqlx::query_as!(
        RaidUser,
        r#"
        INSERT INTO staking_raids_user (user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id
        "#,
        user_id,
        BigDecimal::from(0),
        BigDecimal::from(0),
        BigDecimal::from(0),
        BigDecimal::from(0),
        BigDecimal::from(0),
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_raid_user)
}

pub async fn add_new_user_with_following_pts( 
    pool: &PgPool,
    user_id: String,
    project_id: String,
    new_following_points: BigDecimal
) -> Result<RaidUser, sqlx::Error> {
    let inserted_raid_user = sqlx::query_as!(
        RaidUser,
        r#"
        INSERT INTO staking_raids_user (user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id
        "#,
        user_id,
        BigDecimal::from(0),
        BigDecimal::from(0),
        BigDecimal::from(0),
        new_following_points,
        new_following_points,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_raid_user)
}

pub async fn add_new_user_with_tw_replying_pts( 
    pool: &PgPool,
    user_id: String,
    project_id: String,
    new_tw_replying_points: BigDecimal
) -> Result<RaidUser, sqlx::Error> {
    let inserted_raid_user = sqlx::query_as!(
        RaidUser,
        r#"
        INSERT INTO staking_raids_user (user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id
        "#,
        user_id,
        BigDecimal::from(0),
        BigDecimal::from(0),
        BigDecimal::from(0),
        new_tw_replying_points,
        new_tw_replying_points,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_raid_user)
}

pub async fn add_new_user_with_tw_retweet_pts( 
    pool: &PgPool,
    user_id: String,
    project_id: String,
    new_tw_retweet_points: BigDecimal
) -> Result<RaidUser, sqlx::Error> {
    let inserted_raid_user = sqlx::query_as!(
        RaidUser,
        r#"
        INSERT INTO staking_raids_user (user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id
        "#,
        user_id,
        BigDecimal::from(0),
        BigDecimal::from(0),
        new_tw_retweet_points,
        BigDecimal::from(0),
        new_tw_retweet_points,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_raid_user)
}

pub async fn get_all_raid_users(
    pool: &PgPool,
) -> Result<Vec<RaidUser>> {
    let raid_users = sqlx::query_as!(
        RaidUser,
        r#"
        SELECT *
        FROM staking_raids_user
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(raid_users)
}

pub async fn get_points_by_user_and_project( 
    pool: &PgPool,
    user_id: String,
    project_id: String,
) -> Result<RaidUser, sqlx::Error> {
    let points = sqlx::query_as!(
        RaidUser,
        r#"
        SELECT *
        FROM staking_raids_user
        WHERE user_id = $1 AND project_id = $2
        "#,
        user_id,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(points)
}

pub async fn get_raid_user_by_project_and_user(
    pool: &PgPool,
    user_id: String,
    project_id: String
) -> Result<Option<RaidUser>> {
    let user = sqlx::query_as!(
        RaidUser,
        r#"
        SELECT *
        FROM staking_raids_user
        WHERE user_id = $1 AND project_id = $2
        "#,
        user_id,
        project_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_leaderboard_by_project(
    pool: &PgPool,
    project_id: String,
) -> Result<Vec<RaidUser>, sqlx::Error> {
    let users = sqlx::query_as!(
        RaidUser,
        r#"
        SELECT *
        FROM staking_raids_user
        WHERE project_id = $1
        ORDER BY total_pt DESC
        LIMIT 1
        "#,
        project_id
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn update_raid_user_following_pts(
    pool: &PgPool,
    user_id: String,
    project_id: String,
    new_following_pts: BigDecimal,
) -> Result<RaidUser> {
    let updated_raid_user = sqlx::query_as!(
        RaidUser,
        r#"
        UPDATE staking_raids_user
        SET pt_by_following = $1
        WHERE user_id = $2 AND project_id = $3
        RETURNING user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id
        "#,
        new_following_pts,
        user_id,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_raid_user)
}

pub async fn update_raid_user_tw_replying_pts(
    pool: &PgPool,
    user_id: String,
    project_id: String,
    new_tw_replying_pts: BigDecimal,
) -> Result<RaidUser> {
    let updated_raid_user = sqlx::query_as!(
        RaidUser,
        r#"
        UPDATE staking_raids_user
        SET pt_by_replying = $1
        WHERE user_id = $2 AND project_id = $3
        RETURNING user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id
        "#,
        new_tw_replying_pts,
        user_id,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_raid_user)
}

pub async fn update_raid_user_tw_retweet_pts(
    pool: &PgPool,
    user_id: String,
    project_id: String,
    new_tw_retweet_pts: BigDecimal
) -> Result<RaidUser> {
    let updated_raid_user = sqlx::query_as!(
        RaidUser,
        r#"
        UPDATE staking_raids_user
        SET pt_by_retweet = $1
        WHERE user_id = $2 AND project_id = $3
        RETURNING user_id, pt_by_following, pt_by_liking, pt_by_retweet, pt_by_replying, total_pt, project_id
        "#,
        new_tw_retweet_pts,
        user_id,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_raid_user)
}

