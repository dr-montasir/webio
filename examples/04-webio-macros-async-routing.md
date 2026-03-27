# WebIO Macros & Async Routing

This example demonstrates the modern, streamlined way to build **WebIO** applications using the `webio_macros` crate for cleaner entry points and HTML templating.

## Implementation Example

```rust,no_run
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