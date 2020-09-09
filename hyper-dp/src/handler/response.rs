use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
  pub status: u16,
  pub status_text: String,
  pub payload: T,
}

pub fn server_error(message: String) -> ApiResponse<String> {
  ApiResponse {
    status: 500,
    status_text: String::from("Internal Server Error"),
    payload: message,
  }
}
