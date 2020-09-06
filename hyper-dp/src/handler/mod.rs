use hyper::{Body, Method, Request, Response};
use std::convert::Infallible;

mod static_content;
use static_content as sc;

pub async fn register(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => Ok(sc::get_html("index.html")),
    (&Method::GET, "/favicon.png") => Ok(sc::get_favicon()),
    (&Method::GET, "/index.css") => Ok(sc::get_css("index.css")),
    // Return the 404 Not Found for other routes.
    _ => Ok(sc::get_html("404.html")),
  }
}
