use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::{app_state::AppState, handlers::todos};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(todos::get_todos).post(todos::create_todo))
        .route(
            "/todos/:id",
            put(todos::update_todo).delete(todos::delete_todo),
        )
        .with_state(state)
}
