# 10. Extensions

While **WebIO** remains dependency-free at its core, the ecosystem provides optional extensions to enhance developer productivity. These crates are built following the same "Pure Rust" philosophy—prioritizing performance, minimal overhead, and seamless integration with the WebIO engine.

## 10-1. Forge RSX

<a href="https://crates.io/crates/forge-rsx" target="_blank"><b>Forge-rsx</b></a> stands as a high-performance HTML macro engine for Rust. It enables type-safe, JSX-like template authoring directly within source files, compiling them into optimized strings at build time.

**Core Features**

- **`rsx!` Macro:** Provides a declarative syntax for nesting HTML elements.
- **Component Logic:** Supports embedding Rust expressions and variables directly into attributes or body text.
- **Zero-Cost Abstraction:** Transforms templates into efficient string buffers without runtime overhead.

```rs
use webio::*;
use forge_rsx::rsx;

async fn home_page(_req: Req, _params: Params) -> Reply {
    let name = "WebIO User";
    let page = rsx! { btfy2,
        div {
            class: "container",
            h1 { "Welcome, "{name} }
            p { "Fast, dependency-free rendering." }
        }
    };

    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(page)
}

fn main () {
    let mut app = WebIo::new();

    app.route(GET, "/", home_page);

    app.run("127.0.0.1", "8080");
}
```

<div><a href="https://crates.io/crates/forge-rsx" target="_blank">🔗 <b>View forge-rsx Documentation</b></a></div>

## 10-2. WebIo Macros

<a href="https://crates.io/crates/webio_macros" target="_blank"><b>webio_macros</b></a> provides procedural macro support to reduce boilerplate. It introduces attribute-based routing and streamlined handler definitions, making the codebase cleaner for large-scale applications.

**Core Features**

- **`#[webio_main]`:** Enables the definition of an application entry point using standard async fn main() syntax.
- **Turbo Performance:** Wraps code in the **webio::block_on** engine, maintaining 70µs - 400µs response times.
- **`replace!`:** A versatile tool for substituting **`{{key}}`** placeholders in any content.
- **`html!`:** Acts as a semantic alias for web-specific development.

```rs
use webio::*;
use webio_macros::{webio_main, html};

#[webio_main]
async fn main() {
    let mut app = WebIo::new();

    // Enable live performance logging
    app.log_reply_enabled = true;

    // Standard Async Route: No 'block_on' needed inside #[webio_main]
    app.route(GET, "/", |_req, _params| async {
        // Using the 'html!' macro for clean, reactive-style templating
        let content = html!("<h1>Hello from 🦅 {{name}}!</h1>", name = "WebIO");
        
        Reply::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(content)
    });

    // Configuration and Launch
    app.set_nagle(false).run("0.0.0.0", "8080");
}
```

**Live Output Preview**

The terminal provides real-time monitoring of every incoming request. With `app.log_reply_enabled` set to `true`, you get a clear view of the `HTTP method`, `requested path`, `status code`, and precise execution time.

```shell
🦅 WebIO Live: http://0.0.0.0:8080
[16:27:13] GET /no-block-on -> 404 (1.9506ms)
[16:27:17] GET / -> 200 (79.9µs)
[16:27:19] GET / -> 200 (236.4µs)
[16:27:19] GET / -> 200 (97.6863ms)
[16:27:20] GET / -> 200 (139.1µs)
```

**Performance Breakdown**

The Live Output Preview reflects the engine's efficiency:

- **Ultra-Low Latency:** The second request completed in just **`79.9µs`** (microseconds). This represents the raw speed of the WebIO routing engine and the zero-overhead html! macro.
- **Smart 404 Handling:** The request to /no-block-on correctly returned a 404 in 1.95ms, demonstrating active monitoring and routing for missing endpoints.
- **Cold Start vs. Hot Cache:** Variations in time (such as the 97.6ms entry) typically account for initial `TCP handshakes` or browser-side resource loading. Consistent **`139.1µs`** entries confirm the sustained "warm" performance of the Rust binary.

