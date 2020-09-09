extern crate todo_dp;

use hyper::{Body, Request, Response};
// use hyper::body::
// use std::convert::Infallible;
use super::response;
use todo_dp::todo_list::{NewTodoList, TodoList};
use todo_dp::{todo_list, Pool};

pub async fn get_todo_lists(pool: &Pool) -> Response<Body> {
  let json = match todo_list::get_todo_lists(&pool).await {
    Ok(res) => serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize")),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("GET /todolist {:?}", e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"))
    }
  };
  info!("GET /todolist - 200 OK");
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(json.into())
    .unwrap()
}

pub async fn add_todo_lists(req: Request<Body>, pool: &Pool) -> Response<Body> {
  // Await the full body to be concatenated into a single `Bytes`...
  let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
  // deserialize body
  let list: NewTodoList = match serde_json::from_slice(&full_body) {
    Ok(list) => list,
    Err(e) => {
      // error macro will report module::fn automatically
      warn!("POST /todolist {:?}", e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      let json = serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"));
      return Response::builder()
        .status(400)
        .header("Content-Type", "application/json")
        .body(json.into())
        .unwrap();
    }
  };
  // make request to database
  let json = match todo_list::add_todo_list(&pool, &list.title).await {
    Ok(res) => serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize")),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("POST /todolist {:?}", e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"))
    }
  };
  info!("POST /todolist - 201 OK");
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(json.into())
    .unwrap()
}

pub async fn update_todo_lists(req: Request<Body>, pool: &Pool) -> Response<Body> {
  // Await the full body to be concatenated into a single `Bytes`...
  let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
  // deserialize body
  let list: TodoList = match serde_json::from_slice(&full_body) {
    Ok(list) => list,
    Err(e) => {
      // error macro will report module::fn automatically
      warn!("PUT /todolist {:?}", e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      let json = serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"));
      return Response::builder()
        .status(400)
        .header("Content-Type", "application/json")
        .body(json.into())
        .unwrap();
    }
  };
  // make request to database
  let json = match todo_list::update_todo_list(&pool, list.id, &list.title).await {
    Ok(res) => serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize")),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("PUT /todolist {:?}", e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"))
    }
  };
  info!("PUT /todolist - 200 OK");
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(json.into())
    .unwrap()
}

pub async fn delete_todo_lists(pool: &Pool, lid: i32) -> Response<Body> {
  let json = match todo_list::delete_todo_list(&pool, lid).await {
    Ok(res) => serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize")),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("DELETE /todolist/{} {:?}", lid, e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"))
    }
  };
  info!("DELETE /todolist/{} - 200 OK", lid);
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(json.into())
    .unwrap()
}
