# 3DNTerminal 🎮

Ein modulares 3D-Terminal-Projekt in Rust mit Support für mehrere Frameworks.

## Features

- **Bevy**: Game Engine Framework
- **Nannou**: Creative Coding & Art
- **WGPU**: Low-Level 3D Rendering
- **Modular Design**: Alle Frameworks optional

## Getting Started

```bash
cd ~/3DNTerminal
nix develop /etc/nixos#rust-3d

# Run Bevy example
cargo run --bin bevy-app --features bevy-feature

# Run Nannou example
cargo run --bin nannou-app --features nannou-feature

# Run WGPU example
cargo run --bin wgpu-app --features wgpu-feature
```

## Development

```bash
cargo check                # Syntax check
cargo build --release      # Optimized build
cargo test                 # Run tests
cargo clippy               # Linter
cargo fmt                  # Format code
```

## TUI Integration

Later: Ratatui for Terminal UI

## Project Status

🚀 In Development
