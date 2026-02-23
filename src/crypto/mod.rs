//! # WebIO Cryptographic Engine
//! 
//! This module provides internal, zero-dependency implementations of essential 
//! cryptographic and encoding primitives (SHA-1, Base64, and RFC 6455 Framing).
//! 
//! ### Architectural Rationale:
//! In accordance with WebIO’s **Zero-Dependency Philosophy**, these utilities 
//! are built strictly on the Rust Standard Library (`std`). This approach 
//! eliminates "Supply-Chain Attacks" and ensures the framework remains 
//! ultra-lightweight for **Computational Science** and **High-Stakes Environments**.
//! 
//! ### Core Capabilities:
//! * **SHA-1 (FIPS 180-1):** A deterministic hashing engine for WebSocket handshakes.
//! * **Base64 (RFC 4648):** A high-performance, pre-allocated encoder for binary-to-text 
//!   transformations with O(n) efficiency.
//! * **WSFrame (RFC 6455):** A robust WebSocket framing engine supporting 7-bit, 
//!   16-bit, and 64-bit "Big Data" payloads with XOR unmasking.

use std::net::TcpStream;
use std::io::Read;

// --- CRYPTOGRAPHIC PRIMITIVES ---

/// A specialized, zero-dependency implementation of the SHA-1 Hashing Algorithm (FIPS 180-1).
/// 
/// In accordance with WebIO's **Zero-Dependency Philosophy**, this internal 
/// implementation facilitates the cryptographic handshake required for RFC 6455 
/// WebSocket upgrades without the binary bloat or supply-chain risks of external 
/// crates. It ensures that the framework's core remains lightweight, portable, 
/// and fully auditable.
pub struct Sha1 {
    /// Internal 160-bit state represented as five 32-bit words.
    state: [u32; 5],
    /// Processing buffer for 512-bit message blocks.
    buffer: Vec<u8>,
    /// Total message bit length used for final padding.
    count: u64,
}

impl Sha1 {
    /// Initializes the SHA-1 state with the standard cryptographic constants.
    pub fn new() -> Self {
        Self {
            state: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0],
            buffer: Vec::new(),
            count: 0,
        }
    }

    /// Ingests a byte slice into the hashing engine, updating the bit count.
    pub fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        self.count += (data.len() * 8) as u64;
    }

    /// Finalizes the hashing process by applying FIPS-standard padding
    /// and processing remaining chunks.
    /// 
    /// Returns a deterministic 20-byte (160-bit) message digest.
    pub fn finalize(mut self) -> [u8; 20] {
        // Append bit '1' followed by '0's for block alignment
        self.buffer.push(0x80);
        while (self.buffer.len() % 64) != 56 { self.buffer.push(0); }
        self.buffer.extend_from_slice(&self.count.to_be_bytes());
        
        // Core transformation loop: process 512-bit (64-byte) blocks
        for chunk in self.buffer.chunks_exact(64) {
            let mut w = [0u32; 80];
            for i in 0..16 {
                w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]);
            }
            // Expand 16 words into 80 words
            for i in 16..80 {
                w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1);
            }
            let [mut a, mut b, mut c, mut d, mut e] = self.state;
            // 80 Rounds of hashing transformation
            for i in 0..80 {
                let (f, k) = match i {
                    0..=19 => ((b & c) | (!b & d), 0x5A827999),
                    20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                    40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                    _ => (b ^ c ^ d, 0xCA62C1D6),
                };
                let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
                e = d; d = c; c = b.rotate_left(30); b = a; a = temp;
            }
            // Accumulate the transformation back into the internal state
            self.state[0] = self.state[0].wrapping_add(a);
            self.state[1] = self.state[1].wrapping_add(b);
            self.state[2] = self.state[2].wrapping_add(c);
            self.state[3] = self.state[3].wrapping_add(d);
            self.state[4] = self.state[4].wrapping_add(e);
        }
        // Flatten state words into final byte array
        let mut out = [0u8; 20];
        for i in 0..5 { out[i*4..(i+1)*4].copy_from_slice(&self.state[i].to_be_bytes()); }
        out
    }
}

// --- DATA ENCODING UTILITIES ---

