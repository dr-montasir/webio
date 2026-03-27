# 9. Application Settings

WebIO provides **low-level control** over the underlying TCP stack and memory management. These settings optimize the engine for different workloads, ranging from `real-time messaging` to `massive data ingestion`.

## 9-1. Dynamic Nagle Control

Access to the **Nagle Algorithm** (`TCP_NODELAY`) allows for tuning networking behavior. This configuration is applied within the **Worker Implementation** during the initial connection phase.

**The Strategy: Throughput vs. Latency**

In `handle_connection`, the engine toggles the socket's "no-delay" state based on the global configuration:

```rs
// --- TCP OPTIMIZATION ---
// set_nagle(false) -> Low Latency (Small JSON/HTML, PWA)
// set_nagle(true)  -> High Throughput (Big Data CSVs)
let _ = stream.set_nodelay(!self.nagle_enabled);
```

- **High Throughput (`set_nagle(true)`):** The default mode. It buffers small outgoing packets into larger segments. Best for serving large files or reducing CPU overhead.
- **Ultra-Low Latency (`set_nagle(false)`):** Disables buffering to send every chunk immediately. Best for **WebSockets**, chat interfaces, and interactive APIs.

**Implementation in `main`**

```rust,no_run
use webio::*;

fn main() {
    let mut app = WebIo::new();

    // Option A: Explicitly Enabled (High Throughput)
    app.set_nagle(true).run("0.0.0.0", "8080");

    // Option B: Explicitly Disabled (Low Latency)
    // app.set_nagle(false).run("0.0.0.0", "8080");

    // Option C: Standard Default (Nagle is ON/True)
    // app.run("0.0.0.0", "8080");
}
```

## 9-2. RAM Safety Guards

**(Pre-emptive Heap Protection)**

Strict memory management prevents system exhaustion during high-concurrency or malicious "Big Data" uploads.

**Pre-emptive Rejection (The 10MB Threshold)**

WebIO inspects the `Content-Length` header before allocating memory. If a payload exceeds the **10MB safety limit**, the connection is terminated immediately with a `403 Forbidden` response.

```rust,no_run
use webio::*;

fn main() {
    let mut app = WebIo::new();

    app.route(POST, "/secure-upload", |req, _params| async move {
        // --- 1. DEFINE THRESHOLD ---
        let max_body_size = 10 * 1024 * 1024; // 10MB Threshold

        // --- 2. EXTRACT METADATA ---
        let content_length = req.headers.get("content-length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        // --- 3. ENFORCE GUARD ---
        if content_length > max_body_size {
            return Reply::new(StatusCode::Forbidden)
                .body("Payload Too Large (Max 10MB)");
        }

        Reply::new(StatusCode::Ok).body("Data safe for heap allocation")
    });

    app.run("0.0.0.0", "8080");
}
```

**Protocol Significance**

- **Pre-emptive Rejection:** The connection is evaluated **before** the application attempts to buffer the body, stopping DoS attacks at the header level.
- **Zero-Waste Policy:** If `content_length` exceeds `max_body_size`, the engine returns a 403 Forbidden immediately, preserving CPU and RAM for other concurrent threads.

## 9-3. Thread Isolation Security

WebIO ensures that global settings and high-load tasks never compromise the stability of the entire engine. By moving every connection into a **Dedicated OS Thread**, the framework achieves a level of security and reliability where heavy tasks cannot interfere with lightweight ones.

**Independent Context**

Each connection is isolated at the operating system level. This means if a specific route (like a 10GB video upload) is utilizing **9-1 (High Throughput)** or **9-2 (Streaming)**, it happens in total isolation from other active users.

- **No "Stop-the-World":** A heavy data stream in one thread cannot block the Nagle-disabled "Ultra-Low Latency" response of a Chat WebSocket in another thread.
- **Deterministic Reliability:** Because each worker runs its own `block_on` executor, performance settings applied to the `TcpStream` (like timeouts or Nodelay) are local to that specific connection.

**Performance Guarantee**

By combining **Dynamic Nagle Control** with **Thread Isolation**, the application can simultaneously act as a high-speed API and a heavy-duty file server without manual resource partitioning.

