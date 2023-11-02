use axum::response::IntoResponse;
use hyper::Uri;

pub mod configuration;
pub mod routes;
pub mod startup;

pub async fn greet(uri: Uri) -> impl IntoResponse {
  let mut name = uri.path().to_owned();
  if name == "/".to_owned() {
    format!("Hello, World!")
    } else {
    let _ = name.remove(0);
    format!("Hello, {}!", &name)
    }
}
