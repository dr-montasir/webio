# 🧩 Modular Configuration

WebIO supports separating routing and middleware logic into dedicated setup functions to keep `main` clean.

## 🗺️ Modular Route Setup

Encapsulating routes in a `setup_routes` function allows for better organization, especially as the application grows in complexity.

```rust,no_run
use webio::*;

async fn home(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok).body("<h1>Home Page</h1>")
}

async fn about(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok).body("<h1>About Page</h1>")
}

async fn api_message(req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok).body(format!("Received: {}", req.body))
}

// Encapsulate route registration
fn setup_routes(app: &mut WebIo) {
    app.route(GET, "/", home);                // Standard GET route
    app.route(GET, "/about", about);          // Informational route
    app.route(POST, "/api/msg", api_message); // Data ingestion route
}

fn main() {
    let mut app = WebIo::new();
    setup_routes(&mut app);
    app.run("127.0.0.1", "8080");
}
```

## 🔐 Modular Middleware Setup

Middleware in WebIO acts as a **Pre-flight Guard**. By defining a `setup_middlewares` function, the engine can intercept requests for authentication, logging, or IP blacklisting before they reach the routing table.

**1. Inline Closures (Directly inside setup_middlewares)**

This approach is fine for quick logic when seeing all rules in one place is required.

```rust,no_run
use webio::*;

// --- Middleware Setup ---
fn setup_middlewares(app: &mut WebIo) {
    // 1. Logger
    app.use_mw(|path| {
        println!("[LOG]: Request for -> {}", path);
        None
    });

    // 2. Secret Key Middleware (Dynamic Check)
    app.use_mw(|path| {
        if path.contains("/secret/vault/") {
            // Logic: Only allow the request if the URL ends exactly with '123'
            if path.ends_with("/123") {
                println!("✅ Auth: Dynamic key 123 accepted.");
                return None; 
            } else {
                println!("❌ Auth: Unauthorized key attempted.");
                return Some(
                    Reply::new(StatusCode::Unauthorized)
                        .header("Content-Type", "text/html; charset=UTF-8")
                        .body("<h1>🚫 Access Denied: Invalid Key</h1>")
                );
            }
        }
        None
    });
}

// --- Dynamic Route Handler ---
async fn secret_handler(_req: Req, params: Params) -> Reply {
    // Extract the <key> from the URL
    let key = params.0.get("key").cloned().unwrap_or("unknown".to_string());
    
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(format!("<h1>🔐 Vault Accessed</h1><p>Key used: <b>{}</b></p>", key))
}

fn main() {
    let mut app = WebIo::new();
    
    setup_middlewares(&mut app);

    // 1. Public Admin Route
    app.route(GET, "/admin/dashboard", |_r, _p| async { 
        Reply::new(StatusCode::Ok).body("Admin Panel (Public)") 
    });

    // 2. Dynamic Secret Route using <key>
    // http://localhost:8080/secret/vault/123   ==>   🔐 Vault Accessed     Key used: 123
    // http://localhost:8080/secret/vault/132   ==>   🚫 Access Denied: Invalid Key
    app.route(GET, "/secret/vault/<key>", secret_handler);

    app.run("127.0.0.1", "8080");
}
```

**2. Separate Functions (Outside setup_middlewares)**

This approach is correct for complex logic when maintaining modularity and high readability across larger projects is required.

```rust,no_run
use webio::*;

// --- 1. Middleware Logic Functions ---

fn logger_mw(path: &str) -> Option<Reply> {
    println!("[LOG]: Request for -> {}", path);
    None
}

fn auth_mw(path: &str) -> Option<Reply> {
    if path.starts_with("/protected/") {
        if path.ends_with("/123") {
            println!("✅ Auth: Protected key 123 accepted.");
            return None; 
        } else {
            return Some(
                Reply::new(StatusCode::Unauthorized)
                    .header("Content-Type", "text/html; charset=UTF-8")
                    .body("<h1>🚫 Access Denied: Invalid Key</h1>")
            );
        }
    }
    None
}

// --- 2. Route Handler Functions ---

async fn admin_handler(_req: Req, _p: Params) -> Reply {
    Reply::new(StatusCode::Ok).body("<h1>Admin Panel</h1><p>Publicly visible.</p>")
}

async fn protected_handler(_req: Req, params: Params) -> Reply {
    let key = params.0.get("key").cloned().unwrap_or("unknown".to_string());
    Reply::new(StatusCode::Ok)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(format!("<h1>🛡️ Protected Area</h1><p>Key: <b>{}</b></p>", key))
}

// --- 3. Modular Setup Functions ---

fn setup_middlewares(app: &mut WebIo) {
    app.use_mw(logger_mw);
    app.use_mw(auth_mw);
}

fn setup_routes(app: &mut WebIo) {
    // http://localhost:8080/admin/dashboard ==>   Admin Panel  Publicly visible.
    app.route(GET, "/admin/dashboard", admin_handler);

    // http://localhost:8080/protected/123   ==>   🛡️ Protected Area     Key used: 123
    // http://localhost:8080/protected/132   ==>   🚫 Access Denied: Invalid Key
    app.route(GET, "/protected/<key>", protected_handler);
}

// --- 4. Main Entry Point ---

fn main() {
    let mut app = WebIo::new();
    
    // Configuration Setup
    // First, apply the middleware guards
    setup_middlewares(&mut app);

    // Then, register the routes
    setup_routes(&mut app);

    app.run("127.0.0.1", "8080");
}
```