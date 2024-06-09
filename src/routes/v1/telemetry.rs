// Quick note for anyone concerned, telemetry is disabled by default, always.
// It's an opt-in thing for public and anonymous statistics, mostly for data nerds like me.
// Most of the data is here visible at `/v1/stats`, specifically the "online" part.
//
// If you're looking to the endpoints specific to servers, that would be `game_services.rs`, not here.
//
// - The Color Blurple

use actix_web::web::{get, scope, ServiceConfig};

use crate::{routes::responses::index_get, utils::{cors::default_cors, ratelimit::default_ratelimit}};

pub fn config(cfg: &mut ServiceConfig) {
	cfg.service(
		scope("telem")
            .wrap(default_cors())
			.wrap(default_ratelimit())

			// Handle both "/v1" and "/v1/" as they can be easily mixed up
			.route("", get().to(index_get))
			.route("/", get().to(index_get))
	);
}
