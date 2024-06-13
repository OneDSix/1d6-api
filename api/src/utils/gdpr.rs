use actix_web::{
	body::EitherBody,
	dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
	Error,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};

use crate::routes::errors::ApiErrors;

use backend::is_gdpr;

pub struct GDPRTransformer;

impl<S, B> Transform<S, ServiceRequest> for GDPRTransformer
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type Transform = GDPRService<S>;
	type InitError = ();
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(GDPRService {
			service,
		}))
	}
}

#[doc(hidden)]
pub struct GDPRService<S> {
	service: S,
}

impl<S, B> Service<ServiceRequest> for GDPRService<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	forward_ready!(service);

	fn call(&self, req: ServiceRequest) -> Self::Future {
		if is_gdpr(req.request()) {
			let fut = self.service.call(req);
			Box::pin(async move {
				match fut.await {
					Ok(service_response) => {
						// Return the modified response as Ok.
						Ok(service_response.map_into_left_body())
					}
					Err(e) => {
						// Handle error case
						Err(e)
					}
				}
			})
		} else {
			let response = ApiErrors::GDPRRegion.error_response();
			Box::pin(async { Ok(req.into_response(response.map_into_right_body())) })
		}
	}
}

mod backend {

	use std::collections::HashSet;

	use actix_web::{http::header, HttpRequest};
	use lazy_static::lazy_static;
	use regex::Regex;

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
		"GB", // Great Britain (United Kingdom)
		// GDPR-Adjacent Countries
		"CN", // China
		"RU", // Russia
		"CA", // Canada

		// If more should be added, make a PR and link the related legislation, it'll be accepted asap.
	];

	lazy_static! {
		static ref GDPR_LANGUAGE_SET: HashSet<&'static str> = GDPR_LANGUAGES.iter().cloned().collect();
		pub static ref LANG_REGEX: Regex = Regex::new(r"([a-z]{2})(?:-[A-Z]{2})?(?:;q=\d+\.\d+)?").unwrap();
	}

	pub fn is_gdpr(req: &HttpRequest) -> bool {
		// TODO: rewrite this absurdity
		// update: i made it worse
		if let Some(lang_header) = req.headers().get(header::ACCEPT_LANGUAGE) {
			if let Ok(lang_str) = lang_header.to_str() {
				for cap in LANG_REGEX.captures_iter(lang_str) {
					if let Some(captured_string) = cap.get(1) {
						let uppercased_string = captured_string.as_str().to_uppercase();
						return GDPR_LANGUAGES.iter().any(|test_against| test_against == &uppercased_string.as_str());
					}
				}
			}
		}
		false
	}
}
