# 7. 🚨 Custom 404 Handlers

**WebIO** allows overriding the default `"Not Found"` response. By using the `on_404` method, custom logic can be registered to handle missing routes differently based on the client type.

## 7-1. 🛠️ Content-Aware Errors

The engine automatically selects the appropriate handler by checking the `Accept` header of the incoming request. This ensures browsers receive **HTML** while API tools receive **JSON**.

```rust,no_run
use webio::*;

// --- 1. Error Templates ---

const CUSTOM_404_HTML: &str = r#"
    <div style='text-align:center; font-family:sans-serif;'>
        <h1 style='color:red;'>🛸 404 - Not Found</h1>
        <p>That page doesn't exist on WebIo!</p>
    </div>
"#;

const CUSTOM_404_JSON: &str = r#"{"error": "not_found", "code": 404, "source": "WebIo API"}"#;

// --- 2. Handler Functions ---

/// Selected by WebIO when a browser (Accept: text/html) hits a missing route.
async fn custom_html_404(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::NotFound)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(CUSTOM_404_HTML)
}

/// Selected for API clients or tools like `curl`.
async fn custom_json_404(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::NotFound)
        .header("Content-Type", "application/json")
        .body(CUSTOM_404_JSON)
}

// --- 3. Configuration ---

fn setup_404_error(app: &mut WebIo) {
    app.on_404(custom_html_404);
    app.on_404(custom_json_404);
}

fn main() {
    let mut app = WebIo::new();
    
    // --- Application Configuration ---
    
    // 1. Pre-flight Guards
    // setup_middlewares(&mut app);

    // 2. Routing Table
    // setup_routes(&mut app);

    // 3. Error Handling
    setup_404_error(&mut app);

    app.run("127.0.0.1", "8080");
}
```