```shell
# --- WORKER IMPLEMENTATION LOGIC ---

# 1. Connection arrives at the listener
# 2. WebIO spawns a dedicated OS Thread for this specific stream
# 3. Isolation ensures this stream's settings don't leak to others
stream.set_nodelay(true/false) 

# 4. Execute the asynchronous handler within a synchronous bridge
# Even if this handler takes 10 minutes (Big Data), 
# the rest of the app remains responsive.
let reply = block_on(handler(req, params))

# 5. Immediate resource reclamation upon completion
```

## 9-4. Streaming Finalization

**(Chunked Transfer & Syscall Optimization)**

WebIO implements a high-performance **Finalization Engine** that serializes metadata and streams body fragments directly to the network interface.

**Performance Architecture**

- **Syscall Minimization:** Utilizes a high-capacity (**64KB**) `BufWriter` to batch headers and initial data fragments. This minimizes the "Context Switch Tax" between user-space and the OS kernel, frequently enabling **50µs - 150µs** response times in local environments.
- **Memory Safety:** By moving data in discrete chunks, the engine prevents the allocation of massive byte vectors on the heap, ensuring a stable RAM profile regardless of file size.
- **HTTP/1.1 Chunked Encoding:** This protocol foundation allows WebIO to deliver "Big Data" without requiring prior knowledge of the total payload size.

**The Streaming Loop**

The engine consumes chunks from the `BodyStream` until exhaustion, wrapping each in the RFC-standard hex-size format:
shell

```shell
# --- PROTOCOL EXECUTION FLOW ---

# 1. Construct Status Line & Protocol Headers
# 2. Push directly into the 64KB Buffered Writer
"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n"

# 3. Enter the BIG DATA STREAMING LOOP
# Protocol Format: [Hex Size]\r\n[Payload]\r\n
"1000\r\n[4096 Bytes of Data]\r\n"
"1000\r\n[4096 Bytes of Data]\r\n"

# 4. Termination: Send RFC-standard zero-length chunk
"0\r\n\r\n"

# 5. Resource Reclamation: Force OS-level socket closure
stream.shutdown(Both)
Use code with caution.
```

**Diagnostic & Telemetry**

Upon the final byte flush, the thread triggers high-resolution telemetry logging. This captures the precise duration from the initial `TcpStream` acceptance to the final socket shutdown, providing developers with clear insight into **Request Latency**.

## 9-5. MIME Type Configuration

**WebIO** includes a built-in registry to map file extensions to their corresponding Media Types. For granular application control, the engine provides methods to add, update, or restrict specific formats. Further implementation details are available in the mime and engine source modules.

**Customizing the Registry**

The registry is modified directly through the `WebIo` instance before calling `.run()`. This allows for the support of modern formats or the disabling of sensitive file types.

- **Set Single Type:** Add or update a specific mapping.
- **Bulk Update:** Apply multiple mappings simultaneously using a vector of tuples.
- **Remove Types:** Disable specific extensions to prevent the server from identifying or serving them with specific headers.

```rust,no_run
use webio::*;

fn main() {
    let mut app = WebIo::new();

    // 1. Add support for modern image formats
    app.set_mime("webp", "image/webp");

    // 2. Bulk update for web assets and fonts
    app.set_mimes(vec![
        ("woff2", "font/woff2"), 
        ("wasm", "application/wasm")
    ]);

    // 3. Security: Disable serving of specific script files
    app.remove_mime("php");

    // 4. Disable multiple video formats at once
    let to_remove = vec!["mp4", "webm", "avi", "mov"];
    app.remove_mimes(to_remove);

    app.run("127.0.0.1", "8080");
}
```

**Internal Resolution & Fallback**

The engine performs case-insensitive lookups during the request cycle. If a file extension (e.g., `.JPG` or `.jpg`) is not found in the custom or default registry, **WebIO** defaults to **`application/octet-stream`** to ensure a safe binary fallback.  For more detailed implementation logic, check the [**`mime`**](https://docs.rs/crate/webio/latest/source/src/mime/mod.rs) and [**`engine`**](https://docs.rs/crate/webio/latest/source/src/engine/mod.rs) modules.

---