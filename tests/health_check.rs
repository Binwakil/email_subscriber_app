#[tokio::test]
async fn health_check_works() {
// Arrange
spawn_app().await.expect("Failed to spawn our app.");
// We need to bring in `reqwest`
// to perform HTTP requests against ou
// Act
let response = client
.get("http://127.0.0.1:8000/health_check")
.send()
.await
.expect("Failed to execute request.");
// Assert
assert!(response.status().is_success());
assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
async fn spawn_app() -> std::io::Result<()> {
todo!()
}