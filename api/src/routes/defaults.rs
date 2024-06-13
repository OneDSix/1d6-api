use std::{num::NonZeroU32, sync::Arc};

use governor::{middleware::StateInformationMiddleware, Quota};
use actix_web::{HttpRequest, HttpResponse};
use actix_cors::Cors;
use governor::RateLimiter;
use serde_json::{json, Value};
use lazy_static::lazy_static;

use crate::utils::{dates::SpecialDates, ratelimit::{KeyedRateLimiter, RateLimit}};
use super::errors::ApiErrors;

pub fn default_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allow_any_method()
        .max_age(3600)
        .send_wildcard()
}

pub fn default_ratelimit() -> RateLimit {
    RateLimit(Arc::clone(&LIMITER))
}

/// A constant link to the API docs. Added to the end of some requests.
pub const DOCS_LINK: &'static str = "https://github.com/OneDSix/1d6-api/wiki";

/// Send this whenever an impossible condition is possible.
pub const IMPOSSIBLE_CONDITION: &'static str = "An impossible condition has been reached. Please make an issue with whatever you did to cause this on the API's GitHub here: https://github.com/OneDSix/1d6-api";

lazy_static!{
	/// A frequently used "Ok!" text used for when something was successful.<br>
	/// Use `&*SUCCESS` to access it.
	pub static ref SUCCESS: Value = json!({"success":true});

	/// A basic 3 request per second ratelimiter.<br>
	/// Its so restrictive because I don't want shuttle getting on my case if 1D6 get traction.
	static ref LIMITER: KeyedRateLimiter = Arc::new(
        RateLimiter::keyed(Quota::per_minute(NonZeroU32::new(60).unwrap()))
            .with_middleware::<StateInformationMiddleware>(),
    );
}

pub async fn index_get() -> HttpResponse {

	let message: &str;
	match SpecialDates::is_special() {
		SpecialDates::OneDSixBirthday => message = "Happy Birthday 1D6!",
		SpecialDates::AprilFools => message = "April Fools :P",
		SpecialDates::Halloween => message = "BOO.",
		SpecialDates::Christmas => message = "Merry Christmas...",
		SpecialDates::NewYears => message = "...and a Happy New Year!",
		SpecialDates::NotSpecial => message = "Welcome traveler!",
	}

    let index_data = json!({
		"message": message,
        "api_version": "1",
        "documentation": DOCS_LINK
    });

	let mut returns = HttpResponse::Ok().json(&index_data);
	if SpecialDates::is_special() == SpecialDates::AprilFools {returns = HttpResponse::ImATeapot().json(&index_data);}
	returns
}

pub async fn not_found(req: HttpRequest) -> HttpResponse {
	ApiErrors::NotFound((&req.path()).to_string(), (&req.method()).to_string()).error_response()
}

pub async fn robots_file(_req: HttpRequest) -> HttpResponse {
	HttpResponse::Ok().body("
		User-agent: *\n
		Disallow: /
	")
}
