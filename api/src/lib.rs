use actix_web::web::{get, Data, FormConfig, JsonConfig, PathConfig, QueryConfig, ServiceConfig};
use lazy_static::lazy_static;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use utils::env::check_env_vars;
use routes::{
    defaults::{default_cors, default_ratelimit, not_found, robots_file}, errors::ApiErrors
};

pub mod routes;
pub mod utils;

/// TODO: Update this so it clears all online every 10 minutes, and saves IP addresses so clients can send heartbeats which keep it from clearing
#[derive(Clone, Copy, Debug)]
pub struct OnlineGlobal {
	pub proxies: i32,
	pub clients: i32,
	pub servers: i32,
	pub launchers: i32,
	pub unknown: i32,
}

impl OnlineGlobal {
	pub fn new() -> Self {
		OnlineGlobal { servers: 0, proxies: 0, clients: 0, launchers: 0, unknown: 0, }
	}

	pub fn change_proxies(mut self, change: i32) {
		self.proxies += change;
	}

	pub fn change_client(mut self, change: i32) {
		self.clients += change;
	}

	pub fn change_servers(mut self, change: i32) {
		self.servers += change;
	}

	pub fn change_launchers(mut self, change: i32) {
		self.launchers += change;
	}

	pub fn change_unknown(mut self, change: i32) {
		self.unknown += change;
	}
}

lazy_static! {
	static ref ONLINE: OnlineGlobal = OnlineGlobal::new();
}

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
		secrets,
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
		// Special path for robots.txt
		.route("/robots.txt", get().wrap(default_cors()).wrap(default_ratelimit()).to(robots_file))

		// Data Handling
		.app_data(state)
		;
    };

    Ok(config.into())
}
