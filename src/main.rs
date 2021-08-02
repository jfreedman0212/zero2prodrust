use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{get_configuration, run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration file");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to establish connection to Postgres");
    run(listener, connection_pool)?.await
}
