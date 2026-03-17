# 8. 🏳️ Custom Startup Banner

**WebIO** includes a professional default banner while offering full override capabilities. Assigning a value to the `banner_text` field before calling `.run()` enables:

- **Branding & Emojis:** Integration of custom icons and framework names.
- **Environment Labeling:** Distinct identification for different modes (e.g., "Production Engine" vs "Dev Mode").
- **Automatic Fallback:** If the field remains empty, the engine defaults to the signature `🦅 WebIO Live:` prefix to maintain a clean terminal interface.

```rust,no_run
use webio::*;

fn main () {
    let mut app = WebIo::new();

    app.route(GET, "/", |_req, _params| async {
        Reply::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=UTF-8")
            .body("<h1>My Custom Multi-Threaded Engine</h1>")
    });

    // 1. Personalize Engine Branding.
    // Terminal Output: 🚀 My Custom Multi-Threaded Engine: http://127.0.0.1:8080
    app.banner_text = "🚀 My Custom Multi-Threaded Engine:".to_string();

    // 2. Launch the Multi-Threaded Engine.
    app.run("127.0.0.1", "8080");
}
```