# 1. Overview

WebIO addresses the `"Event Loop Freeze"` common in traditional asynchronous runtimes. By utilizing **OS-level pre-emption**, it ensures that heavy mathematical calculations or massive data parses do not block the responsiveness of the server.

## 1-1. Key Technical Specs

- **Zero Dependencies:** No `tokio`, `hyper`, or `serde` in the core.
- **Performance:** ~50µs – 150µs response latency for cached assets.
- **Memory Safety:** O(1) memory complexity for multi-gigabyte streams.
- **Concurrency:** Go-inspired multi-threading using native OS threads.

## 1-2. Why WebIO?

WebIO is a zero-dependency Rust web framework engineered to prevent "event loop freeze" during heavy calculations. By relying strictly on the **Rust Standard Library**, it prioritizes memory predictability and raw performance for data science and computational tasks. This minimalist engine utilizes dedicated **OS threads** for each connection to ensure non-blocking, pre-emptive multitasking. Featuring a "Safe-Turbo" executor with a custom spin-loop for ultra-low latency and O(1) memory complexity for big data ingestion, WebIO provides a high-integrity implementation for high-stakes environments.

## 1-3. Rationale

**WebIO vs. External Runtimes (Tokio/Hyper)**

Most modern Rust web frameworks (like `Axum` or `Actix`) are built on top of `Tokio`, a high-performance asynchronous event-loop. While excellent for handling millions of idle connections (like a chat app), Tokio faces a "Cooperative Scheduling" challenge in `Data Science` and `Computational` environments.

## 1-4. Event Loop Freeze Problem

In a traditional async runtime, tasks must "yield" voluntarily to the executor. If a single request performs a heavy 10-second mathematical calculation—such as a regression model, matrix inversion, or a large CSV parse—the **entire event loop thread is blocked**. This creates a "Stop-the-World" scenario where one heavy CPU task freezes the responses for all other users on that executor.

## 1-5. WebIO Solution

WebIO is engineered as a specialized core engine for projects like `[Fluxor](https://crates.io/crates/fluxor)`, where mathematical precision and memory predictability are the highest priority.

**Go-Inspired OS Pre-emption**

**WebIO** provides a `Go-like concurrency experience` but utilizes raw **OS Threads** for true kernel-level isolation. This ensures high efficiency for both small JSON responses and large "Big Data" streaming.

- **Safety Distinction:** While WebIO adopts the `"goroutine-per-request model"` strategy found in Go, it enforces `Rust’s Ownership Model`. This provides the concurrency simplicity of Go without the unpredictable latency of a Garbage Collector (GC). Deterministic memory management ensures no "GC pauses" occur during heavy mathematical calculations, providing consistent performance for high-stakes computational tasks.

- **OS-Level Isolation:** Utilizing `OS Threads` managed by the Kernel achieves pre-emptive multitasking. If one handler is 100% occupied by heavy math on Core 1, the OS automatically ensures other threads remain responsive on other cores. This architecture eliminates the risk of "blocking the loop."

- **Safe-Turbo Bridge:** While WebIO uses OS threads for isolation, a specialized **`block_on`** executor allows the use of **`async`** logic (like calling an external API or database) inside threads without the bloat of a massive dependency tree.

- **Zero-RAM Big Data:** Raw **`TcpStream`** access enables moving 100GB+ datasets directly from the socket to disk in 64KB chunks. This bypasses the 10MB RAM safety guard, ensuring the engine remains stable under massive data ingestion.

## 1-6. Performance Architecture

- **Hybrid Spin-Wait Strategy:** The **`block_on`** executor uses a 150k-cycle spin-loop to catch I/O ready states in nanoseconds, maintaining sub-millisecond tail latency by bypassing OS scheduler jitter for "hot" tasks.
- **Smart RAM Cache:** Transparently caches hot assets (<500KB) using an **`RwLock`** to provide ~50µs RAM-speed delivery for CSS/JS/JSON, while large files stream safely from the Disk.
- **Zero-Dependency Integrity:** By strictly avoiding external crates, WebIO is immune to "supply chain attacks" and remains a pure, high-performance core for Computing Science and Mathematical applications.
- **Nagle Control:** Granular builder-pattern control over TCP throughput vs. latency (set_nagle) optimizes for either real-time APIs or massive data syncs.

---