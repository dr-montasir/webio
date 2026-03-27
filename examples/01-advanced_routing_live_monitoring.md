# Advanced Routing & Live Monitoring

This comprehensive example demonstrates the full power of the **WebIO** framework, featuring dynamic routing, middleware authentication, smart 404 handling, and live request logging.

## Live Output Preview
When running this example, your terminal will provide real-time performance metrics for every request:

```shell
🦅 WebIO Live: http://127.0.0.1:8080
[15:40:42] GET / -> 200 (4.0154ms)
[15:40:59] GET / -> 200 (1.1355ms)
[15:40:59] GET / -> 200 (96µs) <--- High-speed internal execution
[15:42:19] GET /req/123abc -> 200 (746.1µs)
[15:42:29] GET /req/123abc -> 200 (204µs)
```

## Implementation Example

```rust,no_run
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
async fn user_handler(_req: Req, params: Params) -> Reply {
    let name = params.0.get("name").cloned().unwrap_or("Guest".to_string());
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
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

/// A protected resource example controlled by middleware.
async fn secret_handler(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("<h1>🔓 Access Granted: Welcome Boss! 💎</h1>")
}

fn main() {
    // Ignition: Launches the Safe-Turbo executor.
    block_on(async {
        let mut app = WebIo::new();

        // Enable live terminal logging
        app.log_reply_enabled = true;

        // 1. Smart 404 Registration
        app.on_404(my_custom_html_404);
        app.on_404(my_custom_json_404);

        // 2. Secret Key Middleware
        app.use_mw(|path| {
            if path.contains("/secret") {
                if path.ends_with("key=boss") {
                    println!("✅ Auth: Key accepted.");
                    return None; 
                } else {
                    println!("❌ Auth: Access Denied");
                    return Some(Reply::new(StatusCode::Unauthorized)
                        .header("Content-Type", "text/html; charset=UTF-8")
                        .body("<h1>🚫 Access Denied: Invalid Key</h1>"));
                }
            }
            None
        });

        // 3. Define Routes
        app.route(GET, "/", hello_get_handler);
        app.route(GET, "/status", status_handler);
        app.route(GET, "/user/<name>", user_handler);
        app.route(GET, "/req/<id>", id_handler);
        app.route(POST, "/user/create", create_user_handler);
        app.route(POST, "/secret", secret_handler);

        // 4. Server Start
        // Disabling Nagle's algorithm for lower latency on small packets.
        app.set_nagle(false).run("127.0.0.1", "8080");
    });
}
```