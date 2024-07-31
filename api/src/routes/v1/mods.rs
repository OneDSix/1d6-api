use crate::{
    routes::{
        defaults::{default_cors, default_ratelimit, SUCCESS},
        errors::ApiErrors,
    },
    utils::validation::AuthChecker,
    AppState,
};

use actix_identity::Identity;
use actix_web::{
    get, patch, post,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
struct NewMod {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub source: String,
    pub maven: String,
    pub license: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Mod {
    pub id: i32,
    pub owner: String,

    pub name: String,
    pub icon: String,
    pub description: String,
    pub source: String,
    pub maven: String,
    pub license: String,
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("mods")
            .wrap(default_cors())
            .wrap(default_ratelimit())
            .service(post_mod)
            .service(get_mod),
    );
}

#[post("/")]
async fn post_mod<'a>(
    identity: Option<Identity>,
    new_mod: Json<NewMod>,
    state: Data<AppState>,
) -> Result<HttpResponse, ApiErrors<'a>> {
    match AuthChecker::check_exists(identity) {
        Ok(success) => {
            if let Some(ident) = success.get_identity() {
                if let Ok(ident_id) = ident.id() {
                    sqlx::query_as(
                        "
						INSERT INTO mods(name, icon, description, download, owner)
						VALUES ($1, $2, $3, $4, $5)
						RETURNING id, name, icon, description, download, owner
						",
                    )
                    .bind(&new_mod.name)
                    .bind(&new_mod.icon)
                    .bind(&new_mod.description)
                    .bind(&new_mod.source)
                    .bind(ident_id)
                    .fetch_one(&state.pool)
                    .await
                    .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

                    Ok(HttpResponse::Created().json(&*SUCCESS))
                } else {
                    Err(ApiErrors::Unauthorized)
                }
            } else {
                Err(ApiErrors::Unauthorized)
            }
        }
        Err(AuthChecker::NoLI) => Err(ApiErrors::NotLoggedIn),
        Err(AuthChecker::Unauthorized) => Err(ApiErrors::Unauthorized),
        _ => Err(ApiErrors::UnknownError("Issue matching login info.".to_string()).into()),
    }
}

#[get("/{id}")]
async fn get_mod<'a>(path: Path<i32>, state: Data<AppState>) -> Result<Json<Mod>, ApiErrors<'a>> {
    let target_mod: Mod = sqlx::query_as("SELECT * FROM mods WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|err| ApiErrors::DatabaseError(err.to_string()).into())?;

    Ok(Json(target_mod))
}

#[patch("/{id}")]
async fn patch_mod<'a>(
    identity: Option<Identity>,
    path: Path<i32>,
    new_mod: Json<NewMod>,
    state: Data<AppState>,
) -> Result<HttpResponse, ApiErrors<'a>> {
    let target_mod: Mod = sqlx::query_as("SELECT * FROM mods WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|err| ApiErrors::DatabaseError(err.to_string()).into())?;

    match AuthChecker::check_against(identity, &target_mod.owner) {
        Ok(_) => {
            sqlx::query_as(
                "
                UPDATE mods
                SET name = $1, icon = $2, description = $3, download = $4
                WHERE id = $5
                ",
            )
            .bind(&new_mod.name)
            .bind(&new_mod.icon)
            .bind(&new_mod.description)
            .bind(&new_mod.source)
            .bind(*path)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

            Ok(HttpResponse::Accepted().json(&*SUCCESS))
        }
        Err(AuthChecker::NoLI) => Err(ApiErrors::NotLoggedIn),
        Err(AuthChecker::Unauthorized) => Err(ApiErrors::Unauthorized),
        _ => Err(ApiErrors::UnknownError("Issue matching login info.".to_string()).into()),
    }
}
