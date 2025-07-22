//! # boykisser
//!
//! **boykisser** is a Rust library that provides compile-time embedded ASCII art and images of the iconic "boykisser" meme, along with a flexible macro system for including your own assets in any Rust project.
//!
//! ## What does it do?
//!
//! - **Instant access to boykisser art:** Use feature flags to include a curated collection of boykisser ASCII art and images, ready to embed in your application or bot.
//! - **Powerful asset macros:** Use the provided macros to embed your own images, ASCII art, or text files at compile time, with type-safe access and metadata.
//! - **Macro-only mode:** If you only want the macro system (for your own assets) and not the boykisser art, simply disable the `art` feature for a minimal dependency.
//!
//! ## Features
//!
//! - Zero runtime cost: all assets are embedded at compile time
//! - Type-safe, ergonomic asset access with compile-time validation
//! - Support for both binary (images, fonts) and text (ASCII art, configs) assets
//! - Easy-to-use macros for asset registration and access
//! - Metadata support for asset information
//! - Optional: Remove all built-in art/images for a pure macro utility
//!
//! ## Example Usage
//!
//! ```rust
//! use boykisser::*;
//!
//! // Access built-in boykisser ASCII art (if the `art` feature is enabled)
//! let ascii = art::boykisser::BOYKISSER;
//! println!("{}", ascii);
//!
//! // Access a built-in image
//! let ferris = images::FERRIS_PNG;
//!
//! // Use the macro system for your own assets
//! boykisser::include_text_asset!(pub MY_ASCII, "assets/my_art.txt", "My custom art");
//! println!("{}", MY_ASCII);
//! ```
//!
//! ## Disabling Art Features
//!
//! If you only want the macro system and not the built-in art/images, disable the `art` feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! boykisser = { version = "...", default-features = false }
//! ```

// Be a perfectionist, no code is good enough!
#![deny(
    clippy::all,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
// Unwraps are a bad practice and do not provide useful error messages/handling.
#![warn(clippy::unwrap_used)]

use std::fmt;

#[cfg(feature = "art")]
pub mod art;

/// Metadata for binary assets
#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub name: &'static str,
    pub size: usize,
    pub format: &'static str,
    pub description: &'static str,
}

/// Metadata for text assets
#[derive(Debug, Clone)]
pub struct TextAssetInfo {
    pub name: &'static str,
    pub description: &'static str,
}

impl fmt::Display for AssetInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} bytes ({})", self.name, self.size, self.format)
    }
}

impl fmt::Display for TextAssetInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

/// Macro to include binary assets with metadata
///
/// # Usage
/// ```ignore
/// use boykisser::include_binary_asset;
///
/// include_binary_asset!(
///     pub ICON_PNG,           // Asset name
///     "assets/icon.png",      // File path
///     "PNG",                  // Format
///     "Application icon"      // Description
/// );
/// ```
#[macro_export]
macro_rules! include_binary_asset {
    ($vis:vis $name:ident, $path:expr, $format:expr, $desc:expr) => {
        $vis static $name: &'static [u8] = include_bytes!($path);

        paste::paste! {
            $vis static [<$name _INFO>]: $crate::AssetInfo = $crate::AssetInfo {
                name: stringify!($name),
                size: $name.len(),
                format: $format,
                description: $desc,
            };
        }
    };
}

/// Macro to include text assets with metadata
///
/// # Usage
/// ```ignore
/// use boykisser::include_text_asset;
///
/// include_text_asset!(
///     pub RUST_LOGO,          // Asset name
///     "assets/rust_logo.txt", // File path
///     "Rust programming language logo"  // Description
/// );
/// ```
#[macro_export]
macro_rules! include_text_asset {
    ($vis:vis $name:ident, $path:expr, $desc:expr) => {
        #[doc = $desc]
        $vis static $name: &str = include_str!($path);

        paste::paste! {
            #[doc = concat!("Asset information for `", stringify!($name), "`: ", $desc)]
            $vis static [<$name _INFO>]: $crate::TextAssetInfo = $crate::TextAssetInfo {
                name: stringify!($name),
                description: $desc,
            };
        }
    };
}

/// Macro to create an asset registry with multiple assets
///
/// # Usage
/// ```ignore
/// use boykisser::create_asset_registry;
///
/// create_asset_registry!(
///     MyAssets,
///     [
///         (LOGO_PNG, "assets/logo.png", "PNG", "Company logo"),
///         (ICON_SVG, "assets/icon.svg", "SVG", "Application icon"),
///     ],
///     [
///         (BANNER_ASCII, "assets/banner.txt", "ASCII art banner"),
///         (CONFIG_TEMPLATE, "assets/config.toml", "Default configuration"),
///     ]
/// );
/// ```
#[macro_export]
macro_rules! create_asset_registry {
    (
        $registry_name:ident,
        [ $( ($bin_name:ident, $bin_path:expr, $bin_format:expr, $bin_desc:expr) ),* $(,)? ],
        [ $( ($text_name:ident, $text_path:expr, $text_desc:expr) ),* $(,)? ]
    ) => {
        pub mod $registry_name {
            // Binary assets
            $(
                $crate::include_binary_asset!(pub $bin_name, $bin_path, $bin_format, $bin_desc);
            )*

            // Text assets
            $(
                $crate::include_text_asset!(pub $text_name, $text_path, $text_desc);
            )*

            /// List all binary assets in this registry
            #[must_use] pub const fn list_binary_assets() -> Vec<&'static $crate::AssetInfo> {
                vec![
                    $(
                        paste::paste! { &[<$bin_name _INFO>] },
                    )*
                ]
            }

            /// List all text assets in this registry
            #[must_use] pub const fn list_text_assets() -> Vec<&'static $crate::TextAssetInfo> {
                vec![
                    $(
                        paste::paste! { &[<$text_name _INFO>] },
                    )*
                ]
            }
        }
    };
}

