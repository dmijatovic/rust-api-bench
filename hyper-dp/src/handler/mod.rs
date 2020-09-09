extern crate todo_dp;
use hyper::{Body, Method, Request, Response};
use std::convert::Infallible;
use todo_dp::{create_pool, Pool};

mod response;
mod todo_item;
mod todo_list;

mod static_content;
use static_content as sc;

async fn dynamic_routes(req: Request<Body>, pool: &Pool) -> Result<Response<Body>, Infallible> {
  let path: Vec<&str> = req.uri().path().split("/").collect();
  match (req.method(), path[1]) {
    (&Method::GET, "todolist") => {
      if path.len() == 4 && path[3] == "items" {
        let list_id: i32 = path[2].parse().unwrap();
        Ok(todo_item::get_todo_items(&pool, list_id).await)
      } else {
        Ok(
          Response::builder()
            .status(400)
            .header("Content-Type", "application/json")
            .body(String::from("400-Bad Request").into())
            .unwrap(),
        )
      }
    }
    (&Method::DELETE, "todolist") => {
      let id: i32 = path[2].parse().unwrap();
      Ok(todo_list::delete_todo_lists(&pool, id).await)
    }
    (&Method::DELETE, "todo") => {
      let id: i32 = path[2].parse().unwrap();
      Ok(todo_item::delete_todo_item(&pool, id).await)
    }
    // Return the 404 Not Found for other routes.
    _ => Ok(sc::get_html("404.html")),
  }
}

pub async fn register(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  // create db connection pool
  let pool: Pool = create_pool().await;
  // register routes
  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => {
      info!("GET / - 200 OK");
      Ok(sc::get_html("index.html"))
    }
    (&Method::GET, "/favicon.png") => Ok(sc::get_favicon()),
    (&Method::GET, "/index.css") => Ok(sc::get_css("index.css")),
    (&Method::GET, "/todolist") => Ok(todo_list::get_todo_lists(&pool).await),
    (&Method::POST, "/todolist") => Ok(todo_list::add_todo_lists(req, &pool).await),
    (&Method::PUT, "/todolist") => Ok(todo_list::update_todo_lists(req, &pool).await),
    // (&Method::GET, "/todo") => Ok(todo_item::get_todo_items(&pool, 1).await),
    (&Method::POST, "/todo") => Ok(todo_item::add_todo_item(&pool, req).await),
    (&Method::PUT, "/todo") => Ok(todo_item::update_todo_item(&pool, req).await),
    // dynamic routes for delete and get items of a list
    _ => dynamic_routes(req, &pool).await,
  }
}
