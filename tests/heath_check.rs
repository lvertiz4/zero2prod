// `tokio::test` is the testing equivalent of `tokio::main`.
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

#[tokio::test]
async fn health_check_works() {
//     The test covers the full range of properties we are interested to check:
//      • the health check is exposed at /health_check;
//      • the health check is behind a GET method;
//      • the health check always returns a 200;
//      • the health check’s response has no body.
    //Instantiate app
    spawn_app();
    //We need to bring in the Request library to instantiate an HTTP client to make requests against our application
    //Code below is de-coupled from our App and its web framework, Axum
    let client = reqwest::Client::new();
    //Send a Get request to the /health_check URL, which is handled by the health_check function in lib.rs
    let response = client
        .get("http://localhost:3000/health_check")
        .send()
        .await
        .expect("Failed to execute Request");
    //compare response to Reqwest Client instance with expected Status Code Response from health_check handler function
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
//Launch instate of our application in the background
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}