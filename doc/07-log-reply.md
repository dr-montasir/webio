# 7. 📡 Log Reply

**WebIO** allows for the monitoring of outgoing responses by enabling **Reply Logging**. When active, the engine tracks status codes and headers sent back to the client, providing better visibility into the server's final output.

- **Optional Monitoring:** Response logging is disabled by default (`false`) to maintain high performance and clean terminal output.
- **Debugging Insights:** Enabling this feature is ideal for verifying that correct status codes (like 200 OK or 404 Not Found) are being dispatched to the end user.

```rust,no_run
use webio::*;

fn main() {
    let mut app = WebIo::new();

    // Log Reply:
    // Enable logging of outgoing responses (Replies).
    // When true, the server logs the status codes and headers sent back to the client.
    app.log_reply_enabled = true;

    // Static files and 404 handlers
    app.use_static("assets"); 
    // app.on_404(my_custom_html_404);
    // app.on_404(my_custom_json_404);

    // Routes
    // app.route(GET, "/", home_page);

    app.run("127.0.0.1", "8080");
}
```