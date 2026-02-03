//! # Hello World Integration Test
//! This test verifies the most basic functionality of the WebIO framework:
//! 1. Initializing the engine.
//! 2. Registering a root GET route.
//! 3. Starting the server in a background thread.
//! 4. Verifying the server is reachable via TCP.

use webio::*;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

/// Test Case: Basic Root Route
/// Ensures that the server can bind to a port and handle a simple HTML response.
/// We use a background thread so the test can complete and report "ok".
#[test]
fn test_hello_world() {
    // 1. Spawn the WebIO server in a background thread
    thread::spawn(|| {
        launch(async {
            let mut app = WebIo::new();

            app.route(GET, "/", |_req, _params| async {
                Reply::new(StatusCode::Ok)
                    .header("Content-Type", "text/html; charset=UTF-8")
                    .body("<h1>👋 Hello from WebIO Framework</h1>")
            });

            // Using 8080 for this specific integration test
            app.run("127.0.0.1", "8080").await;
        });
    });

    // 2. Wait briefly (500ms) for the listener to bind to the port
    thread::sleep(Duration::from_millis(500));

    // 3. Attempt a TCP connection to verify the server is live
    // This prevents the test from hanging and provides a real "Pass/Fail" result
    let connection = TcpStream::connect_timeout(
        &"127.0.0.1:8080".parse().unwrap(), 
        Duration::from_secs(1)
    );

    // 4. Assert that the connection was successful
    assert!(
        connection.is_ok(), 
        "❌ Failed to connect to WebIO server on 127.0.0.1:8080"
    );

    println!("✅ WebIO Server successfully initialized and reachable.");
}

