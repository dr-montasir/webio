# 4. 🚀 Quick Start

WebIO provides a highly flexible routing system. These examples demonstrate the **Closure Pattern** for rapid development and the **Handler Pattern** for structured, modular applications.

## 4-1. 🌐 Basic Server

1. **Closure Pattern**

The closure pattern allows for defining logic directly within the route registration. This approach is ideal for maintaining full engine control with minimal boilerplate in smaller projects.

```rust,no_run
use webio::*;

fn main() {
    let mut app = WebIo::new();

    app.route(GET, "/", |_, _| async {
        Reply::new(StatusCode::Ok)
            .header("Content-Type", "text/plain; charset=UTF-8")
            .body("Hello from 🦅 WebIO!")
    });

    app.run("127.0.0.1", "8080");
}
```

2. **Handler Pattern**

The handler pattern isolates logic into dedicated `async` functions. This is the recommended approach for production-grade applications to ensure the codebase remains clean and testable.

```rust,no_run
use webio::*;

// Logic isolated into a dedicated handler function
async fn hello_handler(_req: Req, _params: Params) -> Reply {
    Reply::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=UTF-8")
            .body("<h1>Hello from 🦅 WebIO!</h1>")
}

fn main() {
    let mut app = WebIo::new();
    
    // Register the handler by name
    app.route(GET, "/", hello_handler);
    
    app.run("127.0.0.1", "8080");
}
```