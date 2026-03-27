# 11. Deployment

Deploying a **WebIO** application with the **Crator** toolkit involves specific network configurations and environment management. Below is the breakdown of deployment methods and technical requirements.

## 11-1. Deployment Methods

- **PaaS (Platform as a Service):** These services often manage the underlying infrastructure automatically. They typically assign a dynamic port via environment variables at runtime.
- **VPS (Virtual Private Server):** For these environments, **Docker** is the recommended standard to ensure consistency between development and production.
- **Serverless / Cloud Functions:** Some specialized providers allow Rust binaries to run as ephemeral functions within serverless or cloud function environments.

## 11-2. Host and Port Configuration

The transition from development to production requires modifying the listener address to allow external traffic.

1. **Address Binding:** The host must change from **`127.0.0.1 (localhost)`** to **`0.0.0.0`**. This tells the application to listen on all available network interfaces.
2. **Port Selection:** The **`port`** must align with the provider’s specific requirements.
    - **Fixed Ports:** Some setups allow a static port (e.g., **`8080`** or **`80`**).
    - **Dynamic Ports:** Many free tiers or managed services assign a unique port automatically.
3. **Implementation:** Use **`crator::get_env_or`** to bridge the gap between static code and dynamic provider requirements.

```rust,no_run
use webio::*;
// use crator::get_env_or;

fn main() {
    let mut app = WebIo::new();

    // ... define routes ...

    app.run("0.0.0.0", "8080"); // Change to the required port

    // or:

    // 1. Host: Change from "127.0.0.1" to "0.0.0.0" for production.
    // let host = "0.0.0.0"; 
    
    // 2. Port: Dynamically pull from the environment (required by most PaaS)
    // or fallback to a default (e.g., "10000").
    // let port = get_env_or("PORT", "10000"); 

    // 3. Execution: Pass references to the host and port.
    // app.run(&host, &port);
}
```

## 11-3. Environment Variables (.env)

If the project relies on external configurations (API keys, database URLs, or the `APP_UA` identity mentioned in the **Crator** logic), environment variables must be handled according to the deployment type:

- **Local / Docker:** Keep the **`.env`** file in the root directory as shown in the [**10-3. Crator**](https://docs.rs/webio/latest/webio/index.html#10-3-crator) **`directory structure`**.
- **Managed Hosts:** Most providers require entering these variables directly into their internal "Environment" or "Secrets" dashboard rather than uploading a physical file.
- **Security:** Ensure the **`.env`** file is included in **`.gitignore`** to prevent leaking sensitive credentials during the deployment process.

---