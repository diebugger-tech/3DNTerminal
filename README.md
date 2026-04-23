# 3DNTerminal 🎮

A high-performance, GPU-accelerated holographic 3D terminal interface written in Rust. Features a "click-to-flip" cyberpunk aesthetic (Apple Vision Pro meets Year 2099 Linux) built on `libcosmic` and `iced`, with true PTY integration via `vte` and `portable-pty`.

## 🚀 Features

- **True PTY Integration**: Full ANSI/VT100 support (`vim`, `htop`, `nano` run flawlessly).
- **True Color Support**: Supports 24-bit RGB and 256-color ANSI palettes.
- **Scrollback Buffer**: GPU-optimized caching and efficient 1000-line history with mouse wheel support.
- **Holographic 3D UI**: Real-time perspective transformations with fluid expand/collapse animations.
- **Sleek Interactions**: Click header to minimize, double-click to toggle, or press `F12`.
- **High Performance**: Locked 60 FPS (16ms ticks) with intelligent dirty-flag canvas caching for 0% idle load.

## ⌨️ Keybindings & Interactions

- **`F12`**: Toggle terminal (Expanded <-> Collapsed)
- **`Double-Click`**: Toggle terminal when expanded
- **`Header Click`**: Click the top 15% (System Header) of the expanded terminal to minimize it.
- **`Mouse Wheel`**: Scroll through terminal history buffer.
- **`Click (Collapsed)`**: Click anywhere on the collapsed corner-widget to expand it.

## 🛠️ Installation & Build from Source

### Prerequisites
- NixOS (or Nix package manager with flakes enabled)
- Rust toolchain (Cargo)

### Build
```bash
# Clone the repository
git clone https://github.com/doko1975/3DNTerminal.git
cd 3DNTerminal

# Enter the nix development shell (provides all necessary dependencies like wgpu, wayland, etc.)
nix develop

# Run the terminal
cargo run --bin 3dnterm
```

## 🐛 Troubleshooting

- **Black screen / No rendering**: Ensure your GPU drivers support Vulkan/WGPU. The app relies on hardware acceleration.
- **Build failures**: Make sure you are inside the `nix develop` shell before running `cargo`.
- **Text clipping**: The terminal dynamically scales based on window size. Try resizing the window if text overflows.

## 🏗️ Architecture

- `src/main.rs`: Core application loop, event routing, and window initialization.
- `src/ui/`: Contains the 3D-Canvas rendering logic and layout.
- `src/terminal/`: Manages the underlying PTY process, VT100 parsing (`vte`), and thread-safe grid state.
- `src/effects/`: Neural-net/Circuit background rendering on a dedicated thread.
