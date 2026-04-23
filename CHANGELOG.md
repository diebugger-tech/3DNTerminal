# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - Refactoring & Stabilization (Unreleased)

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
