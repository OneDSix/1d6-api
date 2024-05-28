use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ApiErrors {
    #[error("Error while validating input: {0}")]
    Validation(String),
}

impl ApiErrors {
    pub fn as_api_error<'a>(&self) -> ApiError<'a> {
        ApiError {
            error: match self {
                ApiErrors::Validation(..) => "invalid_input",
            },
            description: self.to_string(),
        }
    }
}

impl actix_web::ResponseError for ApiErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiErrors::Validation(..) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiError<'a> {
    pub error: &'a str,
    pub description: String,
}

pub async fn not_found() -> impl Responder {
    let data = ApiError {
        error: "not_found",
        description: "the requested route does not exist".to_string(),
    };

    HttpResponse::NotFound().json(data)
}
