use actix_web::{get, Result, web::{Data, scope, ServiceConfig}, HttpResponse};

use crate::{
	routes::{errors::ApiErrors, defaults::{default_cors, default_ratelimit}}, AppState
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("gs")
            .wrap(default_cors())
            .wrap(default_ratelimit())
            .service(get_root)
    );
}

#[get("/")]
async fn get_root<'a>(_state: Data<AppState>) -> Result<HttpResponse, ApiErrors<'a>> {
    Ok(HttpResponse::Ok().finish())
}
