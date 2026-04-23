# 3DNTerminal v0.2 Architecture

## Overview
3DNTerminal is moving towards a modular, production-ready architecture. The application is divided into several decoupled domains to ensure maintainability, testability, and a clear separation of concerns.

## Modul-Struktur

### `src/main.rs`
- The central orchestrator (God Object resolved).
- Defines the `App` component (`cosmic::Application`).
- Manages the UI update loop (`AppEvent` processing) and delegates rendering logic.

### `src/app/`
- **state.rs**: Contains `AppState` and `AnimationPhase`. Tracks the animation progress, viewport geometry, and cursor visibility independently of the rendering framework.
- **events.rs**: Defines `AppEvent` for message passing within the UI loop.

### `src/ui/`
- **hologram.rs**: Encapsulates all 3D Canvas rendering logic. Handles the cross-product matrix transformations for the holographic terminal effect.
- **math.rs**: Core mathematical utilities (bezier curves, vector cross products for hit-testing).

### `src/terminal/`
- **mod.rs**: The `TerminalEngine`. Spawns the PTY and bridges it with the UI. Implements the `Terminal` trait.
- **traits.rs**: Defines the abstract `Terminal` interface, enabling mock-testing and future backend replacements.
- **grid.rs**: The thread-safe terminal state. Implements `vte::Perform` to parse ANSI escape sequences and manage the scrollback buffer.
- **input.rs**: Pure functional keyboard mapping. Converts `cosmic::iced` Keys to ANSI byte sequences.

### `src/config.rs` & `src/constants.rs`
- Centralized configuration system using the Builder Pattern.
- Supports robust default values, TOML file serialization, and Environment Variable overrides.

### `src/error.rs`
- Defines `AppError`, a unified error enum implementing standard `std::error::Error`. Provides clear error boundaries for PTY initialization and configuration.

## Design Decisions
1. **Thread-Safe Rendering**: The `TerminalGrid` is protected by an `Arc<Mutex>` so the PTY reader thread can parse ANSI concurrently while the GUI thread renders.
2. **Dirty Flag Optimization**: `TerminalGrid` tracks a `dirty` flag. `iced::Canvas::Cache` only redraws the text when mutations occur, ensuring 0% GPU load during idle periods.
3. **Builder Pattern Config**: Enforces strong validation (e.g., bounds checking for scrollback buffers) before instantiating the app.
