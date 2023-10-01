use axum::{Router, routing::get, http::{Uri, StatusCode}, response::IntoResponse};

async fn greet(uri: Uri) -> impl IntoResponse {
  let mut name = uri.path().to_owned();
  if name == "/".to_owned() {
    format!("Hello, World!")
    } else {
    let _ = name.remove(0);
    format!("Hello, {}!", &name)
    }
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:uri", get(greet))
        .route("/health_check", get(health_check));        

    axum::Server::bind(&"0.0.0.0:3000".parse().expect("Server did not start as expected"))
        .serve(app.into_make_service())
        .await
        .unwrap();
   
}