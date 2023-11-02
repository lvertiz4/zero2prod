use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    //Panic if we can't read the configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    //The Port number is now coming from the configuration file, rather than randomly selected by OS
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to a random port");
    run(listener)?.await
}
