# 5. 🔀 Dynamic Routing

Path variables are captured using the `<key>` syntax. The **WebIO** engine automatically parses multiple parameters into a thread-safe collection for immediate use.

- **URL Pattern:** `/to/<id>/path/<key>`
- **Data Extraction:** Values for `id` and `key` are isolated and accessible via the `Params` collection, facilitating complex API structures and nested resources.

```rust,no_run
use webio::*;

fn main() {
    let mut app = WebIo::new();

    // Define complex dynamic routes.
    // Example: http://localhost:8080/to/123/path/abc-789
    app.route(GET, "/to/<id>/path/<key>", |_req, params| async move {
        // Parameters are extracted from the zero-indexed collection
        let id = params.0.get("id").cloned().unwrap_or_default();
        let key = params.0.get("key").cloned().unwrap_or_default();
        
        Reply::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(format!("<h1>🔀 ID: {}, KEY: {}</h1>", id, key))
    });

    app.run("127.0.0.1", "8080");
}
```