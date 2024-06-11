use actix_web::web::{get, Data, FormConfig, JsonConfig, PathConfig, QueryConfig, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use utils::env::check_env_vars;
use routes::{
    errors::ApiErrors, defaults::{default_cors, default_ratelimit, not_found}
};

pub mod routes;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub secrets: SecretStore,
}

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

	// Set the global state
	let state = Data::new(AppState {
        pool,
		secrets
    });

	// Check to make sure all the env-vars are present
	if let Err(missing_vec) = check_env_vars(&state) {
		panic!("Missing environment variable(s): {:?}", missing_vec)
	}

	// It's go time.
    let config = move |cfg: &mut ServiceConfig| {
        cfg
		// Error Handling
		.app_data(FormConfig::default().error_handler(|err, _req| ApiErrors::Validation(err.to_string()).into()))
		.app_data(PathConfig::default().error_handler(|err, _req| ApiErrors::Validation(err.to_string()).into()))
		.app_data(QueryConfig::default().error_handler(|err, _req| ApiErrors::Validation(err.to_string()).into()))
		.app_data(JsonConfig::default().error_handler(|err, _req| ApiErrors::Validation(err.to_string()).into()))
		.default_service(get().wrap(default_cors()).wrap(default_ratelimit()).to(not_found))

		// Routes
		.configure(|api_cfg|routes::root_config(api_cfg, &state))

		// Data Handling
		.app_data(state)
		;
    };

    Ok(config.into())
}
