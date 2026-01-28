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
    // The entry point uses our custom executor to block on the async app loop.
    execute(async {
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
// **50µs - 150µs** range (e.g., `[00:50:18] GET / -> 200 (56.9µs)`).

// 🦀 WebIo Live: http://127.0.0.1:8080
// [00:50:09] GET / -> 200 (802.1µs)
// [00:50:14] GET / -> 200 (988.6µs)
// [00:50:16] GET / -> 200 (153.9µs)
// [00:50:16] GET / -> 200 (238.5µs)
// [00:50:18] GET / -> 200 (56.9µs)     *****
// [00:50:18] GET / -> 200 (150.5377ms)
// [00:50:18] GET / -> 200 (834.8µs)
// [00:50:19] GET / -> 200 (103.533ms)
// [00:50:28] GET / -> 200 (833.9µs)
// [00:50:28] GET / -> 200 (141.6603ms)
// [00:50:29] GET / -> 200 (289.9µs)
// [00:50:30] GET / -> 200 (970.6µs)
// [00:50:30] GET / -> 200 (77.3072ms)
// [00:50:31] GET / -> 200 (205.7µs)
// [00:50:31] GET / -> 200 (94.2µs)
// [00:50:45] GET / -> 200 (1.0149ms)
// [00:50:52] GET / -> 200 (802µs)
// [00:50:53] GET / -> 200 (336.5µs)
// [00:50:53] GET / -> 200 (97µs)
// [00:51:09] GET / -> 200 (440.3µs)
// [00:51:15] GET / -> 200 (335.2µs)
// [00:51:16] GET / -> 200 (55.5µs)
// [00:51:16] GET / -> 200 (38.0655ms)
// [00:51:16] GET / -> 200 (175.6µs)
// [00:51:17] GET / -> 200 (452.5µs)
// [00:51:18] GET / -> 200 (228.4µs)
// [00:51:18] GET / -> 200 (36.4482ms)
// [00:51:20] GET / -> 200 (600.6µs)


// 🦀 WebIo Live: http://127.0.0.1:8080
// [03:49:31] GET / -> 200 (781.9µs)
// [03:49:32] GET / -> 200 (336.8µs)
// [03:49:33] GET / -> 200 (112.2µs)
// [03:49:33] GET / -> 200 (106.5µs)
// [03:49:33] GET / -> 200 (385.6µs)
// [03:49:33] GET / -> 200 (33.6601ms)
// [03:49:34] GET / -> 200 (1.1269ms)
// [03:49:34] GET / -> 200 (121.1µs)
// [03:49:34] GET / -> 200 (101.6µs)
// [03:49:34] GET / -> 200 (87.9µs)
// [03:49:35] GET / -> 200 (115.3µs)
// [03:49:35] GET / -> 200 (59.5434ms)   *****
// [03:49:35] GET / -> 200 (238.1µs)
```