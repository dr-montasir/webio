# WebIO Executor: Manual vs. Integrated Async

This example demonstrates how to launch **WebIO** from a standard `fn main()`. It highlights two ways to define routes: using a manual `block_on` wrapper and using the framework's integrated async handler.

## Implementation Example

```rust,no_run
use webio::*;
use webio_macros::html;

fn main() {
    let mut app = WebIo::new();
    app.log_reply_enabled = true;

    // --- Method 1: Manual block_on ---
    // Useful for grouping multiple route definitions or pre-launch logic.
    block_on(async {
        app.route(GET, "/block-on", |_req, _params| async move {
            let content = html!("<h1>Hello from 🦅 {{name}} (Block On)!</h1>", name = "WebIO");
            Reply::new(StatusCode::Ok)
                .header("Content-Type", "text/html; charset=UTF-8")
                .body(content)
        });
    });

    // --- Method 2: Integrated Async Route ---
    // The standard, cleaner way to define routes directly in WebIO.
    app.route(GET, "/no-block-on", |_req, _params| async {
        let content = html!("<h1>Hello from 🦅 {{name}} (No Block On)!</h1>", name = "WebIO");
        Reply::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(content)
    });

    // Launch the engine on all interfaces
    app.set_nagle(false).run("0.0.0.0", "8080");
}
```

## Technical Insight

- **block_on:** Explicitly drives the future to completion. In this example, it is used to register the route before the server starts.
- **app.route:** Inherently accepts an async closure. Even without a surrounding **`block_on`**, the WebIO engine will execute these futures when a request is received.