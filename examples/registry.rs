// Example of using the create_asset_registry macro
// This will create a module `default_assets` with one text asset (RUST_LOGO)
boykisser::create_asset_registry!(
    default_assets,
    [], // No binary assets
    [(RUST_LOGO, "../assets/boykisser.txt", "Boykisser ASCII art")]
);

fn main() {
    // Access the text asset from the registry
    println!("Boykisser art:\n{}", default_assets::RUST_LOGO);
    // List all text assets in the registry
    for info in default_assets::list_text_assets() {
        println!("Asset info: {info}");
    }
}
