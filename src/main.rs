use std::net::TcpListener;
use email_subscriber_app::startup::run;
use email_subscriber_app::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
// Panic if we can't read configuration
let configuration = get_configuration().expect("Failed to read configuration.");
// We have removed the hard-coded `8000` - it's now coming from our settings!
let address = format!("127.0.0.1:{}", configuration.application_port);
let listener = TcpListener::bind(address)?;
run(listener)?.await
}