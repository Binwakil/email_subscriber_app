use email_subscriber_app::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
run().await
}