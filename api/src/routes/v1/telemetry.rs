// Quick note for anyone concerned, telemetry is disabled by default, always.
// It's an opt-in thing for public and anonymous statistics, mostly for data nerds like me.
// Most of the data is here visible at `/v1/stats`, specifically the "online" part.
//
// If you're looking to the endpoints specific to servers, that would be `game_services.rs`, not here.
// GDPR related stuff was moved to `utils::gdpr.rs` as to not clutter this file more than it needs.
//
// - The Color Blurple

use actix_web::web::{scope, ServiceConfig};

use activity::post_activity;
use website::post_website;

use crate::{routes::defaults::{default_cors, default_ratelimit}, utils::gdpr::GDPRTransformer};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("telem")
            .wrap(default_cors())
            .wrap(default_ratelimit())
			.wrap(GDPRTransformer)

            .service(post_activity)
            .service(post_website)
    );
}

mod activity {
    use actix_web::{
        post, web::{Data, Json}, HttpResponse, Result
    };
    use serde::{Deserialize, Serialize};

    use crate::{
        routes::{defaults::SUCCESS, errors::ApiErrors}, AppState, ONLINE
    };

    #[derive(Deserialize, Serialize)]
    struct ServerActivity {
		ping_reason: String,
		client_type: String,
        uptime: f32,
        player_count: i32,
		mod_count: i32,
    }

    #[post("/activity")]
    async fn post_activity<'a>(
        sent_data: Json<ServerActivity>,
        state: Data<AppState>,
    ) -> Result<HttpResponse, ApiErrors<'a>> {

		let _ = sqlx::query_as(
			"
			INSERT INTO activity (ping_reason, client_type, uptime, player_count, mod_count)
			VALUES (ping_reason, client_type, uptime, player_count, mod_count)
			"
		)
		.bind(&sent_data.ping_reason).bind(&sent_data.client_type).bind(&sent_data.uptime).bind(&sent_data.player_count).bind(&sent_data.mod_count)
		.fetch_one(&state.pool)
		.await
		.map_err(|e| ApiErrors::DatabaseError(e.to_string()))?;

		match sent_data.ping_reason.as_str() {
			"startup" => {
				match sent_data.client_type.as_str() {
					"proxy" => ONLINE.change_proxies(1),
					"client" => ONLINE.change_client(1),
					"server" => ONLINE.change_servers(1),
					"launcher" => ONLINE.change_launchers(1),
					_ => ONLINE.change_unknown(1),
				}
			},
			"shutdown" => {
				match sent_data.client_type.as_str() {
					"proxy" => ONLINE.change_proxies(-1),
					"client" => ONLINE.change_client(-1),
					"server" => ONLINE.change_servers(-1),
					"launcher" => ONLINE.change_launchers(-1),
					_ => ONLINE.change_unknown(-1),
				}
			},
			"heartbeat" | _ => {
				// Ignore Heartbeats and Unknowns
			}
		}

		Ok(HttpResponse::Created().json(&*SUCCESS))
    }
}

/// It should be noted that this covers both the website and official launcher,
/// as the launcher just loads and asks for access to certain pages of the website.
/// This just records what device and browser you use, and the React Web Vitals specs, then saves them for later.
/// Its literally just to see if React is being slow and buggy, and if it is, where and on what.
mod website {
    use actix_web::{
        http::header, post, web::{Data, Json}, HttpRequest, HttpResponse, Result
    };
    use serde::{Deserialize, Serialize};

    use crate::{
        routes::{defaults::SUCCESS, errors::ApiErrors}, AppState
    };

    #[derive(Serialize, Deserialize)]
    struct WebVitals {
		// The names are displayed here, look them up for more info.
        cls: f32, // Cumulative Layout Shift
        fcp: f32, // First Contentful Paint
        fid: f32, // First Input Delay
        inp: f32, // Interaction to Next Paint
        lcp: f32, // Largest Contentful Paint
        ttfb: f32, // Time To First Bit
    }

    #[post("/web")]
    async fn post_website<'a>(
        sent_data: Json<WebVitals>,
        req: HttpRequest,
        state: Data<AppState>,
    ) -> Result<HttpResponse, ApiErrors<'a>> {

		let db_user_agent: &str;
		match &req.headers().get(header::USER_AGENT) {
			Some(agent) => db_user_agent = agent.to_str()
				.map_err(|e| ApiErrors::UnknownError(e.to_string()))?,
			None => db_user_agent = "User Agent Not Received"
		}

		let _ = sqlx::query_as(
			"
			INSERT INTO webvitals (cls, fcp, fid, inp, lcp, ttfb, user_agent)
			VALUES (cls, fcp, fid, inp, lcp, ttfb, user_agent)
			"
		)
		.bind(&sent_data.cls).bind(&sent_data.fcp).bind(&sent_data.fid).bind(&sent_data.inp).bind(&sent_data.lcp).bind(&sent_data.ttfb).bind(db_user_agent)
		.fetch_one(&state.pool)
		.await
		.map_err(|e| ApiErrors::DatabaseError(e.to_string()))?;

		Ok(HttpResponse::Created().json(&*SUCCESS))
    }
}
