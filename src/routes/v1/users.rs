#![allow(non_snake_case, unused_imports)]

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key}, error, get, http::StatusCode, post, web::{self, scope, Data, Json, Path, ServiceConfig}, HttpMessage as _, HttpRequest, Responder, Result
};

pub fn config(cfg: &mut ServiceConfig) {
	cfg.service(
		scope("user")
		.service(accountRoot)
		.service(accountLogin)
		.service(accountLogout)

		.wrap(IdentityMiddleware::default())
		.wrap(
			SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
				.cookie_name("auth-token".to_owned())
				.cookie_secure(false)
				.session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(2)))
				.build(),
		)
	);
}

#[get("/")]
async fn accountRoot(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

#[get("/login")]
async fn accountLogin(req: HttpRequest) -> impl Responder {
    // some kind of authentication should happen here

    // attach a verified user identity to the active session
    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    web::Redirect::to("/").using_status_code(StatusCode::OK)
}

#[get("/logout")]
async fn accountLogout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::OK)
}
