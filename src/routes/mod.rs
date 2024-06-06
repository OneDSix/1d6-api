pub mod v1;
pub mod responses;

use actix_web::web::{scope, ServiceConfig};

use crate::utils::cors::default_cors;

pub fn root_config(cfg: &mut ServiceConfig) {
	cfg.service(
        scope("")
            .wrap(default_cors())
            .service(responses::index_get)
    );
}
