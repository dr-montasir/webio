```rust
use webio::*;

// --- Implementation Examples ---

/// A custom 404 handler that serves a styled HTML page.
/// Automatically selected by WebIO when a browser (Accept: text/html) hits a missing route.
async fn my_custom_html_404(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::NotFound)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("<h1 style='color:red;'>🛸 404 - That page doesn't exist on WebIo!</h1>")
}

/// A custom 404 handler that serves a JSON error.
/// Automatically selected for API clients or tools like `curl`.
async fn my_custom_json_404(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::NotFound)
        .header("Content-Type", "application/json")
        .body("{\"error\": \"not_found\", \"code\": 404, \"source\": \"WebIo API\"}")
}

/// Demonstrates basic GET routing.
async fn hello_get_handler(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("<h1>👋 from WebIO Framework</h1>")
}

/// Demonstrates dynamic path parameters using `<name>`.
/// Extracted via the `Params` collection.
async fn user_handler(_req: Req, params: Params) -> Reply {
    let name = params.0.get("name").cloned().unwrap_or("Guest".to_string());
    Reply::new(StatusCode::Ok).
        header("Content-Type", "text/html; charset=UTF-8")
            .body(format!("<h1>Hello 👋, {}!</h1>", name))
}

/// A specialized handler for numeric IDs or other dynamic segments.
async fn id_handler(_req: Req, params: Params) -> Reply {
    let id = params.0.get("id").cloned().unwrap_or("0".to_string());
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(format!("<h1>👋 ID: {}</h1>", id))
}

/// Demonstrates handling POST data directly from the `Req` struct.
async fn create_user_handler(req: Req, _params: Params) -> Reply {
    // Access the POST body directly
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(format!("<h1>👋 User Created with Data: {}</h1>", req.body))
}

/// A typical API endpoint returning JSON content.
async fn status_handler(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "application/json")
        .body("{\"status\": \"online\"}")
}

/// A protected resource example. Access is controlled by the middleware defined in `main`.
async fn secret_handler(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("<h1>🔓 Access Granted: Welcome Boss! 💎</h1>")
}

fn main() {
    // Ignition: Launches the Safe-Turbo executor to drive the async application loop.
    launch(async {
        let mut app = WebIo::new();

        // 1. Smart 404 Registration: Register 404 Handlers (HTML & JSON)
        // WebIO sniffs these handlers to decide which one to use based on 'Accept' headers.
        app.on_404(my_custom_html_404);
        app.on_404(my_custom_json_404);

        // 2. Secret Key Middleware
        // This runs before routing, allowing for early rejection of unauthorized requests.
        app.use_mw(|path| {
            if path.contains("/secret") {
                if path.ends_with("key=boss") {
                    println!("✅ Auth: Key accepted.");
                    return None; // Continue to route handler
                } else {
                    println!("❌ Auth: Access Denied");
                    return Some(Reply::new(StatusCode::Unauthorized)
                        .header("Content-Type", "text/html; charset=UTF-8")
                        .body("<h1>🚫 Access Denied: Invalid Key</h1>"));
                }
            }
            None
        });

        // 3. Define Routes:
        // Routing Table:
        // Supports standard methods and dynamic segments like <id>.
        app.route(GET, "/", hello_get_handler);
        app.route(GET, "/status", status_handler);
        app.route(GET, "/user/<name>", user_handler);
        app.route(GET, "/req/<id>", id_handler);
        app.route(POST, "/user/create", create_user_handler);
        app.route(POST, "/secret", secret_handler);

        // 4. Server Start
        // This will block the current thread and spawn worker threads for each connection.
        app.run("127.0.0.1", "8080").await;
    });
}

// In local environments, WebIo consistently achieves response times in the 
// **70µs - 400µs** range (e.g., 29.01.2026 `[10:48:50] GET / -> 200 (70.8µs)`) 
// without using any `unsafe` code.

// 🦅 WebIo Live: http://127.0.0.1:8080
// [10:48:43] GET / -> 200 (382µs)
// [10:48:44] GET / -> 200 (415.8µs)
// [10:48:45] GET / -> 200 (348.7µs)
// [10:48:46] GET / -> 200 (382.3µs)
// [10:48:46] GET / -> 200 (1.0754ms)
// [10:48:47] GET / -> 200 (420.6µs)
// [10:48:48] GET / -> 200 (150µs)
// [10:48:49] GET / -> 200 (733.5µs)
// [10:48:49] GET / -> 200 (268.6µs)
// [10:48:49] GET / -> 200 (195.1µs)
// [10:48:49] GET / -> 200 (123.5µs)
// [10:48:50] GET / -> 200 (24.842ms)
// [10:48:50] GET / -> 200 (70.8µs)  *****
// [10:48:50] GET / -> 200 (216.1µs)
// [10:48:50] GET / -> 200 (83.8µs)  *****
// [10:48:51] GET / -> 200 (362.3µs)
// [10:49:04] GET / -> 200 (228.7µs)
// [10:49:05] GET / -> 200 (390.9µs)
// [10:49:06] GET / -> 200 (286.2µs)
// [10:49:06] GET / -> 200 (141.1982ms)
// [10:49:07] GET / -> 200 (476.2µs)
// [10:49:07] GET / -> 200 (193.8µs)
// [10:49:08] GET / -> 200 (217.8µs)
// [10:49:09] GET / -> 200 (159.5µs)
// [10:49:09] GET / -> 200 (102.5µs)
// [10:49:09] GET / -> 200 (441.5µs)
// [10:49:10] GET / -> 200 (252.6µs)
// [10:49:14] GET /status -> 200 (327.2µs)
// [10:49:16] GET /status -> 200 (347.2µs)
// [10:49:17] GET /status -> 200 (96.462ms)
// [10:49:18] GET /status -> 200 (317.1µs)
// [10:49:18] GET /status -> 200 (291.9µs)
// [10:49:19] GET /status -> 200 (703.1µs)
// [10:49:20] GET /status -> 200 (365.7µs)
// [10:49:20] GET /status -> 200 (361.7µs)
// [10:49:21] GET /status -> 200 (116.9µs)
// [10:49:21] GET /status -> 200 (156.7µs)
// [10:49:37] GET /status12 -> 404 (293.7µs)
// [10:49:39] GET /status12 -> 404 (320.6µs)
// [10:49:40] GET /status12 -> 404 (118.5µs)
// [10:49:44] GET /status -> 200 (1.031ms)
// [10:49:45] GET /status -> 200 (216.1µs)
// [10:49:46] GET /status -> 200 (797.3µs)
// [10:49:46] GET /status -> 200 (379.3µs)
// [10:49:46] GET /status -> 200 (415µs)
// [10:49:47] GET /status -> 200 (117µs)
// [10:49:47] GET /status -> 200 (130.1317ms)
// [10:49:47] GET /status -> 200 (94.6µs)
// [10:49:50] GET / -> 200 (124.2µs)
// [10:49:52] GET / -> 200 (792.3µs)
// [10:49:52] GET / -> 200 (359.1µs)
// [10:49:52] GET / -> 200 (526.6µs)
// [10:49:53] GET / -> 200 (276.1µs)
// [10:49:53] GET / -> 200 (328.5µs)
// [10:49:53] GET / -> 200 (100.4µs)
// [10:49:53] GET / -> 200 (265.7µs)
// [10:49:53] GET / -> 200 (278.2µs)
// [10:49:53] GET / -> 200 (385.2µs)
// [10:49:54] GET / -> 200 (120.1µs)
```