#![allow(non_snake_case)]

use crate::state;

use actix_web::{
    error, get, post, web::{scope, Data, Json, Path, ServiceConfig}, Result
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use state::AppState;

#[derive(Serialize, Deserialize)]
struct NewMod {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub download: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Mod {
    pub id: i32,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub download: String,
    pub owner: String,
}

pub fn config(cfg: &mut ServiceConfig) {
	cfg.service(
		scope("mod")
		.service(addMod)
		.service(retrieveMod)
	);
}

#[post("/")]
pub async fn addMod(newMod: Json<NewMod>, state: Data<AppState>) -> Result<Json<Mod>> {
    let todo = sqlx::query_as(
		"INSERT INTO mods(name, icon, description, download, owner) VALUES (name, icon, description, download, owner) RETURNING id, name, icon, description, download, owner")
        .bind(&newMod.name).bind(&newMod.icon).bind(&newMod.description).bind(&newMod.download).bind(&newMod.owner)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}


#[get("/{id}")]
pub async fn retrieveMod(path: Path<i32>, state: Data<AppState>) -> Result<Json<Mod>> {
    let todo = sqlx::query_as("SELECT * FROM mods WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}