/// A specialized, zero-dependency implementation of Base64 Encoding (RFC 4648).
/// 
/// In accordance with the **Zero-Dependency Philosophy**, this internal utility 
/// facilitates the binary-to-text transformation required for the WebSocket 
/// cryptographic handshake. By avoiding external encoding crates, WebIO 
/// maintains a deterministic memory footprint and a minimal binary size, 
/// essential for high-integrity **Computational Science** environments.
/// 
/// ### Algorithm Detail:
/// The encoder processes data in 24-bit fragments (3 bytes), mapping them 
/// into four 6-bit indices against the standard 64-character alphabet. 
/// Standard padding characters (`=`) are appended to ensure the output 
/// string length is always a multiple of four.
pub fn base64_encode(input: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    // Memory Optimization: Pre-allocate the exact required capacity
    // to prevent redundant heap reallocations during the encoding process.
    let mut result = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        // Concatenate 8-bit bytes into a 32-bit register for bit-shifting.
        let b = match chunk.len() {
            3 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32),
            2 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8,
            _ => (chunk[0] as u32) << 16,
        };
        // Extract 6-bit indices and map to the CHARSET
        result.push(CHARSET[(b >> 18 & 0x3F) as usize] as char);
        result.push(CHARSET[(b >> 12 & 0x3F) as usize] as char);
        // Handle deterministic padding for incomplete 24-bit blocks
        if chunk.len() > 1 { result.push(CHARSET[(b >> 6 & 0x3F) as usize] as char); } else { result.push('='); }
        if chunk.len() > 2 { result.push(CHARSET[(b & 0x3F) as usize] as char); } else { result.push('='); }
    }
    result
}

// --- WEBSOCKET PROTOCOL ENGINE ---

/// Represents a discrete data packet within the WebSocket protocol.
/// 
/// In a multi-threaded **Computational Science** environment, `WSFrame` facilitates 
/// the real-time ingestion of binary or text data fragments. By isolating frame 
/// parsing into a dedicated structure, WebIO maintains a clean separation between 
/// standard HTTP request-response cycles and persistent, full-duplex streams.
pub struct WSFrame {
    /// The unmasked binary payload extracted from the WebSocket frame.
    pub payload: Vec<u8>,
}

impl WSFrame {
    /// Ingests and decodes a raw WebSocket frame directly from the network stream.
    /// 
    /// This method implements the core framing logic defined in **RFC 6455**. 
    /// It handles dynamic payload length detection and performs the cryptographic 
    /// XOR unmasking required for client-to-server security. 
    /// 
    /// ### Technical Implementation:
    /// * **Length Handling:** Supports small (7-bit), medium (16-bit), and 
    ///   "Big Data" (64-bit) payload lengths, enabling the transmission of 
    ///   massive datasets over a persistent socket.
    /// * **Bitwise Masking:** Complies with the mandatory masking requirement 
    ///   for client frames, ensuring the data is correctly de-obfuscated before 
    ///   being passed to the application layer.
    /// * **I/O Efficiency:** Utilizes `read_exact` to ensure frame integrity, 
    ///   preventing partial reads that could corrupt high-precision data streams.
    pub fn read(stream: &mut TcpStream) -> std::io::Result<Self> {
        let mut head = [0u8; 2];
        stream.read_exact(&mut head)?;

        // The second byte contains the Mask bit (MSB) and Payload Length (7 bits)
        let is_masked = (head[1] & 0x80) != 0;
        let mut len = (head[1] & 0x7F) as usize;

        // Dynamic Length Decoding: Handles extended 16-bit or 64-bit length fields
        if len == 126 {
            let mut extended_len = [0u8; 2];
            stream.read_exact(&mut extended_len)?;
            len = u16::from_be_bytes(extended_len) as usize;
        } else if len == 127 {
            let mut extended_len = [0u8; 8];
            stream.read_exact(&mut extended_len)?;
            len = u64::from_be_bytes(extended_len) as usize;
        }

        // Memory Allocation: Pre-allocates the payload buffer based on the decoded length
        let mut payload = vec![0u8; len];

        if is_masked {
            let mut mask = [0u8; 4];
            stream.read_exact(&mut mask)?;
            stream.read_exact(&mut payload)?;
            // Cryptographic XOR Unmasking (RFC 6455 Section 5.3)
            // Essential for maintaining protocol safety and data integrity.
            for i in 0..len {
                payload[i] ^= mask[i % 4];
            }
        } else {
            // Unmasked frames from clients are technically a protocol violation,
            // but the logic provides a fallback for non-compliant implementations.
            stream.read_exact(&mut payload)?;
        }

        Ok(WSFrame { payload })
    }
}