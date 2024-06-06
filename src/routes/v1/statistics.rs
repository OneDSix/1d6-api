use crate::state;

use actix_web::{
    get, web::{scope, Data, ServiceConfig}, HttpResponse, Result
};
use serde::{Serialize, Deserialize};
use state::AppState;

#[derive(Serialize, Deserialize)]
struct Stats {
    pub users: Option<i64>,
    pub servers: Option<i64>,
    pub authors: Option<i64>,
    pub mods: Option<i64>,
}

pub fn config(cfg: &mut ServiceConfig) {
	cfg.service(
		scope("stats")
		.service(get_stats)
	);
}

// Based on https://github.com/modrinth/labrinth/blob/master/src/routes/v3/statistics.rs, but with some extra optimizations
#[get("/")]
async fn get_stats(_state: Data<AppState>) -> Result<HttpResponse> {

	let stats = Stats {
        users: Some(10),
        servers: Some(10),
        authors: Some(10),
        mods: Some(10),
    };

    Ok(HttpResponse::Ok().json(stats))
}
/*
async fn get_cache(state: Data<AppState>) {
	let user_count = sqlx::query_as(
        "
        SELECT COUNT(id)
        FROM mods
        WHERE status = ANY($1)
        "
    )
    .fetch_one(&state.pool)
    .await
	.map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    let server_count = sqlx::query_as(
        "
        SELECT COUNT(id)
        FROM mods
        WHERE status = ANY($1)
        "
    )
    .fetch_one(&state.pool)
    .await
	.map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    let author_count = sqlx::query_as(
        "
        SELECT COUNT(id)
        FROM mods
        WHERE status = ANY($1)
        "
    )
    .fetch_one(&state.pool)
    .await
	.map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    let mod_count = sqlx::query_as(
        "
        SELECT COUNT(id)
        FROM mods
        WHERE status = ANY($1)
        "
    )
    .fetch_one(&state.pool)
    .await
	.map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    let stats = Stats {
        users: user_count.count,
        servers: server_count.count,
        authors: author_count.count,
        mods: mod_count.count,
    };
}
*/
