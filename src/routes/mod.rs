pub mod v1;
pub mod defaults;
pub mod errors;

use actix_web::web::{get, scope, ServiceConfig};

use defaults::{index_get, default_ratelimit, default_cors};

pub fn root_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
            .wrap(default_cors())
            .wrap(default_ratelimit())
            // all empty addresses end with a "/", so only handle "example.com/"
            .route("/", get().to(index_get)),
    );
}
