//! Nannou Creative Coding Example
#[cfg(feature = "nannou-feature")]
fn main() {
    println!("🎨 Nannou Creative Framework - Coming Soon!");
}

#[cfg(not(feature = "nannou-feature"))]
fn main() {
    eprintln!("Nannou feature not enabled. Run: cargo run --bin nannou-app --features nannou-feature");
}
