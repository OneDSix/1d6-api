#![allow(non_snake_case)]

use shuttle_runtime::SecretStore;
use sqlx::PgPool;

/// Handling for the global state of the app. Try not to write to it too much, as it's accessed from all threads.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
	//pub secrets: SecretStore,
}
