use hyper::{Body, Response};
// use std::convert::Infallible;
// std::fs::read_to_string(path: P);
use std::io::Error;

pub fn get_static_content(file: &str) -> Result<String, Error> {
  let path = format!("./static/{}", file);
  std::fs::read_to_string(&path)
}

pub fn get_static_image(file: &str) -> Result<Vec<u8>, Error> {
  let path = format!("./static/{}", file);
  // std::fs::read(&path)
  std::fs::read(&path)
}

pub fn get_html(file: &str) -> Response<Body> {
  // get content of html file
  let content = get_static_content(file).unwrap();
  // build response
  Response::builder()
    .status(200)
    .header("Content-Type", "text/html")
    .body(content.into())
    .unwrap()
}

pub fn get_favicon() -> Response<Body> {
  // get content of html file
  let content = get_static_image("favicon.png").unwrap();
  // build response
  Response::builder()
    .status(200)
    .header("Content-Type", "image/png")
    .body(content.into())
    .unwrap()
}

pub fn get_css(file: &str) -> Response<Body> {
  // get content of html file
  let content = get_static_content(file).unwrap();
  // build response
  Response::builder()
    .status(200)
    .header("Content-Type", "text/css")
    .body(content.into())
    .unwrap()
}
