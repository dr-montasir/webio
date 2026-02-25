```rust
use webio::*;

use std::io::{BufWriter};
use std::fs::File;

const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>WebIO | High-Performance Rust</title>
    <link rel="icon" href="/images/favicon.ico" type="image/x-icon" />
    <link rel="stylesheet" href="/css/styles.css" />
</head>
<body>
    <main>
        <h1>👋 Welcome to WebIO</h1>
        <p>A zero-dependency, ultra-low-latency web framework.</p>
        
        <section class="gallery">
            <img src="/images/logo.svg" alt="WebIO Logo" width="150" />
        </section>
    </main>
    <script src="/js/script.js"></script>
</body>
</html>"##;

// --- Implementation Examples ---

/// Demonstrates basic GET routing.
async fn hello_get_handler(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(INDEX_HTML)
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

use std::io::{Write, Read};

async fn handle_big_csv(mut req: Req, _params: Params) -> Reply {
    let mut file = std::fs::File::create("huge_dataset.csv").unwrap();
    // Use a small 64KB buffer to move 10GB of data
    let mut buffer = [0; 65536]; 
    while let Ok(n) = req.stream.read(&mut buffer) {
        if n == 0 { break; }
        file.write_all(&buffer[..n]).unwrap();
    }
    Reply::new(StatusCode::Ok).body("Upload Complete")
}

async fn handle_video_upload(mut req: Req, _params: Params) -> Reply {
    // 1. Create the destination file
    let file_path = "uploads/video_data.mp4";
    let file = match File::create(file_path) {
        Ok(f) => f,
        Err(_) => return Reply::new(StatusCode::InternalServerError).body("Could not create file"),
    };

    let mut writer = BufWriter::new(file);
    let mut buffer = [0; 65536]; // 64KB Chunk (High Throughput)
    let mut total_bytes = 0;

    // 2. Stream directly from the RAW socket (req.stream)
    // This bypasses the 10MB RAM limit entirely!
    while let Ok(n) = req.stream.read(&mut buffer) {
        if n == 0 { break; } // Socket closed or upload finished
        if let Err(_) = writer.write_all(&buffer[..n]) {
            return Reply::new(StatusCode::InternalServerError).body("Disk Write Error");
        }
        total_bytes += n as u64;
    }

    let _ = writer.flush();

    Reply::new(StatusCode::Ok)
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"status": "success", "bytes_saved": {}}}"#, total_bytes))
}

async fn chat_handler(req: Req, _params: Params) -> Reply {
    // 1. Upgrade to WebSocket
    if let Some(mut ws) = req.upgrade_websocket() {
        
        // 2. Register the connection in our Global Registry
        if let Ok(mut clients) = CLIENTS.lock() {
            if let Ok(clone) = ws.try_clone() {
                clients.push(clone);
            }
        }

        // 3. Enter the persistent Loop (Isolated on this OS Thread)
        loop {
            // Read binary frames from the browser
            match WSFrame::read(&mut ws) {
                Ok(frame) => {
                    let msg = String::from_utf8_lossy(&frame.payload);
                    // Broadcast to ALL other threads
                    broadcast(&format!("User {}: {}", msg, req.path));
                }
                Err(_) => break, // Disconnect
            }
        }
    }
    
    // 4. Return empty (Stream is closed or hijacked)
    Reply::new(StatusCode::Ok).body("")
}

fn main() {
    let mut app = WebIo::new();
    app.log_reply_enabled = true;
    app.use_static("assets");

    app.on_404(my_custom_html_404);
    app.on_404(my_custom_json_404);

    // --- Routes ---
    app.route(GET, "/", hello_get_handler);
    app.route(GET, "/status", status_handler);
    app.route(GET, "/user/create", create_user_handler);
    app.route(GET, "/secret-handler", secret_handler);
    app.route(GET, "/user/<name>", user_handler);
    app.route(GET, "/req/<id>", id_handler);
    
    // WebSockets & Big Data (Persistent/Heavy)
    app.route(GET, "/chat", chat_handler); 
    app.route(POST, "/csv", handle_big_csv);
    app.route(POST, "/video-upload", handle_video_upload);

    // --- Start ---
    // Start the Multi-Threaded TCP Listener

    // // set false or true
    app.set_nagle(true)
       .run("0.0.0.0", "8080");

    // // set false or true
    // app = app.set_nagle(false);
    // app.run("0.0.0.0", "8080");

    // Default value nagle is on/true
    // app.run("0.0.0.0", "8080");
}
```