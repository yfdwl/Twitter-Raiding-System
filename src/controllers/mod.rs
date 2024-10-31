pub mod handlers;

use apistos::web::{get, post, scope, Scope};
use handlers::raids::{
    post_send_msg_to_channel,
    post_send_msg_to_owner,
    project_following,
    track_leaderboard,
    tweet_replying,
    tweet_retweet
};

pub trait Controller {
    fn get_scope(path: &str) -> Scope;
}
pub struct RaidsController;

impl Controller for RaidsController {
    fn get_scope(path: &str) -> Scope {
        scope(path)
            .service(
                Scope::new("/msg")
                    .route("/owner", post().to(post_send_msg_to_owner))
                    .route("/channel", post().to(post_send_msg_to_channel)),
            )
            .service(
                Scope::new("/track")
                    .route("/leaderboarder/{project_id}", get().to(track_leaderboard)),
            )
            .service(
                Scope::new("/action")
                    .route(
                        "/replying/{project_id}/{tweet_id}",
                        get().to(tweet_replying),
                    )
                    .route("/retweet/{project_id}/{tweet_id}", get().to(tweet_retweet))
                    .route("/following/{project_id}", get().to(project_following)),
            )
    }
}
