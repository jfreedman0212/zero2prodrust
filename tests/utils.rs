use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::get_configuration;

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
}

/// Helper function that spins up our application in a background task (using Tokio)
pub async fn spawn_app() -> TestApp {
    // bind to a random port for testing
    let listener = TcpListener::bind("127.0.0.1:0").expect("Cannot bind to random port");
    let port = listener.local_addr().unwrap().port();
    // load application configuration for the DB connection string
    let config = get_configuration().expect("Failed to read configuration file");
    // build the connection pool
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to establish connection to Postgres");
    // spin up the server and run it as a background task
    let server = zero2prod::run(listener, connection_pool.clone()).expect("Failed to start server");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        connection_pool,
    }
}
