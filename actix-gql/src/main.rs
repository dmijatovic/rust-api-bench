#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::env;

mod db;
mod graphql;
mod handler;

struct ActixConfig {
  host: String,
  port: u32,
  workers: u8,
}

fn get_server_env() -> ActixConfig {
  let host = env::var("SERVER_HOST").unwrap_or(String::from("127.0.0.1"));
  let port: u32 = env::var("SERVER_PORT")
    .unwrap_or(String::from("8080"))
    .parse()
    .unwrap();
  let workers: u8 = env::var("SERVER_WORKERS")
    .unwrap_or(String::from("1"))
    .parse()
    .unwrap();

  ActixConfig {
    host,
    port,
    workers,
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // get env vars
  let cfg = get_server_env();
  // define host
  let host = format!("{}:{}", cfg.host, cfg.port);
  // println!("Starting http server at {:?}", host);

  // enable logger
  // std::env::set_var("RUST_LOG", "actix_web=info");
  std::env::set_var("RUST_LOG", "info");
  env_logger::init();

  // show this
  info!("Starting http server at {:?}", host);

  // create db connection pool
  let pool = db::create_pool().await;
  //start http server
  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .data(pool.clone())
      .configure(handler::register)
      .default_service(web::to(handler::page404))
  })
  .bind(host)?
  .workers(cfg.workers.into())
  .run()
  .await
}
