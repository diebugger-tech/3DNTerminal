# Contributing to 3DNTerminal

We welcome contributions from everyone! Whether you're fixing a bug, adding a new "Skill," or improving accessibility, your help is appreciated.

## How to Help

### 1. Accessibility (A11Y)
Help us make the terminal more inclusive. We're looking for:
- Better color transformation matrices.
- Improved cursor stabilization algorithms.
- Support for screen readers or other assistive technologies.

### 2. Physics & Atmosphere
Help us refine the "Cyberpunk" feeling:
- New `TerminalSkill` implementations for organic animations.
- Improved magnetic focus logic.
- Performance optimizations for the canvas-based rendering.

### 3. Backend Integration
- Finalizing the Alacritty-based PTY integration.
- Improving scrollback performance.

## Getting Started

1. **Fork the repo** and clone it locally.
2. Ensure you have **Nix** and **Rust** installed.
3. Run `nix develop` to set up the environment.
4. Make your changes and submit a **Pull Request**.

## Architecture Overview

- **`src/ui/skill/`**: The core trait for all extensions. Start here if you want to add a new menu or effect.
- **`src/ui/two_d.rs`**: The main rendering loop. 
- **`src/config.rs`**: The central configuration structure.

## Licensing

By contributing to 3DNTerminal, you agree that your contributions will be licensed under the **Apache License, Version 2.0**.
