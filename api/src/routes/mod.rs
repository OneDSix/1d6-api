pub mod assets;
pub mod defaults;
pub mod errors;
pub mod v1;

use actix_web::web::{get, scope, Data, ServiceConfig};
use actix_analytics::Analytics;

use defaults::{index_get, default_ratelimit, default_cors};

use crate::AppState;

pub fn root_config(cfg: &mut ServiceConfig, state: &Data<AppState>) {
	let analytics_key = state.secrets.get("ANALYTICS_KEY").clone().unwrap();

    cfg.service(
        scope("")
            .wrap(default_cors())
            .wrap(default_ratelimit())
			.wrap(Analytics::new(analytics_key))

            // All empty addresses end with a "/", so only handle "example.com/"
            .route("/", get().to(index_get))
			// Everthing Else
			.configure(assets::config)
			.configure(v1::config)
    );
}
