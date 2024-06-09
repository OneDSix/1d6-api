// Quick note for anyone concerned, telemetry is disabled by default, always.
// It's an opt-in thing for public and anonymous statistics, mostly for data nerds like me.
// Most of the data is here visible at `/v1/stats`, specifically the "online" part.
// If you're looking to the endpoints specific to servers, that would be `game_services.rs`, not here.
// - The Color Blurple

use std::collections::HashSet;

use actix_web::{
    http::header,
    web::{scope, ServiceConfig},
    HttpRequest,
};
use lazy_static::lazy_static;

use client::post_client;
use regex::Regex;
use server::post_server;
use website::{get_website, post_website};

use crate::routes::defaults::{default_cors, default_ratelimit};

const GDPR_LANGUAGES: [&str; 35] = [
    // EU and EEA countries
    "AT", // Austria
    "BE", // Belgium
    "BG", // Bulgaria
    "HR", // Croatia
    "CY", // Cyprus
    "CZ", // Czechia
    "DK", // Denmark
    "EE", // Estonia
    "FI", // Finland
    "FR", // France
    "DE", // Germany
    "GR", // Greece
    "HU", // Hungary
    "IS", // Iceland
    "IE", // Ireland
    "IT", // Italy
    "LV", // Latvia
    "LI", // Liechtenstein
    "LT", // Lithuania
    "LU", // Luxembourg
    "MT", // Malta
    "NL", // Netherlands
    "NO", // Norway
    "PL", // Poland
    "PT", // Portugal
    "RO", // Romania
    "SK", // Slovakia
    "SI", // Slovenia
    "ES", // Spain
    "SE", // Sweden
    "CH", // Switzerland
    "GB", // Great Britain
    // GDPR-Adjacent Countries
    "CN", // China
    "RU", // Russia
    "CA", // Canada

	// If more should be added, make a PR and link the related legislation and it'll be accepted asap.
];

lazy_static! {
    static ref GDPR_LANGUAGE_SET: HashSet<&'static str> = GDPR_LANGUAGES.iter().cloned().collect();
    static ref LANG_REGEX: Regex = Regex::new(r"([a-z]{2})(?:-[A-Z]{2})?(?:;q=\d+\.\d+)?").unwrap();
}

fn is_gdpr(req: &HttpRequest) -> bool {
    // TODO: rewrite this absurdity
    if let Some(lang_header) = req.headers().get("Accept-Language") {
        if let Ok(lang_str) = lang_header.to_str() {
            for cap in LANG_REGEX.captures_iter(lang_str) {
                if let Some(captured_string) = cap.get(1) {
                    let uppercased_string = captured_string.as_str().to_uppercase();
                    if GDPR_LANGUAGE_SET.contains(&uppercased_string[..]) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("telem")
            .wrap(default_cors())
            .wrap(default_ratelimit())
            .service(post_server)
            .service(post_client)
            .service(post_website)
            .service(get_website),
    );
}

mod server {
    use actix_web::{
        post,
        web::{Data, Json},
        HttpRequest, HttpResponse, Result,
    };
    use serde::{Deserialize, Serialize};

    use crate::{
        routes::{defaults::SUCCESS, errors::ApiErrors},
        AppState,
    };

    use super::is_gdpr;

    #[derive(Deserialize, Serialize)]
    struct ServerActivity {
        uptime: Option<u64>,
        player_count: Option<u32>,
    }

    #[post("/server")]
    async fn post_server<'a>(
        sent_data: Json<ServerActivity>,
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

mod client {
    use actix_web::{
        post,
        web::{Data, Json},
        HttpRequest, HttpResponse, Result,
    };
    use serde::{Deserialize, Serialize};

    use crate::{
        routes::{defaults::SUCCESS, errors::ApiErrors},
        AppState,
    };

    use super::is_gdpr;

    #[derive(Deserialize, Serialize)]
    struct ClientActivity {
        playtime_past_week: Option<u64>,
        playtime_ever: Option<u64>,
        mod_count: Option<u32>,
    }

    #[post("/client")]
    async fn post_client<'a>(
        sent_data: Json<ClientActivity>,
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
        routes::{defaults::SUCCESS, errors::ApiErrors},
        AppState,
    };

    use super::is_gdpr;

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

    #[get("/web")]
    async fn get_website<'a>(
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
