use actix_identity::Identity;
use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;

use crate::{
	routes::{errors::ApiErrors, defaults::{IMPOSSIBLE_CONDITION, SUCCESS, default_cors, default_ratelimit}},
    utils::username::{PasswordResult, UsernameResult},
    AppState,
};

#[derive(FromRow)]
struct Password(String);

#[derive(Serialize, Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn login_json(name: String) -> Value {
    json!({
        "logged_in_as": name
    })
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("user")
			.wrap(default_cors())
			.wrap(default_ratelimit())
            .service(get_root)
            .service(post_signup)
            .service(post_login)
            .service(post_logout),
    );
}

#[get("")]
async fn get_root<'a>(identity: Option<Identity>) -> Result<HttpResponse, ApiErrors<'a>> {
    let id = match identity.map(|id| id.id()) {
        None => "NoLI".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(_)) => return Err(ApiErrors::AuthenticationCookieError.into()),
    };

    Ok(HttpResponse::Ok().json(login_json(id)))
}

#[post("/signup")]
async fn post_signup<'a>(
    req: HttpRequest,
    json: Json<Credentials>,
    state: Data<AppState>,
) -> Result<HttpResponse, ApiErrors<'a>> {
    #[allow(unused_assignments)]
    let mut returns: Result<HttpResponse, ApiErrors> =
        Err(ApiErrors::UnknownError("Internal operation not yet started".to_string()).into());

    match PasswordResult::password_check(json.password.clone()).await {
        Ok(_) => match UsernameResult::username_check(json.username.clone(), &state).await {
            Err(UsernameResult::DatabaseError(e)) => {
                returns = Err(ApiErrors::DatabaseError(
                    "Error while checking username availability: ".to_string() + &e,
                )
                .into())
            }
            Err(UsernameResult::FowlLanguage) | Err(UsernameResult::SqlInjection) => {
                returns = Err(ApiErrors::DisallowedUsername(json.username.clone()).into())
            }
            Err(UsernameResult::Taken) => {
                returns = Err(ApiErrors::UnavailableUsername(json.username.clone()).into())
            }
            Ok(UsernameResult::Passed) => {
                let _query = sqlx::query_as(
                    "
						INSERT INTO users (username, password)
						VALUES (username, password);
						",
                )
                .bind(&json.username)
                .bind(&json.password)
                .fetch_one(&state.pool)
                .await
                .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

                Identity::login(&req.extensions(), json.username.to_string()).unwrap();
                returns = Ok(HttpResponse::Ok().json(login_json(json.username.clone())))
            }
            Ok(_) | Err(_) => {
                returns = Err(ApiErrors::UnknownError(IMPOSSIBLE_CONDITION.to_string()).into())
            }
        },
        Err(_) => returns = Err(ApiErrors::UnhashedPassword.into()),
    }

    returns
}

#[post("/login")]
async fn post_login<'a>(
    ident: Option<Identity>,
    req: HttpRequest,
    json: Json<Credentials>,
    state: Data<AppState>,
) -> Result<HttpResponse, ApiErrors<'a>> {
    // Get the hashed password from DB
    let query: Password = sqlx::query_as(
        "
		SELECT password
		FROM users
		WHERE username = (username)
		LIMIT 1
		",
    )
    .bind(&json.username)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

    #[allow(unused_assignments)]
    let mut returns: Result<HttpResponse, ApiErrors> =
        Err(ApiErrors::UnknownError("Internal operation not yet started".to_string()).into());

    // This assumes the passwords are hashed, and technically the client can send unhashed passwords,
    // but that shouldn't happen... hopefully.
    if json.password == query.0 {
        // Expire any other cookies before logging in again
        if let Some(user) = ident {
            user.logout();
        }

        Identity::login(&req.extensions(), json.username.to_string()).unwrap();
        returns = Ok(HttpResponse::Ok().json(login_json(json.username.clone())));
    } else {
        returns = Err(ApiErrors::InvalidCredentials.into());
    }

    returns
}

#[post("/logout")]
async fn post_logout(ident: Identity) -> impl Responder {
    ident.logout();
    HttpResponse::Ok().json(&*SUCCESS)
}
