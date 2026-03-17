#![doc(html_logo_url = "https://github.com/dr-montasir/webio/raw/HEAD/webio-logo/logo/webio-logo-288x224-no-text-no-bg.svg")]

#![doc = include_str!("../doc/00-head.md")]
#![doc = include_str!("../doc/01-overview.md")]
#![doc = include_str!("../doc/02-philosophy.md")]
#![doc = include_str!("../doc/03-installation.md")]
#![doc = include_str!("../doc/04-quick-start.md")]
#![doc = include_str!("../doc/05-dynamic-routing.md")]
#![doc = include_str!("../doc/06-modular-configuration.md")]
#![doc = include_str!("../doc/07-custom-404-handlers.md")]
#![doc = include_str!("../doc/08-custom-startup-banner.md")]

// Modules
pub mod core;
pub mod crypto;
pub mod engine;
pub mod mime;
pub mod utils;

// Re-exports for a clean public API
pub use crate::core::*;
pub use crate::crypto::*;
pub use crate::engine::*;
pub use crate::mime::*;
pub use crate::utils::*;