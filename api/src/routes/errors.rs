use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct ApiError<'a> {
    pub error: &'a str,
    pub description: String
}

#[allow(unused)]
#[derive(Debug, Error)]
pub enum ApiErrors<'a> {
	#[error("A dummy error to make sure this enum has a lifetime")]
	DummyError(&'a String),
	#[error("An unknown error occurred: {0}")]
	UnknownError(String),
    #[error("Error while validating input: {0}")]
    Validation(String),
	#[error("Could not find {0} with method {1}")]
    NotFound(String, String),
	#[error("{0}")]
	DatabaseError(String),
	#[error("Invalid Username/Password Combo")]
	InvalidCredentials,
	#[error("This username is taken: {0}")]
	UnavailableUsername(String),
	#[error("This username is disallowed: {0}")]
	DisallowedUsername(String),
	#[error("You sent an unhashed password, bad!")]
	UnhashedPassword,
	#[error("Error with Authentication Cookie")]
	AuthenticationCookieError,
	#[error("You are being rate-limited. Please wait {0} milliseconds. 0/{1} remaining.")]
    RateLimitError(u128, u32),
    #[error("Authentication Error: {0}")]
    AuthenticationError(String),
	#[error("You are unauthorized to access this content.")]
	Unauthorized,
    #[error("Webhook Error: {0}")]
    WebhookError(String),
	#[error("This request came from a region with GDPR or GDPR-Adjacent legislation. We're not taking an risks with telemetry in the EU, sorry! :(")]
	GDPRRegion
}

impl ApiErrors<'_> {
    pub fn as_api_error<'a>(&self) -> ApiError<'a> {
        ApiError {
            error: match self {
				Self::DummyError(..) => "dummy_error",
				Self::UnknownError(..) => "unknown_error",
                Self::Validation(..) => "invalid_input",
				Self::NotFound(..) => "not_found",
				Self::DatabaseError(..) => "database_error",
				Self::InvalidCredentials => "invalid_credentials",
				Self::UnavailableUsername(..) => "unavailable_username",
				Self::DisallowedUsername(..) => "disallowed_username",
				Self::UnhashedPassword => "unhashed_password",
				Self::AuthenticationCookieError => "auth_cookie_error",
				Self::RateLimitError(..) => "rate_limited",
                Self::AuthenticationError(..) => "custom_auth",
				Self::Unauthorized => "unauthorized",
				Self::WebhookError(..) => "webhook_error",
				Self::GDPRRegion => "gdpr_region",
            },
            description: self.to_string(),
        }
    }

	pub fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}

impl<'a> actix_web::ResponseError for ApiErrors<'_> {
    fn status_code(&self) -> StatusCode {
        match self {
			Self::DummyError(..) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::UnknownError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Validation(..) => StatusCode::BAD_REQUEST,
			Self::NotFound(..) => StatusCode::NOT_FOUND,
			Self::DatabaseError(..) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
			Self::UnavailableUsername(..) => StatusCode::BAD_REQUEST,
			Self::DisallowedUsername(..) => StatusCode::BAD_REQUEST,
			Self::UnhashedPassword => StatusCode::BAD_REQUEST,
			Self::AuthenticationCookieError => StatusCode::BAD_REQUEST,
			Self::RateLimitError(..) => StatusCode::TOO_MANY_REQUESTS,
            Self::AuthenticationError(..) => StatusCode::UNAUTHORIZED,
			Self::Unauthorized => StatusCode::UNAUTHORIZED,
			Self::WebhookError(..) => StatusCode::BAD_REQUEST,
			Self::GDPRRegion => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}
