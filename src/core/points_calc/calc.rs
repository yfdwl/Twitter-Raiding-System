use crate::config::{
    Action, UserPoints, BONUS_POINTS_FOR_FOLLOWING, BONUS_POINTS_FOR_LIKE, BONUS_POINTS_FOR_REPLY,
    BONUS_POINTS_FOR_RETWEET,
};

pub fn get_bonus_points(action: &Action) -> i32 {
    match action {
        Action::Following => BONUS_POINTS_FOR_FOLLOWING,
        Action::Retweet => BONUS_POINTS_FOR_RETWEET,
        Action::Like => BONUS_POINTS_FOR_LIKE,
        Action::Reply => BONUS_POINTS_FOR_REPLY,
    }
}

pub fn set_beta_user_bonus_points(
    mut beta_user_ids: Vec<UserPoints>,
    action_user_ids: Vec<String>,
    action: Action,
) -> Vec<UserPoints> {
    let bonus_points = get_bonus_points(&action);

    for beta_user in &mut beta_user_ids {
        if action_user_ids.contains(&beta_user.user_id) {
            beta_user.add_points(bonus_points);
        }
    }

    beta_user_ids
}
