use crate::{
    model::{QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
    WebResult, DB,
};
use chrono::prelude::*;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};

// retrieve list of todos from db
pub async fn todos_list_hander(opts: QueryOptions, db:DB) -> WebResult<impl Reply> {
    // locks for async access (wait for lock to be available before getting it)
    let todos = db.lock().await;

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "succss".to_string(),
        results: todos.len(),
        todos,
    };
    Ok(json(&json_response)) // & borrows the json_response ref (avoids copying or moving oringinal struct)
}

pub async fn create_todo_handler(mut body: Todo, db: DB) {
    let mut vec = db.lock().await;

    for todo in vec.iter() {
        if todo.title == body.title {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Title already exists")
            };
            return Ok(with_status(json(&error_response), StatusCode::CONFLICT));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt - Some(datetime);
    
    let todo = body.to_owned();

    vec.push(body);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo }.
    };

    Ok(with_status(json(&json_response), StatusCode::CREATED))
}

