use crate::db::todo_item::{NewTodoItem, TodoItem};
use crate::db::{todo_item, Pool};
use actix_web::{delete, get, post, put, web, HttpResponse};

use super::response;

#[get("/todolist/{lid}/items")]
pub async fn get_todo_items(pool: web::Data<Pool>, lid: web::Path<i32>) -> HttpResponse {
  match todo_item::get_todo_items(&pool, *lid).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok()
      .content_type("application/json")
      .body(serde_json::to_string(&res).unwrap()),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
    }
  }
}

#[get("/todo/{id}")]
pub async fn get_todo_item(pool: web::Data<Pool>, id: web::Path<i32>) -> HttpResponse {
  match todo_item::get_todo_item(&pool, *id).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok()
      .content_type("application/json")
      .body(serde_json::to_string(&res).unwrap()),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
    }
  }
}

#[post("/todo")]
pub async fn add_todo_item(
  pool: web::Data<Pool>,
  new_item: web::Json<NewTodoItem>,
) -> HttpResponse {
  // pass request to db
  match todo_item::add_todo_item(&pool, &new_item).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok()
      .content_type("application/json")
      .body(serde_json::to_string(&res).unwrap()),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
    }
  }
}

#[put("/todo")]
pub async fn update_todo_item(pool: web::Data<Pool>, todo: web::Json<TodoItem>) -> HttpResponse {
  // pass request to db
  match todo_item::update_todo_item(&pool, &todo).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok()
      .content_type("application/json")
      .body(serde_json::to_string(&res).unwrap()),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
    }
  }
}

#[delete("/todo/{id}")]
pub async fn delete_todo_item(pool: web::Data<Pool>, id: web::Path<i32>) -> HttpResponse {
  // pass request to db
  match todo_item::delete_todo_item(&pool, *id).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok()
      .content_type("application/json")
      .body(serde_json::to_string(&res).unwrap()),
    //if error create server error json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
    }
  }
}
