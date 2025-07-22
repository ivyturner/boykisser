//! Basic usage example for the boykisser asset library
use boykisser::art;

fn main() {
    // Access and print ASCII art asset
    println!(
        "ASCII Art (boykisser smooch):\n{}",
        art::kissers::BOYKISSER_SMOOCH
    );
    println!("Info: {}", art::kissers::BOYKISSER_SMOOCH_INFO);
}
