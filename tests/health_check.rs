use std::net::TcpListener;
use email_subscriber_app::run;
use sqlx::{PgConnection, Connection};
use email_subscriber_app::configuration::get_configuration;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
// Arrange
let app_address = spawn_app();
let client = reqwest::Client::new();
// Act
let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
let response = client
.post(&format!("{}/subscriptions", &app_address))
.header("Content-Type", "application/x-www-form-urlencoded")
.body(body)
.send()
.await
.expect("Failed to execute request.");

// Assert
assert_eq!(200, response.status().as_u16());
}
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
let app_address = spawn_app();
let client = reqwest::Client::new();
let test_cases = vec![
("name=le%20guin", "missing the email"),
("email=ursula_le_guin%40gmail.com", "missing the name"),
("", "missing both name and email")
];
for (invalid_body, error_message) in test_cases {
// Act
let response = client
.post(&format!("{}/subscriptions", &app_address))
.header("Content-Type", "application/x-www-form-urlencoded")
.body(invalid_body)
.send()
.await
.expect("Failed to execute request.");
// Assert
assert_eq!(
400,
response.status().as_u16(),
// Additional customised error message on test failure
"The API did not fail with 400 Bad Request when the payload was {}.",
error_message
);
}
}


#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
// Arrange
let app_address = spawn_app();
let configuration = get_configuration().expect("Failed to read configuration");
let connection_string = configuration.database.connection_string();
// The `Connection` trait MUST be in scope for us to invoke
// `PgConnection::connect` - it is not an inherent method of the struct!
let connection = PgConnection::connect(&connection_string)
.await
.expect("Failed to connect to Postgres.");
let client = reqwest::Client::new();
// Act
let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
let response = client
.post(&format!("{}/subscriptions", &app_address))
.header("Content-Type", "application/x-www-form-urlencoded")
.body(body)
.send()
.await
.expect("Failed to execute request.");
// Assert
assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
// [...]
// The connection has to be marked as mutable!
let mut connection = ...
// Assert
assert_eq!(200, response.status().as_u16());
let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
.fetch_one(&mut connection)
.await
.expect("Failed to fetch saved subscription.");
assert_eq!(saved.email, "ursula_le_guin@gmail.com");
assert_eq!(saved.name, "le guin");
}