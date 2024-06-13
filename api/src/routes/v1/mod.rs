pub mod game_services;
pub mod mods;
pub mod statistics;
pub mod user;
pub mod telemetry;

use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    web::{get, scope, ServiceConfig},
};

use super::defaults::{default_cors, default_ratelimit, index_get};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("v1")
            // Base Middlewares
            .wrap(default_cors())
            .wrap(default_ratelimit())

            // Auth
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
                    .cookie_name("auth-token".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(2)))
                    .build()
            )

            // Handle both "/v1" and "/v1/" as they can be easily mixed up
            .route("", get().to(index_get))
            .route("/", get().to(index_get))

            // Add the rest of the endpoints
            .configure(user::config)
            .configure(mods::config)
            .configure(statistics::config)
            .configure(game_services::config)
			.configure(telemetry::config)
    );
}
