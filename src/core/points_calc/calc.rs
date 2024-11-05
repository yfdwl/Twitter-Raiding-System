use tokio::time::{interval, Duration};
use crate::{
    config::Action,
    core::{
        db::PgDb,
        staking_raids::queries::get_all_raids,
        staking_raids_user::queries::{
            add_new_user_with_following_pts, add_new_user_with_tw_retweet_pts, get_all_raid_users, get_raid_user_by_project_and_user, update_raid_user_following_pts, update_raid_user_tw_replying_pts, update_raid_user_tw_retweet_pts
        },
        xapis::{
            following_users::get_following_users, replying_users::get_replying_users,
            retweeted_users::get_retweeted_users,
        },
    },
};
use actix_web::web::Data;
use bigdecimal::BigDecimal;

fn get_bonus_points(action: &Action) -> BigDecimal {
    match action {
        Action::Following => BigDecimal::from(10),
        Action::Retweet => BigDecimal::from(5),
        Action::Like => BigDecimal::from(3),
        Action::Reply => BigDecimal::from(4),
    }
}

async fn add_following_points(
    db: Data<PgDb>,
    user_id: String,
    project_id: String
) {
    let followers = get_following_users(&project_id).await;

    if followers.contains(&user_id) {
        // Attempt to fetch the raid user
        let raid_user = match get_raid_user_by_project_and_user(
            db.pool(),
            user_id.clone(),  // clone beta_user for the query
            project_id.clone(), // clone project_id for the query
        )
        .await
        {
            Ok(user) => user,
            Err(e) => {
                eprintln!("Error fetching raid user by project and user ID: {:?}", e);
                return;
            }
        };

        match raid_user {
            Some(raid_user) => {
                if let Err(e) = update_raid_user_following_pts(
                    db.pool(),
                    raid_user.user_id,
                    raid_user.project_id,
                    get_bonus_points(&Action::Following),
                )
                .await
                {
                    eprintln!("Failed to update following points. Error: {:?}", e);
                }
            }
            None => {
                if let Err(e) = add_new_user_with_following_pts(
                    db.pool(),
                    user_id.clone(),
                    project_id.clone(),
                    get_bonus_points(&Action::Following),
                )
                .await
                {
                    eprintln!("Failed to add new raid user with following points for beta user. Error: {:?}", e);
                }
            }
        }
    }
}

async fn add_tweet_replying_points(
    db: Data<PgDb>,
    user_id: String,
    project_id: String,
    tweet_id: String,
) {
    let replying_users = get_replying_users(&tweet_id).await;

    if replying_users.contains(&user_id) {
        let raid_user = match get_raid_user_by_project_and_user(
            db.pool(),
            user_id.clone(),    // clone beta_user for the query
            project_id.clone(), // clone project_id for the query
        )
        .await
        {
            Ok(user) => user,
            Err(e) => {
                eprintln!("Error fetching raid user by project and user ID: {:?}", e);
                return;
            }
        };
        match raid_user {
            Some(raid_user) => {
                if let Err(e) = update_raid_user_tw_replying_pts(
                    db.pool(),
                    raid_user.user_id,
                    raid_user.project_id,
                    get_bonus_points(&Action::Reply),
                )
                .await
                {
                    eprintln!("Failed to update following points. Error: {:?}", e);
                }
            }
            None => {
                if let Err(e) = add_new_user_with_following_pts(
                    db.pool(),
                    user_id.clone(),
                    project_id.clone(),
                    get_bonus_points(&Action::Reply),
                )
                .await
                {
                    eprintln!("Failed to add new raid user with following points for beta user. Error: {:?}", e);
                }
            }
        }
    }
}

async fn add_tw_retweet_points(
    db: Data<PgDb>,
    user_id: String,
    project_id: String,
    tweet_id: String,
) {
    let retweet_users = get_retweeted_users(&tweet_id).await;

    if retweet_users.contains(&user_id) {
        let raid_user = match get_raid_user_by_project_and_user(
            db.pool(),
            user_id.clone(),    // clone beta_user for the query
            project_id.clone(), // clone project_id for the query
        )
        .await
        {
            Ok(user) => user,
            Err(e) => {
                eprintln!("Error fetching raid user by project and user ID: {:?}", e);
                return;
            }
        };
        match raid_user {
            Some(raid_user) => {
                if let Err(e) = update_raid_user_tw_retweet_pts(
                    db.pool(),
                    raid_user.user_id,
                    raid_user.project_id,
                    get_bonus_points(&Action::Retweet),
                )
                .await
                {
                    eprintln!("Failed to update following points. Error: {:?}", e);
                }
            }
            None => {
                if let Err(e) = add_new_user_with_tw_retweet_pts(
                    db.pool(),
                    user_id.clone(),
                    project_id.clone(),
                    get_bonus_points(&Action::Retweet),
                )
                .await
                {
                    eprintln!("Failed to add new raid user with following points for beta user. Error: {:?}", e);
                }
            }
        }
    }
}

async fn add_points(db: Data<PgDb>) {
    let raids = match get_all_raids(db.pool()).await {
        Ok(raids) => raids,
        Err(e) => {
            eprintln!("Error fetching all raids: {:?}", e);
            return;
        }
    };

    let raid_users = match get_all_raid_users(db.pool()).await {
        Ok(raid_users) => raid_users,
        Err(e) => {
            eprintln!("Error fetching all raid users: {:?}", e);
            return;
        }
    };

    for raid in &raids {
        for raid_user in &raid_users {
            if raid.project_id == raid_user.project_id {
                let _ = add_following_points(db.clone(), raid_user.user_id.clone(), raid.project_id.clone());
                if let Some(tw_ids) = &raid.tw_ids {
                    for tw_id in tw_ids {
                        let _= add_tweet_replying_points(db.clone(), raid_user.user_id.clone(), raid.project_id.clone(), tw_id.clone());
                        let _= add_tw_retweet_points(db.clone(), raid_user.user_id.clone(), raid.project_id.clone(), tw_id.clone());
                    }
                }
            }
        }
    }
}

pub async fn run_points_calc_system(db: Data<PgDb>) {
    let mut interval = interval(Duration::from_secs(1800));

    loop {
        interval.tick().await;

        let db_clone = db.clone();

        tokio::spawn(async move {
            let _= add_points(db_clone);
        });
    }
}
