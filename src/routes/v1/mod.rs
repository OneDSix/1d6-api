use actix_web::web::{scope, ServiceConfig};

use crate::utils::cors::default_cors;

use super::responses::index_get;

pub mod mods;
pub mod users;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
		scope("/v1")
			.wrap(default_cors())
			.service(index_get)
			.configure(users::config)
			.configure(mods::config)
	);
}
