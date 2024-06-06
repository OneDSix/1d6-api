use crate::{
	routes::responses::ApiErrors,
	state,
};

use actix_web::{
    get, post, Result, web::{scope, Data, Json, Path, ServiceConfig}
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use state::AppState;

#[derive(Serialize, Deserialize)]
struct NewMod {
    pub owner: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub download: String,
	pub maven: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Mod {
    pub id: i32,
    pub owner: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub download: String,
	pub maven: String,
	pub git_based: bool,
	pub jitpack: bool,
}

pub  fn config(cfg: &mut ServiceConfig) {
	cfg.service(
		scope("mods")
		.service(post_mod)
		.service(get_mod)
	);
}

#[post("/")]
async fn post_mod(new_mod: Json<NewMod>, state: Data<AppState>) -> Result<Json<Mod>, ApiErrors> {
    let todo = sqlx::query_as(
		"
		INSERT INTO mods(name, icon, description, download, owner)
		VALUES (name, icon, description, download, owner)
		RETURNING id, name, icon, description, download, owner
		"
	)
	.bind(&new_mod.name).bind(&new_mod.icon).bind(&new_mod.description).bind(&new_mod.download).bind(&new_mod.owner)
	.fetch_one(&state.pool)
	.await
	.map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

    Ok(Json(todo))
}

#[get("/{id}")]
async fn get_mod(path: Path<i32>, state: Data<AppState>) -> Result<Json<Mod>, ApiErrors> {
    let todo = sqlx::query_as("SELECT * FROM mods WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|err| ApiErrors::DatabaseError(err.to_string()).into())?;

    Ok(Json(todo))
}
