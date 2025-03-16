use crate::model::Todo;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct TodoData {
    pub todo: Todo
}

#[derive(Serialize, Debug)]
pub struct SingleTodoResponse {
    pub status: String, 
    pub data: TodoData,
}

#[derive(Serialize, Debug)] // each struct independent hence have to keep deriving 
pub struct TodoListResponse {
    pub status: String,
    pub results: usize, // unsigned integer
    pub todos: Vec<Todo>,
}