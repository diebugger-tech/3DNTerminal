# 3DNTerminal

*Less app, more atmosphere.*

A floating, translucent terminal built with Rust and libcosmic, designed for the COSMIC Desktop. It belongs to your desktop—floating, fading, and staying out of the way until you need it.

## Features

- **Holographic Rendering**: Translucent UI with neon glow and organic "Breathe" animation.
- **Modular Skill System**: Extensible architecture for themes, physics, and settings.
- **Advanced A11Y**: 
  - **Tremor Compensation**: Low-pass filter for smooth cursor interaction.
  - **Color Filters**: Real-time transformations for Protanopia, Deuteranopia, and Tritanopia.
  - **Motion Reduction**: Granular control over animation damping.
- **Fine-grained Physics**: Individual dashboards for Breathe intensity and Magnetic radius.
- **Adaptive Layout**: Snaps to all four screen corners with smooth 3D transitions.
- **Multi-Session**: Tab-based terminal sessions.
- **Wayland Native**: Built on iced and libcosmic for modern Linux desktops.

## Architecture

3DNTerminal follows a modular **Skill-based architecture**:
- **Core**: Handles PTY, grid rendering, and window state.
- **Skills**: Pluggable modules (Settings, Physics, Themes, A11Y) that extend the UI and logic.
- **Navigation**: The Hamburger menu acts as a starter, while Skill Overlays provide detailed control dashboards.

## Installation

### Prerequisites
- **NixOS** or **Nix** with Flakes enabled.
- **COSMIC Desktop** (recommended for native experience).

### Build & Run
```bash
nix develop
cargo run --bin 3dnterm
```

## License
Licensed under the [Apache License, Version 2.0](LICENSE).
