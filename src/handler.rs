use crate::{
    model::{QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
    WebResult, DB,
};
use chrono::prelude::*;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};

pub async fn todos_list_hander(opts: QueryOptions, db:DB) -> WebResult<impl Reply> {
    let todos = db.lock().await;

    let limit = opts.limit.unwrap_or(10);
}