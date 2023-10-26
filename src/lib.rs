use std::net::TcpListener;
use axum::{Router, routing::{get, IntoMakeService, post}, http::{Uri, StatusCode}, response::IntoResponse, Form};
use hyper::{Server, server::conn::AddrIncoming};
use serde::Deserialize;

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

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: Form<FormData>) -> StatusCode {
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> hyper::Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:uri", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));        

    let server = axum::Server::from_tcp(listener)
        .expect("Could not instantiate TcpListener")
        .serve(app.into_make_service());

    Ok(server)      
   
}