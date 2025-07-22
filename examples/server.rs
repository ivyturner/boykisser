// Example integration showing how to use this in a web server or application
use boykisser::{
    art::{
        get_char_count, get_line_count,
        rust::{RUST_BANNER, RUST_BANNER_INFO, RUST_LOGO, RUST_LOGO_INFO},
    },
    images::{ICON_PNG, ICON_PNG_INFO, LOGO_SVG, LOGO_SVG_INFO},
};

/// Example function showing how to serve assets in a web application
#[must_use]
pub fn serve_asset(path: &str) -> Option<(Vec<u8>, &'static str)> {
    match path {
        "/icon.png" => Some((ICON_PNG.to_vec(), "image/png")),
        "/logo.svg" => Some((LOGO_SVG.to_vec(), "image/svg+xml")),
        "/banner.txt" => Some((RUST_BANNER.as_bytes().to_vec(), "text/plain")),
        "/rust-logo.txt" => Some((RUST_LOGO.as_bytes().to_vec(), "text/plain")),
        _ => None,
    }
}

/// Example showing asset metadata usage
pub fn print_asset_info() {
    println!("=== Binary Assets ===");
    println!("{ICON_PNG_INFO}");
    println!("{LOGO_SVG_INFO}");

    println!("\n=== Text Assets ===");
    println!("{RUST_LOGO_INFO}");
    println!("{RUST_BANNER_INFO}");

    println!("\n=== Runtime Statistics ===");
    println!(
        "RUST_LOGO: {} lines, {} chars",
        get_line_count(RUST_LOGO),
        get_line_count(RUST_LOGO)
    );
    println!(
        "BANNER: {} lines, {} chars",
        get_line_count(RUST_BANNER),
        get_char_count(RUST_BANNER)
    );
}

fn main() {
    // Serve assets
    if let Some((_, content_type)) = serve_asset("/icon.png") {
        println!("Serving /icon.png as {content_type}");
    }

    // Print asset information
    print_asset_info();
}
