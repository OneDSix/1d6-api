use actix_files::Files;
use actix_web::web::{scope, ServiceConfig};

use super::defaults::{default_cors, default_ratelimit};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("assets")
            // Base Middlewares
            .wrap(default_cors())
            .wrap(default_ratelimit())

            // Anything in the assets folder is fair game
            .service(Files::new("/", "assets"))
    );
}
