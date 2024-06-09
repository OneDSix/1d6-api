use actix_web::web::{get, Data, FormConfig, JsonConfig, PathConfig, QueryConfig, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

use crate::{
	utils::{cors::default_cors, ratelimit::default_ratelimit},
	routes::{errors::ApiErrors, responses::not_found}
};
pub mod utils;
pub mod routes;

#[rustfmt::skip]
pub async fn run (
    pool: PgPool,
	secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
	// Errors and Logging
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

	// Database Init
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
	let state = Data::new(AppState {
        pool,
		secrets
    });

	// It's go time.
    let config = move |cfg: &mut ServiceConfig| {
        cfg
		// Error Handling
		.app_data(FormConfig::default().error_handler(|err, _req| ApiErrors::Validation(err.to_string()).into()))
		.app_data(PathConfig::default().error_handler(|err, _req| ApiErrors::Validation(err.to_string()).into()))
		.app_data(QueryConfig::default().error_handler(|err, _req| ApiErrors::Validation(err.to_string()).into()))
		.app_data(JsonConfig::default().error_handler(|err, _req| ApiErrors::Validation(err.to_string()).into()))
		.default_service(get().wrap(default_cors()).wrap(default_ratelimit()).to(not_found))

		// Data Handling
		.app_data(state)

		// Routes
		.configure(routes::v1::config)
		.configure(routes::root_config)
		;
    };

    Ok(config.into())
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
	pub secrets: SecretStore,
}
