mod utils;

/// Tests that we can hit the /health_check endpoint successfully
/// and that the response is correct
#[actix_rt::test]
async fn health_check_works() {
    // Arrange (create the app and the request client)
    let base_url = utils::spawn_app().await;
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
