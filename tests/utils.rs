use std::net::TcpListener;

/// Helper function that spins up our application in a background task (using Tokio)
pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Cannot bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
