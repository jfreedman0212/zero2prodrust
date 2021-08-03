mod utils;

/// Runs a POST request with valid form data and checks the DB to see if
/// the data was actually inserted (to be removed later to keep the black box nature
/// of the tests)
#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange (spin up the app, the request client, and the input data for the form)
    let test_app = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=josh%20freedman&email=joshfreedman%40pm.me";

    // Act (perform the request)
    let response = client
        .post(format!("{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute subscriptions request");

    // Assert (ensure the correct status code, and check the database for the inserted record)
    assert_eq!(200, response.status().as_u16());
    // TODO: remove the DB call in the future if a GET /subscriptions endpoint ever gets added
    let saved = sqlx::query!(
        // language=SQL
        "SELECT Email, Name FROM Subscriptions"
    )
    .fetch_one(&test_app.connection_pool)
    .await
    .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "joshfreedman@pm.me");
    assert_eq!(saved.name, "josh freedman");
}

/// Runs several POST requests with various invalid inputs, attemtping to ensure
/// that all required data is provided
#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange (spin up test app, request client, and define invalid test cases)
    let test_app = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=josh%20freedman", "Missing email address"),
        ("email=joshfreedman%40pm.me", "Missing name"),
        ("", "Missing both email address and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act (run request)
        let response = client
            .post(format!("{}/subscriptions", test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute subscriptions request");

        // Assert (that we got 400 status code)
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with the payload {}",
            error_message
        );
    }
}
