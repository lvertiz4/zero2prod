use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    //env_logger::init() does call set_logger, so this is all we need to do
    // We are falling back to printing all the logs at info-level or above.
    // if the RUST_LOG environment variable has not been set (i.e. a default setting)
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    //Panic if we can't read the configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    //The Port number is now coming from the configuration file, rather than randomly selected by OS
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to a random port");
    run(listener, connection_pool)?.await
}
