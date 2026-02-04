# WebIO 🦀

> **A minimalist async web framework for Rust with a zero-dependency philosophy.**

<div style="text-align: center;">
  <a href="https://github.com/dr-montasir/webio"><img src="https://img.shields.io/badge/github-dr%20montasir%20/%20webio-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24" style="margin-top: 10px;" alt="github" /></a> <a href="https://crates.io/crates/webio"><img src="https://img.shields.io/crates/v/webio.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24" style="margin-top: 10px;" alt="crates.io"></a> <a href="https://docs.rs/webio"><img src="https://img.shields.io/badge/docs.rs-webio-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24" style="margin-top: 10px;" alt="docs.rs"></a> <a href="https://choosealicense.com/licenses/mit"><img src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555" height="24" style="margin-top: 10px;" alt="license"></a>
</div>

---

## 🧪 Research Status: Experimental
**WebIO** is currently in a **research and development phase**. 
- **API Stability:** Expect significant breaking changes from version to version.
- **Goal:** To explore the boundaries of high-performance async web servers using **only the Rust `std` library**.
- **Production Warning:** Do not use this in a production environment until a stable `1.0.0` version is released.

## 🎯 Core Philosophy
The goal of this project is to provide a fully functional async web framework with **zero external dependencies**. 
- **No** `tokio` or `async-std`.
- **No** `serde` (where possible).
- **No** `hyper`.
- **No** `bloat`.
- **No** `unsafe` code.
- **Just pure Rust.**

## 🛠️ Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
webio = "0.4.1-alpha"
