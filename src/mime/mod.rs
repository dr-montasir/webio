//! # MIME Module
//! 
//! This module provides a centralized registry for MIME types (Media Types)
//! and the standalone logic required to manage them within the application.
//! It includes a comprehensive list of default mappings and helper functions
//! to modify those mappings at runtime.

use std::collections::HashMap;

/// Returns a comprehensive, pre-configured collection of common MIME types.
/// 
/// This collection covers over 100 extensions across categories like web,
/// images, video, audio, office documents, and programming formats.
/// 
/// # Returns
/// A `HashMap<String, String>` where keys are file extensions (without dots)
/// and values are their corresponding MIME type strings.
pub fn default_mime_types() -> HashMap<String, String> {
    [
        // --- Web & Text ---
        ("html", "text/html"),
        ("ejs", "text/html"),
        ("xhtml", "application/xhtml+xml"),
        ("css", "text/css"),
        ("scss", "text/scss"),
        ("less", "text/css"),
        ("js", "text/javascript"),
        ("ts", "application/typescript"),
        ("json", "application/json"),
        ("jsonld", "application/ld+json"),
        ("map", "application/json-patch+json"),
        ("geojson", "application/geo+json"),
        ("graphql", "application/graphql"),
        ("xml", "application/xml"),
        ("rss", "application/rss+xml"),
        ("atom", "application/atom+xml"),
        ("yaml", "text/yaml"),
        ("toml", "application/toml"),
        ("csv", "text/csv"),

        // --- Documents & Plain Text ---
        ("txt", "text/plain"),
        ("rs", "text/plain"),
        ("plain", "text/plain"),
        ("log", "text/plain"),
        ("conf", "text/plain"),
        ("init", "text/plain"),
        ("md", "text/markdown"),
        ("markdown", "text/markdown"),
        ("pdf", "application/pdf"),
        ("rtf", "application/rtf"),
        ("srt", "application/x-subrip"),
        ("ics", "text/calendar"),
        ("vcard", "text/vcard"),

        // --- Images ---
        ("png", "image/png"),
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("gif", "image/gif"),
        ("webp", "image/webp"),
        ("svg", "image/svg+xml"),
        ("svgz", "image/svg+xml"),
        ("ico", "image/x-icon"),
        ("bmp", "image/bmp"),
        ("tiff", "image/tiff"),

        // --- Audio ---
        ("mp3", "audio/mpeg"),
        ("wav", "audio/wav"),
        ("aac", "audio/aac"),
        ("ogg", "audio/ogg"),
        ("flac", "audio/flac"),
        ("m4a", "audio/mp4a-latm"),
        ("mid", "audio/midi"),
        ("midi", "audio/midi"),

        // --- Video ---
        ("mp4", "video/mp4"),
        ("webm", "video/webm"),
        ("mov", "video/quicktime"),
        ("mkv", "video/x-matroska"),
        ("avi", "video/x-msvideo"),
        ("mpg", "video/mpeg"),
        ("mpeg", "video/mpeg"),
        ("flv", "video/x-flv"),

        // --- Fonts ---
        ("woff", "font/woff"),
        ("woff2", "font/woff2"),
        ("ttf", "font/ttf"),
        ("otf", "font/otf"),
        ("eot", "application/vnd.ms-fontobject"),

        // --- Programming & Binaries ---
        ("wasm", "application/wasm"),
        ("wat", "application/wasm"), // Wasm preferred over text/wat for execution
        ("py", "text/x-python"),
        ("php", "application/x-httpd-php"),
        ("sh", "application/x-sh"),
        ("bash", "application/x-shellscript"),
        ("bat", "application/bat"),
        ("sql", "application/sql"),
        ("bin", "application/octet-stream"),
        ("exe", "application/octet-stream"),
        ("dll", "application/octet-stream"),
        ("so", "application/x-sharedlib"),
        ("class", "application/java-vm"),
        ("jar", "application/java-archive"),

        // --- Archives & Packages ---
        ("zip", "application/zip"),
        ("gz", "application/gzip"),
        ("tar", "application/x-tar"),
        ("deb", "application/vnd.debian.binary-package"),
        ("apk", "application/vnd.android.package-archive"),
        ("epub", "application/epub+zip"),

        // --- MS Office ---
        ("doc", "application/msword"),
        ("dot", "application/msword"),
        ("docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        ("dotx", "application/vnd.openxmlformats-officedocument.wordtemplate"),
        ("xls", "application/vnd.ms-excel"),
        ("xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        ("xltx", "application/vnd.openxmlformats-officedocument.spreadsheetml.template"),
        ("ppt", "application/vnd.ms-powerpoint"),
        ("pptx", "application/vnd.openxmlformats-officedocument.presentationml.presentation"),
        ("msg", "application/vnd.ms-outlook"),

        // --- Others ---
        ("kml", "application/vnd.google-earth.kml+xml"),
        ("kmz", "application/vnd.google-earth.kmz"),
        ("iot", "application/x-iot-data"),
        ("pem", "application/x-x509-ca-cert"),
        ("p12", "application/x-pkcs12"),
    ]
    .iter()
    .map(|&(ext, mime)| (ext.to_string(), mime.to_string()))
    .collect()
}

// --- MIME Management Logic ---

/// Inserts or updates a single MIME type in the provided map.
/// 
/// # Arguments
/// * `map` - A mutable reference to the MIME type `[HashMap](std::collections::HashMap)`.
/// * `ext` - The file extension string.
/// * `mime` - The corresponding MIME media type string.
pub fn set_mime_logic(map: &mut HashMap<String, String>, ext: &str, mime: &str) {
    map.insert(ext.to_string(), mime.to_string());
}

/// Inserts multiple MIME types into the provided map at once.
/// 
/// # Arguments
/// * `map` - A mutable reference to the MIME type `[HashMap](std::collections::HashMap)`.
/// * `types` - A vector of tuples containing `(extension, mime_type)`.
pub fn set_mimes_logic(map: &mut HashMap<String, String>, types: Vec<(&str, &str)>) {
    for (ext, mime) in types {
        set_mime_logic(map, ext, mime);
    }
}

/// Removes a MIME type mapping from the provided map.
/// 
/// # Arguments
/// * `map` - A mutable reference to the MIME type `[HashMap](std::collections::HashMap)`.
/// * `ext` - The file extension to remove.
pub fn remove_mime_logic(map: &mut HashMap<String, String>, ext: &str) {
    map.remove(ext);
}

/// Removes multiple MIME type mappings from the provided map.
/// 
/// # Arguments
/// * `map` - A mutable reference to the MIME type `[HashMap](std::collections::HashMap)`.
/// * `exts` - A vector of extensions to be removed.
pub fn remove_mimes_logic(map: &mut HashMap<String, String>, exts: Vec<&str>) {
    for ext in exts {
        remove_mime_logic(map, ext);
    }
}