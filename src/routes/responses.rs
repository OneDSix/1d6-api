use actix_web::{get, http::StatusCode, HttpResponse};
use serde_json::json;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::utils::dates::special_days::{SpecialDates::*, is_special};

/// A constant link to the API docs. Added to the end of some requests.
pub const DOCS_LINK: &str = "https://github.com/OneDSix/1d6-api/wiki";

#[derive(Serialize, Deserialize)]
pub struct ApiError<'a> {
    pub error: &'a str,
    pub description: String
}

#[allow(unused)]
#[derive(Debug, Error)]
pub(crate) enum ApiErrors {
	#[error("An unknown error occurred.")]
	UnknownError(String),
    #[error("Error while validating input: {0}")]
    Validation(String),
	#[error("Could not find: {0}")]
    NotFound(String),
	#[error("{0}")]
	DatabaseError(String),
	#[error("Invalid Username/Password Combo")]
	InvalidCredentials(String)
}

impl ApiErrors {
    pub fn as_api_error<'a>(&self) -> ApiError<'a> {
        ApiError {
            error: match self {
				ApiErrors::UnknownError(..) => "unknown_error",
                ApiErrors::Validation(..) => "invalid_input",
				ApiErrors::NotFound(..) => "not_found",
				ApiErrors::DatabaseError(..) => "database_error",
				ApiErrors::InvalidCredentials(..) => "invalid_credentials",
            },
            description: self.to_string(),
        }
    }
}

impl actix_web::ResponseError for ApiErrors {
    fn status_code(&self) -> StatusCode {
        match self {
			ApiErrors::UnknownError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::Validation(..) => StatusCode::BAD_REQUEST,
			ApiErrors::NotFound(..) => StatusCode::NOT_FOUND,
			ApiErrors::DatabaseError(..) => StatusCode::INTERNAL_SERVER_ERROR,
			ApiErrors::InvalidCredentials(..) => StatusCode::UNAUTHORIZED
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}

#[get("/")]
pub async fn index_get() -> HttpResponse {

	let message: &str;
	match is_special() {
		OneDSixBirthday => message = "Happy Birthday 1D6!",
		AprilFools => message = "April Fools :P",
		Halloween => message = "BOO.",
		Christmas => message = "Merry Christmas...",
		NewYears => message = "...and a Happy New Year!",
		NotSpecial => message = "Welcome traveler!",
	}

    let index_data = json!({
		"message": message,
        "api_version": "1",
        "documentation": DOCS_LINK
    });

	let mut returns = HttpResponse::Ok().json(&index_data);
	if is_special() == AprilFools {returns = HttpResponse::ImATeapot().json(&index_data);}
	returns
}

pub async fn not_found() -> HttpResponse {
	let error_data = ApiError {
		error: "not_found",
		description: "the requested route does not exist".to_string()
	};

    HttpResponse::NotFound().json(error_data)
}
