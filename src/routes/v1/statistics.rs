use std::sync::Mutex;

use crate::{
    routes::errors::ApiErrors,
    utils::{cacher::Cache, cors::default_cors, ratelimit::default_ratelimit},
	AppState
};

use lazy_static::lazy_static;

use actix_web::{
    get,
    web::{scope, Data, ServiceConfig},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow)]
struct QueryData {
    pub count: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Totals {
    pub users: Option<i64>,
    pub servers: Option<i64>,
    pub authors: Option<i64>,
    pub mods: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Online {
    pub users: Option<i64>,
    pub servers: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats {
    pub totals: Totals,
    pub online: Online,
    pub refresh_in: i64,
}

struct StatCache {
    pub stat_cacher: Cache<Stats>,
}

// Init the global cache value (in the most roundabout way possible of course)
lazy_static! {
    static ref GLOBAL_STAT_CACHE: Mutex<StatCache> = Mutex::new(StatCache {
        stat_cacher: Cache::<Stats>::new(),
    });
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("stats")
            .wrap(default_cors())
            .wrap(default_ratelimit())
            .service(get_stats),
    );
}

// Based on https://github.com/modrinth/labrinth/blob/master/src/routes/v3/statistics.rs, but with some extra optimizations and more information
#[get("")]
async fn get_stats(state: Data<AppState>) -> Result<HttpResponse, ApiErrors> {
    let mut stats = GLOBAL_STAT_CACHE.lock().unwrap();

    if let Some(data) = get_cache(stats.stat_cacher.get_expiration(), state)
        .await
        .ok()
    {
        stats.stat_cacher.safe_update(data);
    }

    Ok(HttpResponse::Ok().json(stats.stat_cacher.cache_data.clone()))
}

async fn get_cache(refresh_in: i64, state: Data<AppState>) -> Result<Stats, ApiErrors> {
    let user: QueryData = sqlx::query_as("SELECT COUNT(id) FROM users")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

    /*
    let server_count = sqlx::query_as(
        "
        SELECT COUNT(id)
        FROM mods
        "
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    let author_count = sqlx::query_as(
        "
        SELECT COUNT(id)
        FROM mods
        "
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    */

    let mods: QueryData = sqlx::query_as("SELECT COUNT(id) FROM mods")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

    Ok(Stats {
        totals: Totals {
            users: Some(user.count),
            servers: Some(0),
            authors: Some(0),
            mods: Some(mods.count),
        },
        online: Online {
            users: Some(0),
            servers: Some(0),
        },
        refresh_in: refresh_in,
    })
}
