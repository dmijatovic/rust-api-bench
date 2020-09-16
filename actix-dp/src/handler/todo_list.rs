extern crate todo_dp;

use todo_dp::todo_list::{NewTodoList, TodoList};
use todo_dp::{todo_list, Pool};
// use crate::db::todo_list::{NewTodoList, TodoList};
// use crate::db::{todo_list, Pool};
use actix_web::{delete, get, post, put, web, HttpResponse};

use super::response;

#[get("/todolist")]
pub async fn get_todo_lists(pool: web::Data<Pool>) -> HttpResponse {
  // pass request to db
  match todo_list::get_todo_lists(&pool).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok().json(res),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError().json(res)
    }
  }
}

#[post("/todolist")]
pub async fn add_todo_list(
  pool: web::Data<Pool>,
  new_list: web::Json<NewTodoList>,
) -> HttpResponse {
  // pass request to db
  match todo_list::add_todo_list(&pool, &new_list.title).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok().json(res),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError().json(res)
    }
  }
}

#[put("/todolist")]
pub async fn update_todo_list(pool: web::Data<Pool>, list: web::Json<TodoList>) -> HttpResponse {
  // pass request to db
  match todo_list::update_todo_list(&pool, list.id, &list.title).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok().json(res),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError().json(res)
    }
  }
}

#[delete("/todolist/{lid}")]
pub async fn delete_todo_list(pool: web::Data<Pool>, lid: web::Path<i32>) -> HttpResponse {
  // pass request to db
  match todo_list::delete_todo_list(&pool, *lid).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok().json(res),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError().json(res)
    }
  }
}

#[get("/todolist/{lid}")]
pub async fn get_todo_list(pool: web::Data<Pool>, lid: web::Path<i32>) -> HttpResponse {
  // pass request to db
  match todo_list::get_todo_list(&pool, *lid).await {
    //if request is ok return json data
    Ok(res) => HttpResponse::Ok().json(res),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      HttpResponse::InternalServerError().json(res)
    }
  }
}
