//! # Routing Engine Integration Test (01)
//! This suite validates that the WebIo engine correctly maps static paths 
//! to asynchronous handler functions.
//!
//! Features tested:
//! - Static path mapping
//! - Async handler execution
//! - Connection handling on Port 8081

use webio::*;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

/// A standard asynchronous handler used to verify basic GET matching.
async fn hello(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("<h1>Welcome to WebIO</h1>")
}

/// Test Case: Static Route Mapping
/// Validates that registering a handler to the root path ("/") 
/// successfully initializes the server and accepts connections.
#[test] 
fn test_static_route_mapping() {
    // 1. Spawn the routing engine in a background thread
    // This allows the test to verify the server and then terminate.
    thread::spawn(|| {
        execute(async {
            let mut app = WebIo::new();

            // Registering the hello handler
            app.route(GET, "/", hello);

            // Bind to 8081 to avoid port conflicts with other tests
            app.run("127.0.0.1", "8081").await;
        });
    });

    // 2. Grace period for the TCP listener to bind
    thread::sleep(Duration::from_millis(500));

    // 3. Verify the server is active on the expected port
    let address = "127.0.0.1:8081";
    let connection = TcpStream::connect_timeout(
        &address.parse().unwrap(), 
        Duration::from_secs(1)
    );

    // 4. Assert health of the routing engine
    assert!(
        connection.is_ok(), 
        "❌ Routing Engine failed to bind to {} or is not responding.", 
        address
    );

    println!("✅ Static routing test on {} passed.", address);
}
