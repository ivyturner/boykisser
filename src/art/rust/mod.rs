use crate::{TextAssetInfo, include_binary_asset};

/// Rust logo ASCII art
#[cfg(feature = "rust")]
pub static RUST_LOGO: &str = r"
    ██████╗ ██╗   ██╗███████╗████████╗
    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝
    ██████╔╝██║   ██║███████╗   ██║   
    ██╔══██╗██║   ██║╚════██║   ██║   
    ██║  ██║╚██████╔╝███████║   ██║   
    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   
    ";

/// Rust logo metadata
#[cfg(feature = "rust")]
pub static RUST_LOGO_INFO: TextAssetInfo = TextAssetInfo {
    name: "RUST_LOGO",
    description: "Rust programming language logo in ASCII art",
};

/// Simple banner
#[cfg(feature = "rust")]
pub static RUST_BANNER: &str = r"
    ╔══════════════════════════════════════╗
    ║            Welcome to Rust!          ║
    ║        Fast • Safe • Productive      ║
    ╚══════════════════════════════════════╝
    ";

/// Banner metadata
#[cfg(feature = "rust")]
pub static RUST_BANNER_INFO: TextAssetInfo = TextAssetInfo {
    name: "RUST_BANNER",
    description: "Welcome banner for Rust applications",
};

include_binary_asset!(
    pub FERRIS_PNG,
    "../../../assets/ferris.png",
    "PNG",
    "Ferris the crab in PNG format"
);
