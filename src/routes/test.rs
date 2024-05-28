#![allow(non_snake_case)]

use crate::state::AppState;
use actix_web::{
    error, get, post,
    web::{Data, Json, Path, ServiceConfig},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
struct NewTodo {
    pub note: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub note: String,
}

pub fn config(cfg: &mut ServiceConfig) {
	cfg.service(addTodo);
	cfg.service(retrieveTodo);
}

#[post("")]
pub async fn addTodo(todo: Json<NewTodo>, state: Data<AppState>) -> Result<Json<Todo>> {
    let todo = sqlx::query_as("INSERT INTO todos(note) VALUES ($1) RETURNING id, note")
        .bind(&todo.note)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}

#[get("/{id}")]
pub async fn retrieveTodo(path: Path<i32>, state: Data<AppState>) -> Result<Json<Todo>> {
    let todo = sqlx::query_as("SELECT * FROM todos WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}
