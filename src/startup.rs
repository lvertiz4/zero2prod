use axum::{
    extract::Extension,
    routing::{get, post, IntoMakeService},
    Router,
};
use hyper::{server::conn::AddrIncoming, Server};
use sqlx::PgPool;
use std::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    greet,
    routes::{health_check::health_check, subscriptions::subscribe},
};

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> hyper::Result<Server<AddrIncoming, IntoMakeService<Router>>>  {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:uri", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        // Register the database connection as part of the application state
        .layer(Extension(connection))
        //TracerLayer initiates the tower_http::trace module middleware - see main.rs for env_logger init function that delivers messages from tower_http which are wrapped within the Log crate API
        .layer(TraceLayer::new_for_http());

    let server = axum::Server::from_tcp(listener)
        .expect("Could not instantiate TcpListener")
        .serve(app.into_make_service());

    Ok(server)
}
