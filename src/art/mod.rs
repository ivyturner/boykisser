// Example usage and predefined assets

// Since we can't include actual files in this example, we'll use inline strings
// In a real implementation, you'd use include_str!("assets/rust_logo.txt")

#[cfg(feature = "rust")]
pub mod rust;

#[cfg(feature = "kissers")]
pub mod kissers;

/// Get line count for a text asset at runtime
#[must_use]
pub fn get_line_count(text: &str) -> usize {
    text.lines().count()
}

/// Get character count for a text asset at runtime
#[must_use]
pub fn get_char_count(text: &str) -> usize {
    text.chars().count()
}
