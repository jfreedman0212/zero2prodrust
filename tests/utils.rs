use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::get_configuration;

/// Helper function that spins up our application in a background task (using Tokio)
pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Cannot bind to random port");
    let port = listener.local_addr().unwrap().port();
    let config = get_configuration().expect("Failed to read configuration file");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to establish connection to Postgres");
    let server = zero2prod::run(listener, connection_pool).expect("Failed to start server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
