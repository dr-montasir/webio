<div align="center">
  <br>
  <a href="https://github.com/dr-montasir/webio" target="_blank">
      <img src="https://github.com/dr-montasir/webio/raw/HEAD/webio-logo/logo/webio-logo-288x224-no-text-no-bg.svg" width="100">
  </a>
  <br>
</div>

# WebIO 🦅

> **A minimalist, high-performance web framework for Rust built with a zero-dependency philosophy.**
**Built exclusively on the Rust Standard Library (`std`), WebIO achieves an ultra-light footprint, lightning-fast compilation, and a transparent security audit trail.**

<div style="text-align: center;">
  <a href="https://github.com/dr-montasir/webio"><img src="https://img.shields.io/badge/github-dr%20montasir%20/%20webio-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24" style="margin-top: 10px;" alt="github" /></a> <a href="https://crates.io/crates/webio"><img src="https://img.shields.io/crates/v/webio.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24" style="margin-top: 10px;" alt="crates.io"></a> <a href="https://docs.rs/webio"><img src="https://img.shields.io/badge/docs.rs-webio-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24" style="margin-top: 10px;" alt="docs.rs"></a> <a href="https://choosealicense.com/licenses/mit"><img src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555" height="24" style="margin-top: 10px;" alt="license"></a>
</div>

---
## Why WebIO?

WebIO is a zero-dependency Rust web framework engineered to prevent "event loop freeze" during heavy calculations. By relying strictly on the **Rust Standard Library**, it prioritizes memory predictability and raw performance for data science and computational tasks.
This minimalist engine utilizes dedicated **OS threads** for each connection to ensure non-blocking, pre-emptive multitasking. Featuring a "Safe-Turbo" executor with a custom spin-loop for ultra-low latency and O(1) memory complexity for big data ingestion, WebIO provides a high-integrity implementation for high-stakes environments.

### 🔬 The Rationale: WebIO vs. External Runtimes (Tokio/Hyper)

Most modern Rust web frameworks (like Axum or Actix) are built on top of **Tokio**, a high-performance asynchronous event-loop. While excellent for handling millions of *idle* connections (like a chat app), Tokio faces a "Cooperative Scheduling" challenge in **Data Science** and **Computational** environments.

### 🧠 The "Event Loop Freeze" Problem
In a traditional async runtime, tasks must "yield" voluntarily to the executor. If a single request performs a heavy 10-second mathematical calculation—such as a regression model, matrix inversion, or a large CSV parse—the **entire event loop thread is blocked**. This creates a "Stop-the-World" scenario where one heavy CPU task freezes the responses for all other users on that executor.

### 🚀 The WebIO Solution: Go-Inspired OS Pre-emption
WebIO is engineered as a specialized core engine for projects like **Fluxor**, where mathematical precision and memory predictability are the highest priority. 

**WebIO provides a Go-like concurrency experience** but utilizes raw **OS Threads** for true kernel-level isolation. This ensures high efficiency for both small JSON responses and large "Big Data" streaming.

* **The Safety Distinction:** While WebIO adopts the **"goroutine-per-request model"** strategy found in Go, it enforces **Rust’s Ownership Model**. This provides the concurrency simplicity of Go without the unpredictable latency of a Garbage Collector (GC). Deterministic memory management ensures no "GC pauses" occur during heavy mathematical calculations, providing consistent performance for high-stakes computational tasks.

* **OS-Level Isolation:** Utilizing **OS Threads** managed by the Kernel achieves pre-emptive multitasking. If one handler is 100% occupied by heavy math on Core 1, the OS automatically ensures other threads remain responsive on other cores. This architecture eliminates the risk of "blocking the loop."
* **Safe-Turbo Bridge:** While WebIO uses OS threads for isolation, a specialized **`block_on`** executor allows the use of `async` logic (like calling an external API or database) inside threads without the bloat of a massive dependency tree.
* **Zero-RAM Big Data:** Raw `TcpStream` access enables moving 100GB+ datasets directly from the socket to disk in 64KB chunks. This bypasses the 10MB RAM safety guard, ensuring the engine remains stable under massive data ingestion.

## 🛠️ Performance Architecture
* **Hybrid Spin-Wait Strategy:** The `block_on` executor uses a 150k-cycle spin-loop to catch I/O ready states in nanoseconds, maintaining sub-millisecond tail latency by bypassing OS scheduler jitter for "hot" tasks.
* **Smart RAM Cache:** Transparently caches hot assets (<500KB) using an `RwLock` to provide **~50µs** RAM-speed delivery for CSS/JS/JSON, while large files stream safely from the Disk.
* **Zero-Dependency Integrity:** By strictly avoiding external crates, WebIO is immune to "supply chain attacks" and remains a pure, high-performance core for Computing Science and Mathematical applications.
* **Nagle Control:** Granular builder-pattern control over TCP throughput vs. latency (set_nagle) optimizes for either real-time APIs or massive data syncs.

