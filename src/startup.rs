use axum::{
    routing::{get, post, IntoMakeService}, 
    Router,
};
use hyper::{server::conn::AddrIncoming, Server,};
use std::net::TcpListener;

use crate::{
    greet,
    routes::{health_check::health_check,subscriptions::subscribe}, 
};

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