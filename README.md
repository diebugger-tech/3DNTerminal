# 3DNTerminal

*Less app, more atmosphere.*

A floating, translucent terminal built with Rust and libcosmic, designed for the COSMIC Desktop. It belongs to your desktop—floating, fading, and staying out of the way until you need it.

## Features

- **Holographic Rendering**: Translucent UI with neon glow and organic "Breathe" animation.
- **Modular Skill System**: Extensible architecture for themes, physics, and settings.
- **Interactive Physics**: 
  - **Breathe**: Organic sinus-hover in corner states.
  - **A11Y Mode**: Integrated "Reduce Motion" master switch.
  - **Magnetic Focus**: (In progress) Buttons react to cursor proximity.
- **Adaptive Layout**: Snaps to all four screen corners with smooth 3D transitions.
- **Multi-Session**: Tab-based terminal sessions.
- **Wayland Native**: Built on iced and libcosmic for modern Linux desktops.

## Architecture

3DNTerminal follows a modular **Skill-based architecture**:
- **Core**: Handles PTY, grid rendering, and window state.
- **Skills**: Pluggable modules (Settings, Physics, Themes) that extend the UI and logic.
- **Hamburger Menu**: A dynamic control center that automatically integrates all registered skills.

## Installation

### Prerequisites
- **NixOS** or **Nix** with Flakes enabled.
- **COSMIC Desktop** (recommended for native experience).

### Build & Run
```bash
nix develop
cargo run --bin 3dnterm
```

## Keybindings
- **F12**: Toggle between Expanded (Center) and Collapsed (Corner) modes.
- **Minimize (−)**: Dock to system bar.
- **Maximize (□)**: Toggle center/corner.
- **Arrows (↖ ↗ ↙ ↘)**: Jump to specific corners.

## Configuration
Config is stored in `~/.config/3dnterminal/config.toml`.
Example `[physics]` section:
```toml
[physics]
breathe = true
magnetic = true
reduce_motion = false
```

## License
Licensed under the [Apache License, Version 2.0](LICENSE).