## 🧪 Research Status: Experimental
**WebIO** is currently in an active **research and development phase**. 
- **API Stability:** Expect breaking changes as we refine the core engine.
- **Goal:** To provide a high-integrity core engine for **Data Science**, **Computing Science**, and frameworks like **Fluxor**.
- **Production Warning:** Use with caution until the stable `1.0.0` released.

## 🎯 Core Philosophy
WebIO provides a fully functional web engine with **zero external dependencies**. By strictly utilizing the Rust std library, it ensures an ultra-light footprint, rapid compilation, and total memory predictability.
**zero external dependencies**. 
- **No** `tokio`, `async-std` or `hyper`.
- **No** `serde` or heavy middleware bloat.
- **No** `unsafe`  code in the executor.
- **Just pure, high-performance Rust**.

## 🛠️ Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
webio = "0.7.0-alpha"
```

## 🚀 What's New in v0.5.0-alpha
The `v0.5.0-alpha` release represents a major architectural shift from a single-threaded async model to a **High-Performance Multi-threaded Hybrid Engine** inspired by the Go concurrency model.

## 🧵 Go-Inspired Multi-threading
WebIO now mimics the Go strategy by spawning a unique **OS Thread** for every incoming connection. This ensures **Pre-emptive Multitasking**: a heavy mathematical calculation on one thread will never "freeze" the server for other users—a common pitfall in traditional async runtimes like Tokio.

## ⚡ Safe-Turbo block_on Executor
We have replaced the legacy runtime with a specialized **Safe-Turbo Bridge**. It utilizes a **150,000-cycle spin-loop** to catch I/O ready states in nanoseconds. This bypasses OS scheduler jitter, maintaining sub-millisecond tail latency (~200µs) while remaining 100% safe.

## 🛡️ RAM Safety & Big Data Streaming
- **10MB Safety Valve**: A pre-emptive RAM guard protects the heap by rejecting massive payloads before they are ingested.
- **Zero-RAM Ingestion**: Handlers now have direct access to the `TcpStream`, allowing multi-gigabyte files (CSVs, Videos, Datasets) to be moved directly to disk in 64KB chunks with O(1) **memory complexity**.

## 🛰️ Real-Time WebSockets & Global Broadcast
Full **RFC 6455** compliance is now built-in. WebIO handles the cryptographic handshake (SHA-1/Base64) and binary framing manually. The new **Global Client Registry** allows any thread to broadcast messages to all connected users simultaneously.

## 🧠 Smart Asset Caching
Introduced an **`RwLock`** **RAM Cache** for static assets. Small files (<500KB) are served at RAM speeds (~50µs), while larger files are intelligently served via the OS Page Cache to prevent memory exhaustion.

## 📈 Dynamic Nagle Control
New builder-pattern support for `set_nagle(bool)`. Toggle between **Low Latency** (TCP_NODELAY) for real-time APIs or **High Throughput** for massive data synchronization.

## 🧬 Concurrency Comparison

| Feature          | Traditional Async (Tokio)  | WebIO v0.5.0 (OS Threads)   |
| ---------------- | -------------------------- | --------------------------- |
| **Heavy Math**   | ❌ Blocks the Event Loop    | ✅ Isolated per CPU Core     |
| **Tail Latency** | ⚠️ Variable (Task Stealing) | ✅ Ultra-Low (Spin-Wait)     |
| **Memory**       | ⚠️ Managed by Runtime       | ✅ Deterministic (Ownership) |
| **Dependencies** | ❌ Hundreds                 | ✅ **Zero (std only)**       |

## 🚀 Big Data POST Examples

WebIO limits standard `POST` data to **10MB** for RAM safety, requiring the use of `req.stream` for **zero-RAM ingestion** of large datasets exceeding this limit. This approach ensures O(1) memory complexity, allowing files of any size (e.g., 10GB+ CSVs or videos) to be streamed directly to disk, keeping RAM usage flat. For more information, visit [WebIO documentation](https://docs.rs/webio/latest/webio).

```rust,no_run
use webio::*;

use std::io::{Write, Read, BufWriter};
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

/// Video upload handler
async fn handle_video_upload(mut req: Req, _params: Params) -> Reply {
    // 1. Create the destination file
    let file_path = "uploads/video_data.mp4";
    let file = match File::create(file_path) {
        Ok(f) => f,
        Err(_) => return Reply::new(StatusCode::InternalError).body("Could not create file"),
    };

    let mut writer = BufWriter::new(file);
    let mut buffer = [0; 65536]; // 64KB Chunk (High Throughput)
    let mut total_bytes = 0;

    // 2. Stream directly from the RAW socket (req.stream)
    // This bypasses the 10MB RAM limit entirely!
    while let Ok(n) = req.stream.read(&mut buffer) {
        if n == 0 { break; } // Socket closed or upload finished
        if let Err(_) = writer.write_all(&buffer[..n]) {
            return Reply::new(StatusCode::InternalError).body("Disk Write Error");
        }
        total_bytes += n as u64;
    }

    let _ = writer.flush();

    Reply::new(StatusCode::Ok)
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"status": "success", "bytes_saved": {}}}"#, total_bytes))
}

/// Chat handler
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

