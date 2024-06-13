//! Based on https://github.com/modrinth/labrinth/blob/master/src/routes/v3/statistics.rs
//! but with some extra optimizations (just caching as of now) and more information about the ecosystem

use std::sync::Mutex;

use lazy_static::lazy_static;

use actix_web::{
    get,
    web::{scope, Data, ServiceConfig},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    routes::{
        defaults::{default_cors, default_ratelimit},
        errors::ApiErrors,
    },
    utils::cacher::{Cache, Caching},
    AppState, ONLINE,
};

#[derive(FromRow)]
struct QueryData {
    pub count: i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Totals {
    pub users: i64,
    pub servers: i64,
    pub authors: i64,
    pub projects: i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct OnlineStatistics {
    pub proxies: i32,
    pub clients: i32,
    pub servers: i32,
    pub launchers: i32,
    pub unknown: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Stats {
    pub totals: Totals,
    pub online: OnlineStatistics,
    pub refresh_in: i64
}

struct StatCache {
    pub stat_cacher: Cache<Stats>,
	pub done_first_cache: bool
}

// Init the global cache value (in the most roundabout way possible of course)
lazy_static! {
    static ref GLOBAL_STAT_CACHE: Mutex<StatCache> = Mutex::new(StatCache {
        stat_cacher: Cache::<Stats>::new(),
		done_first_cache: false
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

#[get("")]
async fn get_stats<'a>(state: Data<AppState>) -> Result<HttpResponse, ApiErrors<'a>> {
    let mut stats = GLOBAL_STAT_CACHE.lock().unwrap();

    let data = get_cache(stats.stat_cacher.get_expiration(), state)
        .await
        .map_err(|e| return e)?; // May throw a DatabaseError, pass that onto the client

	if !stats.done_first_cache {
		stats.done_first_cache = true;
		stats.stat_cacher.unsafe_update(data);
	} else {
		stats.stat_cacher.update(data);
	}

    Ok(HttpResponse::Ok().json(stats.stat_cacher.cache_data.clone()))
}

async fn get_cache<'a>(refresh_in: i64, state: Data<AppState>) -> Result<Stats, ApiErrors<'a>> {

    let user: QueryData = sqlx::query_as("SELECT COUNT(id) FROM users")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

    let server: QueryData = sqlx::query_as("SELECT COUNT(id) FROM servers")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

	/*
	TODO: this doesnt work, something is wrong with the SQL command and keep returning errors

	let authors: QueryData = sqlx::query_as(
		"
			SELECT COUNT DISTINCT user_id AS unique_user_count
			FROM (
				SELECT user_id FROM project_owners
				UNION
				SELECT user_id FROM server_owners
			) AS combined_user_ids;
		"
		)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;
	*/

    let projects: QueryData = sqlx::query_as("SELECT COUNT(id) FROM projects")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ApiErrors::DatabaseError(e.to_string()).into())?;

    Ok(Stats {
        totals: Totals {
            users: user.count,
            servers: server.count,
            authors: 0, // look above at the TODO line
            projects: projects.count,
        },
        // TODO: messy cloning, memory go bye bye
        online: OnlineStatistics {
            proxies: ONLINE.proxies.clone(),
            clients: ONLINE.clients.clone(),
            servers: ONLINE.servers.clone(),
            launchers: ONLINE.launchers.clone(),
            unknown: ONLINE.unknown.clone(),
        },
        refresh_in,
    })
}
