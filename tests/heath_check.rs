// `tokio::test` is the testing equivalent of `tokio::main`.
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
//     The test covers the full range of properties we are interested to check:
//      • the health check is exposed at /health_check;
//      • the health check is behind a GET method;
//      • the health check always returns a 200;
//      • the health check’s response has no body.
    //Instantiate app
    // spawn_app();
    //We need to bring in the Request library to instantiate an HTTP client to make requests against our application's Server
    //Code below is de-coupled from our App and its web framework, Axum
    let address = spawn_app();
    let client = reqwest::Client::new();
    //Send a Get request to the /health_check URL, which is handled by the health_check function in lib.rs
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request from Client");
    //compare response to Reqwest Client instance with expected Status Code Response from health_check handler function
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
//Launch instance of our application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Could not retrieve TcpListener");
    // //Retrieve the port number assigned to us by the Operating System
    let port = listener.local_addr().expect("Could not retrieve local address").port();
    let server = zero2prod::run(listener)
    .expect("Failed to bind to address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    //We return the application address to the caller
   format!("http://127.0.0.1:{}", port)
}