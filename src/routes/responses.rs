use actix_web::HttpResponse;
use serde_json::json;

use crate::utils::validation::dates::SpecialDates;

use super::errors::ApiError;

/// A constant link to the API docs. Added to the end of some requests.
pub const DOCS_LINK: &str = "https://github.com/OneDSix/1d6-api/wiki";

/// Send this whenever an impossible condition is possible.
pub const IMPOSSIBLE_CONDITION: &'static str = "An impossible condition has been reached. Please make an issue with whatever you did to cause this on the API's GitHub here: https://github.com/OneDSix/1d6-api";

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

pub async fn not_found() -> HttpResponse {
	let error_data = ApiError {
		error: "not_found",
		description: "the requested route does not exist".to_string()
	};

    HttpResponse::NotFound().json(error_data)
}
