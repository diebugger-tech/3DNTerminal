# 3DNTerminal v0.3.6

> *Less app, more atmosphere. More inclusive, more alive.*

`#rust` `#cosmic` `#terminal` `#barrierefrei` `#accessibility` `#wayland`

A floating, translucent terminal built with Rust and libcosmic, designed for the COSMIC Desktop. It belongs to your desktop—floating, fading, and staying out of the way until you need it.

---

## 🌈 Accessibility First (Barrierefrei)

3DNTerminal isn't just about looks. We believe power tools should be inclusive.
- **Tremor Compensation**: Integrated low-pass filter to smooth out shaky cursor movements, making the UI easier to navigate for users with motor impairments.
- **Vision Filters**: Real-time color transformations for **Protanopia**, **Deuteranopia**, and **Tritanopia**.
- **Motion Reduction**: Granular control over animation damping to accommodate users sensitive to motion.

## 🧲 Physics-based UI

- **Magnetic Focus**: The entire window reacts organically to your cursor proximity, giving the UI a "physical" presence on your desktop.
- **Sinusoidal "Breathe"**: Organic hovering animations in the collapsed state.
- **Holographic Rendering**: Glassmorphism effects with adaptive neon glow.

## 🧩 Modular Skill System

Extend the terminal with your own logic:
- **Skills**: Pluggable modules for Themes, Physics, and A11Y.
- **Dashboards**: Clean Hamburger navigation with detailed sub-menu control centers.

## 🚀 Roadmap

- [ ] Full Alacritty PTY/Shell Integration (v0.4.0)
- [ ] AI-Assisted Terminal Skills (Ollama Integration)
- [ ] Screen Reader & TTS Support
- [ ] Bidirectional Knowledge Sync (Obsidian/SurrealDB)

## Installation

### Prerequisites
- **NixOS** or **Nix** with Flakes enabled.
- **COSMIC Desktop**.

### Build & Run
```bash
nix develop
cargo run --bin 3dnterm
```

## Contributing
We love community contributions! Check out [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

## License
Licensed under the [Apache License, Version 2.0](LICENSE).
