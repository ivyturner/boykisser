//! Basic usage example for the boykisser asset library
use boykisser::{art, images};

fn main() {
    // Access and print ASCII art asset
    println!("ASCII Art (Rust Logo):\n{}", art::rust::RUST_LOGO);
    println!("Info: {}", art::rust::RUST_LOGO_INFO);

    // Access and print binary asset info
    println!(
        "\nBinary Asset (ICON_PNG): {} bytes",
        images::ICON_PNG.len()
    );
    println!("Info: {}", images::ICON_PNG_INFO);

    // Optionally, print the binary asset as base64 (for demonstration)
    let base64 = boykisser::utils::to_base64(images::ICON_PNG);
    let trunc_len = std::cmp::min(16, base64.len());
    println!(
        "\nICON_PNG as base64 (truncated): {}...",
        &base64[..trunc_len]
    );

    let ferris_base64 = boykisser::utils::to_base64(images::FERRIS_PNG);
    let ferris_trunc_len = std::cmp::min(16, ferris_base64.len());
    println!(
        "\nFERRIS_PNG as base64 (truncated): {}...",
        &ferris_base64[..ferris_trunc_len]
    );
}
