#[macro_use]
extern crate log;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

mod handler;

async fn start_server() {
  //
  let host = env::var("SERVER_HOST").unwrap_or(String::from("127.0.0.1"));
  let port: u32 = env::var("SERVER_PORT")
    .unwrap_or(String::from("8080"))
    .parse()
    .unwrap();
  // define
  // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

  // A `Service` is needed for every connection, so this
  // creates one from our routes main function.
  let service = make_service_fn(|_conn| async {
    // service_fn converts our function into a `Service`
    Ok::<_, Infallible>(service_fn(handler::register))
  });

  let server = Server::bind(&addr).serve(service);
  info!("Starting server on http://{}", addr);

  // Run this server... forever!
  if let Err(e) = server.await {
    error!("server error: {}", e);
  }
}

#[tokio::main]
async fn main() -> Result<(), String> {
  // enable logger
  std::env::set_var("RUST_LOG", "info");
  env_logger::init();
  //start hyper server
  start_server().await;
  // return OK at the end
  Ok(())
}
