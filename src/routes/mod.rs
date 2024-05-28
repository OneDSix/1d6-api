pub mod v1;
pub mod errors;
pub mod test;
pub mod responses;

use actix_web::web::{scope, ServiceConfig};

use crate::utils::cors::default_cors;

pub fn root_config(cfg: &mut ServiceConfig) {
	cfg.service(
		scope("test")
		.wrap(default_cors())
		.configure(test::config)
	);
	cfg.service(
        scope("")
            .wrap(default_cors())
            .service(responses::index_get)
    );
}
