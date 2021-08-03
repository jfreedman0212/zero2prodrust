use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{get_configuration, DatabaseSettings};

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
    let mut config = get_configuration().expect("Failed to read configuration file");
    config.database.database_name = Uuid::new_v4().to_string();
    // build the connection pool
    let connection_pool = configure_database(&config.database).await;
    // spin up the server and run it as a background task
    let server = zero2prod::run(listener, connection_pool.clone()).expect("Failed to start server");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        connection_pool,
    }
}

/// Helper function that spins up a new logical database
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create a new logical database with a random name for testing purposes
    let mut connection = PgConnection::connect(&config.connection_string_without_db_name())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database");
    // set up the connection pool
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to establish connection to Postgres");
    // run all migrations on this database
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate");
    // return the connection pool
    connection_pool
}
