use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key}, error, get, post, web::{scope, Data, Json, ServiceConfig}, HttpMessage, HttpRequest, HttpResponse, Responder
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{routes::responses::ApiErrors, state::AppState};

#[derive(Serialize, Deserialize, FromRow)]
struct Password {
	pub password: String
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn config(cfg: &mut ServiceConfig) {
	cfg.service(
		scope("user")
		.service(get_root)
		.service(post_login)
		.service(post_logout)

		.wrap(IdentityMiddleware::default())
		.wrap(
			SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
				.cookie_name("auth-token".to_owned())
				.cookie_secure(false)
				.session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(2)))
				.build()
		)
	);
}

#[get("")]
async fn get_root(identity: Option<Identity>) -> impl Responder {
    let id = match identity.map(|id| id.id()) {
        None => "NoLI".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

	let index_data = json!({
        "logged_in_as": id
    });

    Ok(HttpResponse::Ok().json(index_data))
}

#[post("/signup")]
async fn post_signup(_req: HttpRequest, state: Data<AppState>) -> impl Responder {
	let index_data = json!({
        "api_version": "1",
        "documentation": "DOCS_LINK"
    });

	HttpResponse::Ok().json(index_data)
}

#[post("/login")]
async fn post_login(req: HttpRequest, json: Json<Credentials>, state: Data<AppState>) -> Result<HttpResponse, ApiErrors> {

	// Get the hashed password from DB
	let query: Password = sqlx::query_as(
		"
		SELECT password
		FROM users
		WHERE username = (username)
		LIMIT 1
		"
	)
	.bind(&json.username)
	.fetch_one(&state.pool)
	.await
	.map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

	#[allow(unused_assignments)]
	let mut returns: Result<HttpResponse, ApiErrors> = Err(ApiErrors::UnknownError("Operation not started".to_string()).into());

	// Make sure both passwords are the same
	if json.password == query.password {
		// Success
		Identity::login(&req.extensions(), json.username.to_string()).unwrap();
		returns = Ok(HttpResponse::Ok().json(json!({"logged_in_as": &json.username})));
	} else {
		// Failure
		returns = Err(ApiErrors::InvalidCredentials("Invalid Username/Password Combo".to_string()).into());
	}

	returns
}

#[post("/expires")]
async fn post_expires(id: Identity) -> impl Responder {
	let index_data = json!({
        "logged_in_as": id.id().unwrap(),
        "token_expires": "NYI"
    });

    HttpResponse::Ok().json(index_data)
}

#[post("/logout")]
async fn post_logout(id: Identity) -> impl Responder {
    id.logout();
    HttpResponse::Ok().json(json!({"success": true}))
}
