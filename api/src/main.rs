use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use onedsixapi::run;

#[shuttle_runtime::main]
async fn main (
	#[shuttle_shared_db::Postgres] pool: PgPool,
	#[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
	run(pool, secrets).await
}
