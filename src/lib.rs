use axum::{Router, routing::{get, IntoMakeService}, http::{Uri, StatusCode}, response::IntoResponse};
use hyper::{Server, server::conn::AddrIncoming};

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

pub fn run() -> hyper::Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:uri", get(greet))
        .route("/health_check", get(health_check));        

    let server: Server<AddrIncoming, IntoMakeService<Router>> = axum::Server::bind(&"0.0.0.0:3000".parse().expect("Server did not start as expected"))
        .serve(app.into_make_service());

    Ok(server)      
   
}