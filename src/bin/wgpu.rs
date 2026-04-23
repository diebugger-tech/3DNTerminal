//! WGPU Low-Level 3D Example
#[cfg(feature = "wgpu-feature")]
fn main() {
    println!("⚡ WGPU Low-Level Framework - Coming Soon!");
}

#[cfg(not(feature = "wgpu-feature"))]
fn main() {
    eprintln!("WGPU feature not enabled. Run: cargo run --bin wgpu-app --features wgpu-feature");
}
