use sqlx::{Connection, PgConnection};
use zero2prod::get_configuration;

mod utils;

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let base_url = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=josh%20freedman&email=joshfreedman%40pm.me";
    let config = get_configuration().expect("Cannot read from config file");
    let mut connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Cannot connect to Postgres");

    // Act
    let response = client
        .post(format!("{}/subscriptions", base_url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute subscriptions request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT Email, Name FROM Subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email, "joshfreedman@pm.me");
    assert_eq!(saved.name, "josh freedman");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let base_url = utils::spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=josh%20freedman", "Missing email address"),
        ("email=joshfreedman%40pm.me", "Missing name"),
        ("", "Missing both email address and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", base_url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute subscriptions request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with the payload {}",
            error_message
        );
    }
}
