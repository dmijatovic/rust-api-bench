use actix_files;
use actix_web::{web::ServiceConfig, HttpResponse};

mod response;
mod todo_item;
mod todo_list;

fn static_files() -> actix_files::Files {
  actix_files::Files::new("/", "./static").index_file("index.html")
}

pub async fn page404() -> HttpResponse {
  let html = std::fs::read_to_string("./static/404.html").unwrap_or(String::from("404 Not found"));
  HttpResponse::Ok().content_type("text/html").body(html)
}

pub fn register(cfg: &mut ServiceConfig) {
  //add graphQL schema and routes
  cfg
    // todo list api routes
    .service(todo_list::get_todo_lists)
    .service(todo_list::get_todo_list)
    .service(todo_list::add_todo_list)
    .service(todo_list::update_todo_list)
    .service(todo_list::delete_todo_list)
    // todo item api routes
    .service(todo_item::get_todo_items)
    .service(todo_item::get_todo_item)
    .service(todo_item::add_todo_item)
    .service(todo_item::update_todo_item)
    .service(todo_item::delete_todo_item)
    // static files with index.html
    .service(static_files());
}
