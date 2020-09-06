use actix_files;
use actix_web::{delete, get, post, put};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
// use std::io::ErrorKind::Other;

use crate::db;
use crate::errors::AppError;
use crate::models::{CreateTodoList, PostTodoItem, TodoList};

pub fn static_files() -> actix_files::Files {
  actix_files::Files::new("/", "./static").index_file("index.html")
}

// get todos route
#[get("/todolist")]
pub async fn get_todo_lists(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
  // connect to db
  let client: Client = db_pool.get().await.map_err(AppError::db_error)?;

  // make query
  let result = db::get_todo_lists(&client).await;
  // return response
  result.map(|todos| HttpResponse::Ok().json(todos))
}

// create todo list
#[post("/todolist")]
pub async fn create_todo_list(
  db_pool: web::Data<Pool>,
  json: web::Json<CreateTodoList>,
) -> Result<impl Responder, AppError> {
  let client: Client = db_pool.get().await.map_err(AppError::db_error)?;

  let result = db::create_todo_list(&client, json.title.clone()).await;

  // return response
  result.map(|todos| HttpResponse::Ok().json(todos))
}

// update todo list
#[put("/todolist")]
pub async fn update_todo_list(
  db_pool: web::Data<Pool>,
  json: web::Json<TodoList>,
) -> Result<impl Responder, AppError> {
  let client: Client = db_pool.get().await.map_err(AppError::db_error)?;

  let result = db::update_todo_list(&client, &json).await;

  result.map(|todos| HttpResponse::Ok().json(todos))
}

// get todo items from the list
#[get("/todolist/{list_id}/items")]
pub async fn get_list_items(
  db_pool: web::Data<Pool>,
  path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
  let client: Client = db_pool.get().await.map_err(AppError::db_error)?;

  let result = db::get_list_items(&client, path.0).await;

  // return response
  result.map(|items| HttpResponse::Ok().json(items))
}

#[post("/todo")]
pub async fn create_todo_item(
  db_pool: web::Data<Pool>,
  new_item: web::Json<PostTodoItem>,
) -> Result<impl Responder, AppError> {
  let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
  let result = db::create_todo_item(&client, &new_item).await;
  // return response
  result.map(|item| HttpResponse::Ok().json(item))
  // match result {
  //   Ok(items) => HttpResponse::Ok().json(items),
  //   Err(_) => HttpResponse::InternalServerError().into()
  // }
}

// #[put("/todos/{list_id}/items/{item_id}")]
// pub async fn check_todo_item(
//   db_pool: web::Data<Pool>,
//   path: web::Path<(i32, i32)>,
// ) -> impl Responder {
//   let client: Client = db_pool
//     .get()
//     .await
//     .expect("Error connecting to the database");

//   let result = db::check_todo_item(&client, path.0, path.1).await;

//   let ok_resp = ApiResponse {
//     status: 200,
//     status_text: String::from("OK"),
//   };

//   match result {
//     Ok(()) => HttpResponse::Ok().json(&ok_resp),
//     Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ApiResponse {
//       status: 200,
//       status_text: String::from("Already checked:"),
//     }),
//     Err(_) => HttpResponse::InternalServerError().into(),
//   }
// }

#[delete("/todo/{id}")]
pub async fn delete_todo_item(db_pool: web::Data<Pool>, id: web::Path<i32>) -> impl Responder {
  // let resp = String::from(format!("Test this back {}",path));
  let client: Client = db_pool
    .get()
    .await
    .expect("Error connecting to the database");

  let result = db::delete_todo_item(&client, id.clone()).await;

  result.map(|todos| HttpResponse::Ok().json(todos))

  // match result {
  //   Ok(items) => HttpResponse::Ok().json(items),
  //   Err(_) => HttpResponse::InternalServerError().into()
  // }
}
