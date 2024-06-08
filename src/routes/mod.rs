pub mod errors;
pub mod responses;
pub mod v1;

use actix_web::web::{get, scope, ServiceConfig};
use responses::index_get;

use crate::utils::{cors::default_cors, ratelimit::default_ratelimit};

pub fn root_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
            .wrap(default_cors())
            .wrap(default_ratelimit())
            // all empty addresses end with a "/", so only handle "example.com/"
            .route("/", get().to(index_get)),
    );
}
