//! Bevy 3D Example
#[cfg(feature = "bevy-feature")]
fn main() {
    println!("🎮 Bevy 3D Framework - Coming Soon!");
}

#[cfg(not(feature = "bevy-feature"))]
fn main() {
    eprintln!("Bevy feature not enabled. Run: cargo run --bin bevy-app --features bevy-feature");
}
