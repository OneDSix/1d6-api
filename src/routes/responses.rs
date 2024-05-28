use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/")]
pub async fn index_get() -> HttpResponse {
    let data = json!({
        "api_version": "1",
        "documentation": "TODO"
    });

    HttpResponse::Ok().json(data)
}
