mod configuration;
mod routes;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use routes::{health_check, subscribe};
use sqlx::PgPool;
use std::net::TcpListener;

/// Spins up the application, provided with a TCP Listener to bind to
pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub use configuration::*;
