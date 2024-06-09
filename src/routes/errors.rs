use actix_web::{http::StatusCode, HttpResponse};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct ApiError<'a> {
    pub error: &'a str,
    pub description: String
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum ApiErrors {
	#[error("An unknown error occurred.")]
	UnknownError(String),
    #[error("Error while validating input: {0}")]
    Validation(String),
	#[error("Could not find: {0}")]
    NotFound(String),
	#[error("{0}")]
	DatabaseError(String),
	#[error("Invalid Username/Password Combo")]
	InvalidCredentials(String),
	#[error("This username is taken")]
	UnavailableUsername(String),
	#[error("This username is disallowed")]
	DisallowedUsername(String),
	#[error("Sent an unhashed password")]
	UnhashedPassword(String),
	#[error("Error with Authentication Cookie")]
	AuthenticationCookieError(String),
	#[error("You are being rate-limited. Please wait {0} milliseconds. 0/{1} remaining.")]
    RateLimitError(u128, u32),
    #[error("Authentication Error: {0}")]
    CustomAuthentication(String),
	#[error("You are unauthorized to access this content.")]
	Unauthorized,
}

impl ApiErrors {
    pub fn as_api_error<'a>(&self) -> ApiError<'a> {
        ApiError {
            error: match self {
				Self::UnknownError(..) => "unknown_error",
                Self::Validation(..) => "invalid_input",
				Self::NotFound(..) => "not_found",
				Self::DatabaseError(..) => "database_error",
				Self::InvalidCredentials(..) => "invalid_credentials",
				Self::UnavailableUsername(..) => "unavailable_username",
				Self::DisallowedUsername(..) => "disallowed_username",
				Self::UnhashedPassword(..) => "unhashed_password",
				Self::AuthenticationCookieError(..) => "auth_cookie_error",
				Self::RateLimitError(..) => "rate_limited",
                Self::CustomAuthentication(..) => "custom_auth",
				Self::Unauthorized => "unauthorized"
            },
            description: self.to_string(),
        }
    }
}

impl actix_web::ResponseError for ApiErrors {
    fn status_code(&self) -> StatusCode {
        match self {
			Self::UnknownError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Validation(..) => StatusCode::BAD_REQUEST,
			Self::NotFound(..) => StatusCode::NOT_FOUND,
			Self::DatabaseError(..) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::InvalidCredentials(..) => StatusCode::UNAUTHORIZED,
			Self::UnavailableUsername(..) => StatusCode::BAD_REQUEST,
			Self::DisallowedUsername(..) => StatusCode::BAD_REQUEST,
			Self::UnhashedPassword(..) => StatusCode::BAD_REQUEST,
			Self::AuthenticationCookieError(..) => StatusCode::BAD_REQUEST,
			Self::RateLimitError(..) => StatusCode::TOO_MANY_REQUESTS,
            Self::CustomAuthentication(..) => StatusCode::UNAUTHORIZED,
			Self::Unauthorized => StatusCode::UNAUTHORIZED
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}
