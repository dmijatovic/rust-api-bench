use actix_files;
use actix_web::{get, web::ServiceConfig, HttpResponse};

use crate::graphql as gql;

mod graphql;
use graphql::{get_graphql, post_graphql};

#[get("/")]
pub async fn index() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html")
    .body("<h1>Actix GraphQL example</h1>")
}

// pub fn list_files() -> actix_files::Files {
//   actix_files::Files::new("/list", "./static").show_files_listing()
// }

fn static_files() -> actix_files::Files {
  actix_files::Files::new("/", "./static").index_file("index.html")
}

pub async fn page404() -> HttpResponse {
  let html = std::fs::read_to_string("./static/404.html").unwrap_or(String::from("404 Not found"));
  HttpResponse::Ok().content_type("text/html").body(html)
}

// pub async fn page404() -> Result<NamedFile, std::error::Error> {
//   Ok(NamedFile::open("./static/404.html")?)
// }

pub fn register(cfg: &mut ServiceConfig) {
  // Create Juniper schema
  let schema = std::sync::Arc::new(gql::create_schema());
  //add graphQL schema and routes
  cfg
    .data(schema)
    // shows graphiql user interace
    .service(get_graphql)
    // end point for graphql queries
    .service(post_graphql)
    // static files with index.html
    .service(static_files());
}
