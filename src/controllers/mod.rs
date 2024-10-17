pub mod handlers;
pub use handlers::raids::get_raids_points;

use apistos::web::{get, scope, Scope};

pub trait Controller {
    fn get_scope(path: &str) -> Scope;
}
pub struct RaidsController;

impl Controller for RaidsController {
    fn get_scope(path: &str) -> Scope {
        scope(path)
            .service(Scope::new("/").route("", get().to(get_raids_points))) // localhost:8000/raids/
    }
}