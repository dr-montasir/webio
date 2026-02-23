//! # WebIO Utility & Protocol Engine
//! 
//! This module provides a collection of deterministic helpers for filesystem 
//! discovery, WebSocket protocol encoding, and cross-thread communication.
//! 
//! ### Key Utility Capabilities:
//! * **Recursive Asset Discovery:** Performs depth-first searches to locate 
//!   hidden assets (like `favicon.ico`) in complex directory trees.
//! * **RFC 6455 Handshake:** Implements the official WebSocket "Magic String" 
//!   logic using the internal `crypto` module.
//! * **Dynamic Frame Encoding:** Constructs WebSocket binary frames with 
//!   support for 7-bit, 16-bit, and 64-bit "Big Data" lengths.
//! * **Global Broadcast Hub:** Utilizes `LazyLock` and `Mutex` to manage 
//!   a thread-safe registry of active WebSocket participants for real-time sync.

use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Mutex, LazyLock};
use std::net::TcpStream;
use std::io::Write;
use crate::crypto::{Sha1, base64_encode};

// --- SYSTEM UTILITIES & BROADCAST ENGINE ---

/// Performs a depth-first recursive search for a specific filename within a directory tree.
/// 
/// This utility enables dynamic asset discovery, such as locating a `favicon.ico` 
/// hidden within deep subdirectories. By traversing the filesystem recursively, 
/// WebIO provides a flexible "Smart Static" delivery system that reduces 
/// manual route configuration for complex frontend directory structures.
pub fn find_file_recursive(dir: &Path, filename: &str) -> Option<PathBuf> {
    if !dir.is_dir() { return None; }
    
    // Check current scope: Priority 1
    let current_check = dir.join(filename);
    if current_check.exists() { return Some(current_check); }

    // Recursive Descent: Scan subdirectories
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_file_recursive(&path, filename) {
                    return Some(found);
                }
            }
        }
    }
    None
}

/// Generates the cryptographic `Sec-WebSocket-Accept` signature (RFC 6455).
/// 
/// This internal helper combines the client-provided key with the WebSocket 
/// "Magic String" (GUID) and performs a SHA-1 hash followed by Base64 encoding. 
/// This deterministic handshake is the security foundation of WebIO's 
/// zero-dependency WebSocket implementation.
pub fn derive_websocket_accept(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    // The "Magic String" defined by the official IETF specification
    hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    base64_encode(&hasher.finalize())
}

/// Serializes a raw text message into a compliant WebSocket Binary Frame.
/// 
/// Following the **RFC 6455 Section 5.2** protocol, this function constructs 
/// unmasked server-to-client frames. It intelligently scales the frame header 
/// based on payload size, supporting everything from short status updates 
/// (7-bit length) to massive Big Data broadcasts (64-bit length).
pub fn encode_ws_frame(message: &str) -> Vec<u8> {
    let payload = message.as_bytes();
    let len = payload.len();
    let mut frame = Vec::new();

    // Opcode: 0x81 (Final Fragment + Text Frame)
    frame.push(0x81);

    // Dynamic Payload Length Encoding
    if len <= 125 {
        frame.push(len as u8);
    } else if len <= 65535 {
        frame.push(126);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        // Support for multi-gigabyte real-time data streaming
        frame.push(127);
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }

    // Payload
    frame.extend_from_slice(payload);
    frame
}

/// A globally accessible, thread-safe registry of active WebSocket participants.
/// 
/// Utilizes [`LazyLock`] (Rust 1.80+) for zero-cost initialization and a 
/// [`Mutex`] to coordinate cross-thread access. This allows any OS thread 
/// within the WebIO engine to interact with the persistent socket pool.
pub static CLIENTS: LazyLock<Mutex<Vec<TcpStream>>> = LazyLock::new(|| {
    Mutex::new(Vec::new())
});

/// Disseminates a message to all active WebSocket clients across the entire engine.
/// 
/// `broadcast` is a high-level orchestration method that facilitates real-time 
/// synchronization between isolated OS threads. It automatically performs 
/// **Ghost Connection Cleanup**: if a write fails (e.g., client disconnected), 
/// the socket is atomically removed from the registry, ensuring RAM stability.
pub fn broadcast(message: &str) {
    if let Ok(mut clients) = CLIENTS.lock() {
        let frame = encode_ws_frame(message);
        clients.retain_mut(|client| {
            client.write_all(&frame).is_ok()
        });
    }
}