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

use crate::routes::defaults::{default_cors, default_ratelimit};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("telem")
            .wrap(default_cors())
            .wrap(default_ratelimit())

            .service(post_activity)
            .service(post_website)
    );
}

mod activity {
    use actix_web::{
        post,
        web::{Data, Json},
        HttpRequest, HttpResponse, Result,
    };
    use serde::{Deserialize, Serialize};

    use crate::{
        routes::{defaults::SUCCESS, errors::ApiErrors}, utils::gdpr::is_gdpr, AppState
    };

	const FROM_TYPE: [&str; 3] = [
		"CLIENT", 		// The request is coming from a game client
		"SERVER", 		// The request is coming from a dedicated server
		"UNKNOWN", 		// The request didn't contain what it is, assuming neither
	];

    #[derive(Deserialize, Serialize)]
    struct ServerActivity {
		ping_reason: Option<String>,
		from_type: Option<String>,
        uptime: Option<u32>,
        player_count: Option<u32>,
		mod_count: Option<u32>,
    }

    #[post("/activity")]
    async fn post_activity<'a>(
        sent_data: Json<ServerActivity>,
        req: HttpRequest,
        state: Data<AppState>,
    ) -> Result<HttpResponse, ApiErrors<'a>> {
        if is_gdpr(&req) {
            return Err(ApiErrors::GDPRRegion);
        } else {
			match sent_data.ping_reason {

				// The pinger is starting up
				Some(_) => {}
				// The pinger is shutting down
				Some(_) => {}
				// Assuming this is a Heartbeat request
				None | Some(_) => {}
			};

            Ok(HttpResponse::NotImplemented().json(&*SUCCESS))
        }
    }
}

/// It should be noted that this covers both the website and launcher,
/// as the launcher just loads the website and asks for access to certain pages of the site.
/// This just records what device and browser you use, and the React Web Vitals specs, then saves them for later.
/// Its literally just to see if React is being slow and buggy.
mod website {
    use actix_web::{
        get, post,
        web::{Data, Json},
        HttpRequest, HttpResponse, Result,
    };
    use serde::{Deserialize, Serialize};

    use crate::{
        routes::{defaults::SUCCESS, errors::ApiErrors}, utils::gdpr::is_gdpr, AppState
    };

    #[derive(Serialize, Deserialize)]
    struct WebVitals {
        cls: Option<u32>, // Cumulative Layout Shift
        fcp: Option<u32>, // First Contentful Paint
        fid: Option<u32>,
        inp: Option<u32>,
        lcp: Option<u32>,  // Largest Contentful Paint
        ttfb: Option<u32>, // Time To First Bit
    }

    #[post("/web")]
    async fn post_website<'a>(
        sent_data: Json<WebVitals>,
        req: HttpRequest,
        state: Data<AppState>,
    ) -> Result<HttpResponse, ApiErrors<'a>> {
        if is_gdpr(&req) {
            return Err(ApiErrors::GDPRRegion);
        } else {
            Ok(HttpResponse::NotImplemented().json(&*SUCCESS))
        }
    }
}
