# MIME Type Configuration

This example demonstrates how to customize the **WebIO** internal registry to handle specific file extensions. It covers adding new media types for modern formats and enhancing security by removing unwanted file associations.

## Implementation Example

```rust,no_run
use webio::*;

fn main() {
    let mut app = WebIo::new();

    // 1. Add support for modern image formats
    app.set_mime("webp", "image/webp");

    // 2. Bulk update for web assets and fonts
    app.set_mimes(vec![
        ("woff2", "font/woff2"), 
        ("wasm", "application/wasm")
    ]);

    // 3. Security: Disable serving of specific script files
    app.remove_mime("php");

    // 4. Disable multiple video formats at once
    let to_remove = vec!["mp4", "webm", "avi", "mov"];
    app.remove_mimes(to_remove);

    app.run("127.0.0.1", "8080");
}
```

## Technical Insight

- **`set_mime` / `set_mimes`**: Used to define or update how the server identifies file types. This is essential for modern web standards like **WebP** or **WebAssembly**.
- **`remove_mime` / `remove_mimes`**: Used to strip extensions from the registry. If a file extension is removed, the engine will no longer serve it with a specific `Content-Type` header, defaulting to a safe binary fallback.
- **Case Insensitivity**: The engine automatically handles extension lookups in a case-insensitive manner (e.g., `.JPG` and `.jpg` are treated identically).

For more detailed implementation logic, check the [**`mime`**](https://docs.rs/crate/webio/latest/source/src/mime/mod.rs) and [**`engine`**](https://docs.rs/crate/webio/latest/source/src/engine/mod.rs) modules.