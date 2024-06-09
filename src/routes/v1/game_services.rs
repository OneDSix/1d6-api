use actix_web::web::{get, scope, ServiceConfig};

use crate::{
    routes::responses::index_get,
    utils::{cors::default_cors, ratelimit::default_ratelimit},
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("gs")
            .wrap(default_cors())
            .wrap(default_ratelimit())
            // Handle both "/v1" and "/v1/" as they can be easily mixed up
            .route("", get().to(index_get))
            .route("/", get().to(index_get)),
    );
}
