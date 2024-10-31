use bigdecimal::BigDecimal;
use crate::config::Action;

pub fn get_bonus_points(action: &Action) -> BigDecimal {
    match action {
        Action::Following => BigDecimal::from(10),
        Action::Retweet => BigDecimal::from(5),
        Action::Like => BigDecimal::from(3),
        Action::Reply => BigDecimal::from(4),
    }
}