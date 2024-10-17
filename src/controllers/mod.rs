pub mod handlers {
    pub mod hey;
}

use apistos::web::{get, scope, Scope};

pub trait Controller {
    fn get_scope(path: &str) -> Scope;
}
pub struct RaidsController;

impl Controller for RaidsController {
    fn get_scope(path: &str) -> Scope {
        scope(path)
        .service(Scope::new("/hey").route("", get().to(handlers::hey::manual_hello)))
    }
}