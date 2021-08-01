/// Tests that we can hit the /health_check endpoint successfully
/// and that the response is correct
#[actix_rt::test]
async fn health_check_works() {
    // Arrange (create the app and the request client)
    spawn_app();
    let client = reqwest::Client::new();

    // Act (send the request and track the response)
    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to execute Health Check Request");

    // Assert (200 status code with empty response body)
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/// Helper function that spins up our application in a background task (using Tokio)
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to start server");
    let _ = tokio::spawn(server);
}
