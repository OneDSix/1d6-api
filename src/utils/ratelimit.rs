//! Based off https://github.com/modrinth/labrinth/blob/master/src/util/ratelimit.rs

use std::{num::NonZeroU32, str::FromStr, sync::Arc};

use actix_web::{
	ResponseError,
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use governor::{
	clock::{Clock, DefaultClock},
	middleware::{self, StateInformationMiddleware},
	state, Quota, RateLimiter
};
use lazy_static::lazy_static;
use futures_util::future::{ready, LocalBoxFuture, Ready};

use crate::routes::errors::ApiErrors;

pub fn default_ratelimit() -> RateLimit {
    RateLimit(Arc::clone(&LIMITER))
}

lazy_static! {
    static ref LIMITER: KeyedRateLimiter = Arc::new(
        RateLimiter::keyed(Quota::per_second(NonZeroU32::new(5).unwrap()))
            .with_middleware::<StateInformationMiddleware>(),
    );
}

pub type KeyedRateLimiter<K = String, MW = middleware::StateInformationMiddleware> =
    Arc<RateLimiter<K, state::keyed::DefaultKeyedStateStore<K>, DefaultClock, MW>>;

pub struct RateLimit(pub KeyedRateLimiter);

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RateLimitService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitService {
            service,
            rate_limiter: Arc::clone(&self.0),
        }))
    }
}

#[doc(hidden)]
pub struct RateLimitService<S> {
    service: S,
    rate_limiter: KeyedRateLimiter,
}

impl<S, B> Service<ServiceRequest> for RateLimitService<S>
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
        let conn_info = req.connection_info().clone();
        let ip = conn_info.peer_addr();

        if let Some(ip) = ip {
            let ip = ip.to_string();

            match self.rate_limiter.check_key(&ip) {
                Ok(snapshot) => {
                    let fut = self.service.call(req);

                    Box::pin(async move {
                        match fut.await {
                            Ok(mut service_response) => {
                                // Now you have a mutable reference to the ServiceResponse, so you can modify its headers.
                                let headers = service_response.headers_mut();
                                headers.insert(
                                    actix_web::http::header::HeaderName::from_str(
                                        "x-ratelimit-limit",
                                    )
                                    .unwrap(),
                                    snapshot.quota().burst_size().get().into(),
                                );
                                headers.insert(
                                    actix_web::http::header::HeaderName::from_str(
                                        "x-ratelimit-remaining",
                                    )
                                    .unwrap(),
                                    snapshot.remaining_burst_capacity().into(),
                                );

                                headers.insert(
                                    actix_web::http::header::HeaderName::from_str(
                                        "x-ratelimit-reset",
                                    )
                                    .unwrap(),
                                    snapshot
                                        .quota()
                                        .burst_size_replenished_in()
                                        .as_secs()
                                        .into(),
                                );

                                // Return the modified response as Ok.
                                Ok(service_response.map_into_left_body())
                            }
                            Err(e) => {
                                // Handle error case
                                Err(e)
                            }
                        }
                    })
                }
                Err(negative) => {
                    let wait_time = negative.wait_time_from(DefaultClock::default().now());

                    let mut response = ApiErrors::RateLimitError(
                        wait_time.as_millis(),
                        negative.quota().burst_size().get(),
                    )
                    .error_response();

                    let headers = response.headers_mut();

                    headers.insert(
                        actix_web::http::header::HeaderName::from_str("x-ratelimit-limit").unwrap(),
                        negative.quota().burst_size().get().into(),
                    );
                    headers.insert(
                        actix_web::http::header::HeaderName::from_str("x-ratelimit-remaining")
                            .unwrap(),
                        0.into(),
                    );
                    headers.insert(
                        actix_web::http::header::HeaderName::from_str("x-ratelimit-reset").unwrap(),
                        wait_time.as_secs().into(),
                    );

                    Box::pin(async { Ok(req.into_response(response.map_into_right_body())) })
                }
            }
        } else {
            let response =
                ApiErrors::CustomAuthentication("Unable to obtain user IP address!".to_string())
                    .error_response();
            Box::pin(async { Ok(req.into_response(response.map_into_right_body())) })
        }
    }
}
