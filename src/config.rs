pub const BONUS_POINTS_FOR_FOLLOWING: i32 = 10;
pub const BONUS_POINTS_FOR_RETWEET: i32 = 5;
pub const BONUS_POINTS_FOR_LIKE: i32 = 2;
pub const BONUS_POINTS_FOR_REPLY: i32 = 3;

pub enum Action {
    Following,
    Retweet,
    Like,
    Reply,
}

#[derive(Debug, Clone)]
pub struct UserPoints {
    pub user_id: String,
    pub points: i32,
}

impl UserPoints {
    pub fn new(user_id: String, points: i32) -> Self {
        Self { user_id, points }
    }

    pub fn points(&self) -> i32 {
        self.points
    }

    pub fn set_points(&mut self, points: i32) {
        self.points = points;
    }

    pub fn add_points(&mut self, points: i32) {
        self.points += points;
    }
}