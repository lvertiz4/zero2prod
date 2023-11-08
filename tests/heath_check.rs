// `tokio::test` is the testing equivalent of `tokio::main`.
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;

struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

//Launch instance of our application in the background
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not retrieve TcpListener");
    // //Retrieve the port number assigned to us by the Operating System
    let port = listener
        .local_addr()
        .expect("Could not retrieve local address")
        .port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let server = run(listener, connection_pool.clone()).expect("Failed to bind to address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    //We return the TestApp instance to the caller
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    //create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");
    //Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

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
    let address = spawn_app().await.address;
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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Set up instance of web serve application and testing Client instance
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    //Act
    let body = "name=luis%20vertiz&email=lvertiz%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute subscription POST request");
    //Testing
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "lvertiz@gmail.com");
    assert_eq!(saved.name, "luis vertiz");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=luis%20vertiz", "missing the email"),
        ("email=lvertiz%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute POST Request");
        assert_eq!(
            422,
            response.status().as_u16(), // Axum defaults to 422 instead of 400
            //Additional customized error message on test failue
            "The API did not fail with 400 Bad Request when payload was {}.",
            error_message
        );
    }
}
