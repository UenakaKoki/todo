use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub completed: bool,
}
