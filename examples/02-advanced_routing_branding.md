# Advanced Dynamic Routing & Engine Branding

This section explores **WebIO's** ability to handle complex nested resources and the flexibility to personalize the engine's startup interface.

## Advanced Dynamic Routing

Path variables are captured using the `<key>` syntax. The **WebIO** engine automatically parses multiple parameters into a thread-safe collection for immediate use.

- **URL Pattern:** `/to/<id>/path/<key>`
- **Data Extraction:** Values for `id` and `key` are isolated and accessible within the route handler, facilitating complex API structures and nested resources.

## Customizable Startup Banner

**WebIO** includes a professional default banner while offering full override capabilities. Assigning a value to the `banner_text` field before calling `.run()` enables:

- **Branding & Emojis:** Integration of custom icons and framework names.
- **Environment Labeling:** Distinct identification for different modes (e.g., "Production Engine" vs "Dev Mode").
- **Automatic Fallback:** If the field remains empty, the engine defaults to the signature `🦅 WebIO Live:` prefix to maintain a clean terminal interface.

## Implementation Example

```rust,no_run
use webio::*;

fn main() {
    let mut app = WebIo::new();

    // 1. Define complex dynamic routes.
    // Access via: http://localhost:8080/to/123/path/4567
    app.route(GET, "/to/<id>/path/<key>", |_req, params| async move {
        let id = params.0.get("id").cloned().unwrap_or_else(|| "0".into());
        let key = params.0.get("key").cloned().unwrap_or_else(|| "0".into());
        
        Reply::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(format!("<h1>ID: {}, KEY: {}</h1>", id, key))
    });

    // 2. Personalize Engine Branding.
    // Terminal Output: 🚀 My Custom Multi-Threaded Engine: http://127.0.0.1:8080
    app.banner_text = "🚀 My Custom Multi-Threaded Engine: ".to_string();

    // 3. Launch the Multi-Threaded Engine.
    app.run("127.0.0.1", "8080");
}
```