<div><a href="https://crates.io/crates/webio_macros" target="_blank">🔗 <b>View webio_macros Documentation</b></a></div>

## 10-3. Crator

<a href="https://crates.io/crates/crator" target="_blank"><b>Crator</b></a> is the official Rust high-performance toolkit for **`HTTP/HTTPS`** requests, **`JSON`** processing, and **environment management**.

**Core Features**

- **HTTP/HTTPS Clien:** A lightweight, high-performance, and synchronous `HTTP/HTTPS` client for Rust.
- **JSON extractor:** A lightweight, zero-dependency `JSON extractor` designed for maximum performance and efficiency.
- **rsj! macro:** High-performance, zero-dependency **`rsj! macro`** for declarative, JSX-like JSON generation with support for loops and conditionals.
- **Crates.io API Metadata:** High-performance functions to fetch and interact with structured Crates.io API metadata.
- **Env:** Panic-free utilities for safely loading and managing environment variables.

**Directory Structure**

```shell
my_webio_project/
├── .env                <-- Environment Config (External to src/)
├── Cargo.toml          <-- Manifest & Dependencies
├── src/                
│   └── main.rs         <-- WebIO + Crator Logic
└── target/             <-- Compiled Binaries  
```

**Main Application Code**

The following implementation integrates dynamic **User-Agent** identities and environment-driven port configuration.

```rs
use webio::*;
use crator::{Http, Json, rsj, get_env_or};

async fn get_crate_metadata(_req: Req, params: Params) -> Reply {
    // 1. URL Path Definition: Targets the specific crates.io API endpoint.
    let crate_name = params.0.get("name").cloned().unwrap_or_else(|| "crator".into());
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    
    // 2. Dynamic Identity: Use 'get_env_or' to set the User-Agent identity.
    // Fallback defaults to "crator-client/0.1.0" if no env variable is found.
    let user_agent = get_env_or("APP_UA", "crator-client/0.1.0").to_string();

    // 3. HTTP Request: Crator client with mandatory User-Agent and timeout.
    let response_result = Http::get(&url)
        .header("User-Agent", user_agent)
        .timeout(10)
        .send("");

    match response_result {
        Ok(res) if res.status() == 200 => {
            // 4. Parse JSON with crator::Json: Zero-dependency extraction.
            let raw_json = Json::from_str(res.body());
            
            let name = raw_json["crate"]["id"].as_str().unwrap_or("unknown");
            let downloads = raw_json["crate"]["downloads"].as_f64().unwrap_or(0.0);

            // 5. Build response with rsj! ('tabed' mode): Provides formatted JSON output.
            let output = rsj!(tabed, obj {
                crate_id: name,
                total_downloads: downloads,
                status: "success"
            });

            Reply::new(StatusCode::Ok)
                .header("Content-Type", "application/json; charset=UTF-8")
                .body(output.to_string())
        }
        Ok(res) => {
            Reply::new(StatusCode::NotFound)
                .header("Content-Type", "text/plain; charset=UTF-8")
                .body(format!("API Error: Status {}", res.status()))
        }
        Err(e) => {
            Reply::new(StatusCode::InternalServerError)
                .header("Content-Type", "text/plain; charset=UTF-8")
                .body(format!("❌ Connection Error: {}", e))
        }
    }
}

fn main() {
    let mut app = WebIo::new();
    
    // Route matching for dynamic crate metadata retrieval.
    // Access via: http://localhost:8080/api/info/webio
    app.route(GET, "/api/info/<name>", get_crate_metadata);

    // 6. Environment Configuration: Dynamic port retrieval with standard fallback.
    let host = "127.0.0.1";
    let port = get_env_or("PORT", "8080");

    app.run(&host, &port);
}
```      

<div><a href="https://crates.io/crates/crator" target="_blank">🔗 <b>View crator Documentation</b></a></div>

---