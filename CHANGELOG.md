# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.6] - 2026-04-28
### Added
- **Advanced A11Y**: Tremor compensation (low-pass filter) and global color blindness filters.
- **Magnetic Focus**: Physics-based UI that reacts to cursor proximity.
- **Modular Style System**: Centralized design tokens in `src/ui/style.rs`.
- **Toggle-Switch Widgets**: Modern sub-menu controls replacing legacy checkboxes.

### Changed
- Refactored `two_d.rs` to use centralized style tokens.
- Cleaned up Hamburger menu by moving granular controls to skill dashboards.
- Improved project documentation for community release (README, CONTRIBUTING, AGENTS).


## [0.3.5] - 2026-04-28
### Added
- **Modular Physics Architecture**: Introduced `PhysicsConfig` struct for granular control over effects.
- **Interactive Skill Extensions**: Added `draw_menu_extension` and `on_menu_click` to the `TerminalSkill` trait, enabling toggles/sliders directly in the Hamburger menu.
- **A11Y Mode**: Implemented a "Reduce Motion" master switch that disables autonomous animations (Breathe, Hamburger slide).
- **Physics Skill Enhancement**: Interactive status indicator in the sidebar that cycles through modular flags (Breathe, Minimal, Magnetic, A11Y).

### Changed
- Refactored `two_d.rs` to respect modular physics and A11Y flags.
- Updated `main.rs` to support the new skill-based interaction model.
- Cleaned up unused imports and improved code modularity.

## [0.2.1] - 2026-04-24
### Added
- **Window Controls**: Modular button system for Minimize, Maximize, and Close with hover states and tooltips.
- **3D Hit-Testing**: Perspective-aware mouse interaction for precise control on the holographic plane.
- **Tracing & Logging**: Integrated `tracing` crate with configurable log levels (`RUST_LOG`).
- **Feature Flags**: Modular build support for `logging`, `3d-effects`, and `experimental`.
- **Message Refactoring**: Unified `Message` system moved to `src/app/events.rs`.

### Changed
- Improved button rendering with Unicode icons ("−", "□", "×").
- Refined PTY lifecycle logging and error reporting.

## [0.2.0] - Refactoring & Stabilization

### Added
- **Configuration Builder**: `src/config.rs` added with robust builder pattern, TOML serialization, and validation.
- **Environment Variables**: Overrides like `3DNTERM_MAX_SCROLLBACK` supported.
- **Terminal Trait Interface**: Added `Terminal` trait in `src/terminal/traits.rs` to abstract backend logic for testing and swapping PTY implementations.
- **AppError Enum**: Robust `Result<T, AppError>` propagation in `src/error.rs` replacing panics and string errors.
- **ARCHITECTURE.md**: Comprehensive architectural documentation.

### Changed
- **Modularized UI Rendering**: Completely decoupled 3D rendering into `src/ui/hologram.rs`, drastically reducing `main.rs` size and complexity.
- **Encapsulated Grid State**: `TerminalGrid` mutex logic is now fully hidden behind the `TerminalEngine` interface.
- **Input Handling Refactoring**: Moved `Key` to ANSI byte mapping out of `main.rs` into `src/terminal/input.rs`.
- **Decoupled Application State**: Separated `AnimationPhase` and inner geometric state into `src/app/state.rs`.

### Removed
- Removed tightly coupled rendering loops from `main.rs`.

## [0.1.0] - Initial MVP Release
### Added
- Working PTY integration with `vte` parser.
- 3D Holographic Rendering with `iced::Canvas`.
- Slide-in and Slide-out animations (Flip & Glide).
- Echte 24-Bit TrueColor and 256-Color Palette Support.
- 60 FPS performance cap via Hardware Ticks and Canvas Cache.
