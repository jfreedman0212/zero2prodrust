use std::net::TcpListener;

/// Tests that we can hit the /health_check endpoint successfully
/// and that the response is correct
#[actix_rt::test]
async fn health_check_works() {
    // Arrange (create the app and the request client)
    let base_url = spawn_app();
    let client = reqwest::Client::new();

    // Act (send the request and track the response)
    let response = client
        .get(format!("{}/health_check", &base_url))
        .send()
        .await
        .expect("Failed to execute Health Check Request");

    // Assert (200 status code with empty response body)
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/// Helper function that spins up our application in a background task (using Tokio)
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Cannot bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
