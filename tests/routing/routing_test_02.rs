//! # Async State Integration Test (02)
//! This suite validates that the WebIo engine can safely capture and use 
//! external variables from the parent scope using the `async move` pattern.
//!
//! Features tested:
//! - Ownership transfer into the async executor
//! - Server lifecycle on Port 8082

use webio::*;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

/// Test Case: Async Move and State Capture
/// Ensures that variables from the main thread can be moved into the 
/// server execution block and remain valid during the server's lifecycle.
#[test] 
fn test_async_state_capture() {
    let message = String::from("WebIO Test Server State");

    // 1. Spawn the server in a background thread
    // We use 'move' on the closure AND the async block to transfer ownership of 'message'
    thread::spawn(move || {
        execute(async move {
            let mut app = WebIo::new();
            
            // This proves the message was successfully moved into the async engine
            println!("🦀 {} is starting...", message);

            app.route(GET, "/", |_req, _params| async {
                Reply::new(StatusCode::Ok)
                    .header("Content-Type", "text/html; charset=UTF-8")
                    .body("<h1>Welcome to WebIO</h1>")
            });

            // Bind to 8082 to maintain parallel test execution
            app.run("127.0.0.1", "8082").await;
        });
    });

    // 2. Wait for the TCP listener to initialize
    thread::sleep(Duration::from_millis(500));

    // 3. Verify the server is active on Port 8082
    let address = "127.0.0.1:8082";
    let connection = TcpStream::connect_timeout(
        &address.parse().unwrap(), 
        Duration::from_secs(1)
    );

    // 4. Final assertion
    assert!(
        connection.is_ok(), 
        "❌ Server failed to capture state or bind to {}.", 
        address
    );

    println!("✅ Async move and state capture test on {} passed.", address);
}
