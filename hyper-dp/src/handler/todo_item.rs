extern crate todo_dp;

use super::response;
use hyper::{Body, Request, Response};
use todo_dp::todo_item::{NewTodoItem, TodoItem};
use todo_dp::{todo_item, Pool};

pub async fn get_todo_items(pool: &Pool, lid: i32) -> Response<Body> {
  let json = match todo_item::get_todo_items(&pool, lid).await {
    Ok(res) => serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize")),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("GET /todolist/{}/items {:?}", lid, e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"))
    }
  };
  info!("GET /todolist/{}/items - 200 OK", lid);
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(json.into())
    .unwrap()
}

pub async fn add_todo_item(pool: &Pool, req: Request<Body>) -> Response<Body> {
  let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
  let item: NewTodoItem = match serde_json::from_slice(&full_body) {
    Ok(item) => item,
    Err(e) => {
      // error macro will report module::fn automatically
      warn!("POST /todo {:?}", e);
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
  let json = match todo_item::add_todo_item(&pool, &item).await {
    Ok(res) => serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize")),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("POST /todo {:?}", e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"))
    }
  };
  info!("POST /todo - 200 OK");
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(json.into())
    .unwrap()
}

pub async fn update_todo_item(pool: &Pool, req: Request<Body>) -> Response<Body> {
  let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
  let item: TodoItem = match serde_json::from_slice(&full_body) {
    Ok(item) => item,
    Err(e) => {
      // error macro will report module::fn automatically
      warn!("{:?}", e);
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
  let json = match todo_item::update_todo_item(&pool, &item).await {
    Ok(res) => serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize")),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("{:?}", e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"))
    }
  };
  info!("PUT /todo - 200 OK");
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(json.into())
    .unwrap()
}

pub async fn delete_todo_item(pool: &Pool, id: i32) -> Response<Body> {
  let json = match todo_item::delete_todo_item(&pool, id).await {
    Ok(res) => serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize")),
    //if error create server serror json response
    Err(e) => {
      // error macro will report module::fn automatically
      error!("DELETE /todo/{} {:?}", id, e);
      let res = response::server_error(format!("{:?}", e));
      // format!("{:?}", res)
      serde_json::to_string(&res).unwrap_or(String::from("Failed to serialize"))
    }
  };
  info!("DELETE /todo/{} - 200 OK", id);
  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(json.into())
    .unwrap()
}