/// Trait for asset retrieval by name
pub trait AssetRetriever {
    fn get_binary_asset(&self, name: &str) -> Option<&'static [u8]>;
    fn get_text_asset(&self, name: &str) -> Option<&'static str>;
    fn list_assets(&self) -> Vec<String>;
}

pub mod images {
    use super::AssetInfo;

    // Example binary assets (in practice, these would use include_bytes!)

    /// Placeholder for a PNG icon (normally would be `include_bytes!("assets/icon.png`"))
    pub static ICON_PNG: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A,
        0x0A, // PNG signature
             // ... rest of PNG data would be here
    ];

    pub static FERRIS_PNG: &[u8] = include_bytes!("../assets/ferris.png");

    /// Icon metadata
    pub static ICON_PNG_INFO: AssetInfo = AssetInfo {
        name: "ICON_PNG",
        size: ICON_PNG.len(),
        format: "PNG",
        description: "Application icon in PNG format",
    };

    /// Placeholder for SVG logo
    pub static LOGO_SVG: &[u8] = b"<svg><!-- SVG content --></svg>";

    pub static LOGO_SVG_INFO: AssetInfo = AssetInfo {
        name: "LOGO_SVG",
        size: LOGO_SVG.len(),
        format: "SVG",
        description: "Company logo in SVG format",
    };
}

// Example of using the registry macro
create_asset_registry!(
    default_assets,
    [
        // Binary assets would reference real files:
        // (FAVICON, "assets/favicon.ico", "ICO", "Website favicon"),
        // (LOGO, "assets/logo.png", "PNG", "Application logo"),
    ],
    [
        // Text assets would reference real files:
        // (README, "assets/README.md", "Application readme file"),
        // (LICENSE, "assets/LICENSE", "Software license"),
    ]
);

/// Utility functions for working with assets
pub mod utils {

    /// Convert binary asset to base64 string (useful for embedding in HTML/CSS)
    #[must_use]
    pub fn to_base64(data: &[u8]) -> String {
        base64_encode(data)
    }

    /// Simple base64 encoding (for demonstration - use a proper base64 crate in production)
    fn base64_encode(input: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();

        for chunk in input.chunks(3) {
            let mut buf = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = byte;
            }

            let b = (u32::from(buf[0]) << 16) | (u32::from(buf[1]) << 8) | u32::from(buf[2]);

            result.push(CHARS[((b >> 18) & 63) as usize] as char);
            result.push(CHARS[((b >> 12) & 63) as usize] as char);
            result.push(if chunk.len() > 1 {
                CHARS[((b >> 6) & 63) as usize] as char
            } else {
                '='
            });
            result.push(if chunk.len() > 2 {
                CHARS[(b & 63) as usize] as char
            } else {
                '='
            });
        }

        result
    }

    /// Get MIME type from file extension
    #[must_use]
    pub fn mime_type_from_extension(ext: &str) -> &'static str {
        match ext.to_lowercase().as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "ico" => "image/x-icon",
            "txt" => "text/plain",
            "html" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "json" => "application/json",
            _ => "application/octet-stream",
        }
    }

    /// Create a data URL from binary asset
    #[must_use]
    pub fn create_data_url(data: &[u8], mime_type: &str) -> String {
        format!("data:{};base64,{}", mime_type, to_base64(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_art_access() {
        assert!(!art::rust::RUST_LOGO.is_empty());
        assert!(!art::rust::RUST_BANNER.is_empty());
        assert_eq!(art::rust::RUST_LOGO_INFO.name, "RUST_LOGO");
    }

    #[test]
    fn test_binary_asset_access() {
        assert!(!images::ICON_PNG.is_empty());
        assert_eq!(images::ICON_PNG_INFO.format, "PNG");
        assert_eq!(images::ICON_PNG_INFO.size, images::ICON_PNG.len());
    }

    #[test]
    fn test_base64_encoding() {
        let data = b"Hello, World!";
        let encoded = utils::to_base64(data);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_mime_type_detection() {
        assert_eq!(utils::mime_type_from_extension("png"), "image/png");
        assert_eq!(utils::mime_type_from_extension("txt"), "text/plain");
        assert_eq!(
            utils::mime_type_from_extension("unknown"),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_data_url_creation() {
        let data = b"test";
        let url = utils::create_data_url(data, "text/plain");
        assert!(url.starts_with("data:text/plain;base64,"));
    }
}
