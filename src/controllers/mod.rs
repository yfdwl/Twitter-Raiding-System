pub mod handlers;
pub use handlers::raids::get_raids_points;

use apistos::web::{get, post, scope, Scope};
use handlers::raids::{post_send_msg_to_channel, post_send_msg_to_owner};

pub trait Controller {
    fn get_scope(path: &str) -> Scope;
}
pub struct RaidsController;

impl Controller for RaidsController {
    fn get_scope(path: &str) -> Scope {
        scope(path)
            .service(Scope::new("").route("", get().to(get_raids_points)))
            .service(
                Scope::new("/msg")
                .route("/owner", post().to(post_send_msg_to_owner))
                .route("/channel", post().to(post_send_msg_to_channel))
            )
    }
}