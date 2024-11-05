pub mod handlers;

use apistos::web::{get, post, scope, Scope};
use handlers::raids::{
    create_new_project, get_points, join_project, post_send_msg_to_channel, post_send_msg_to_owner, track_leaderboard,
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
            .service(Scope::new("/create").route("", post().to(create_new_project)))
            .service(
                Scope::new("/project/join")
                    .route("/{user_id}/{project_id}", get().to(join_project)),
            )
            .service(Scope::new("/points").route("/{user_id}/{project_id}", get().to(get_points)))
            .service(Scope::new("/leaderboard").route("/{project_id}", get().to(track_leaderboard)))
    }
}
