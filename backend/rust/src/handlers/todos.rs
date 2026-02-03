use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::{
    app_state::AppState,
    models::Todo,
    requests::{CreateTodoRequest, UpdateTodoRequest},
};

pub async fn get_todos(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos"
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(todos)
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(req): Json<CreateTodoRequest>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO todos (title, completed) VALUES (?, false)",
        req.title
    )
    .execute(&state.db)
    .await
    .unwrap();

    Json(Todo {
        id: result.last_insert_id() as i32,
        title: req.title,
        completed: false,
    })
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateTodoRequest>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE todos SET completed = ? WHERE id = ?",
        req.completed,
        id
    )
    .execute(&state.db)
    .await
    .unwrap();

    if result.rows_affected() == 0 {
        return (axum::http::StatusCode::NOT_FOUND, "Todo not found").into_response();
    }

    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    Json(todo).into_response()
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "DELETE FROM todos WHERE id = ?",
        id
    )
    .execute(&state.db)
    .await
    .unwrap();

    if result.rows_affected() == 0 {
        return (axum::http::StatusCode::NOT_FOUND, "Todo not found").into_response();
    }

    Json(serde_json::json!({ "message": "deleted" }))
}
