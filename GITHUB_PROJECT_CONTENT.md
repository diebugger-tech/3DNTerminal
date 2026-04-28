# 3DNTerminal — GitHub Project & Issue Content

Copy and paste these items into your GitHub Project Board or create them as Issues.

---

## 🏗️ v0.4.0: Alacritty PTY Integration
**Description**: 
Transition from the current frontend mock-grid to a fully functional terminal using the `alacritty_terminal` crate as the PTY/Parser backend.
- Connect to Bash/Zsh/Nix-shell.
- Implement scrollback buffer.
- Ensure 2D Canvas rendering handles all ANSI escape sequences.

---

## 🤖 v0.5.0: AI Skill - Ollama Support
**Description**: 
Integrate local AI agents via Ollama.
- Create a dedicated `AiSkill` module.
- Implement the "AI-Firewall" in the `SecuritySkill` to prevent unauthorized command execution.
- Add an overlay for chatting with the terminal context.

---

## 🧠 v0.6.0: Obsidian & SurrealDB Sync
**Description**: 
Turn the terminal into a knowledge hub.
- Develop a plugin for bidirectional sync with Obsidian Vaults.
- Use SurrealDB as a graph-based memory for terminal sessions.
- Allow AI agents to query the "Second Brain" for context.

---

## 🔊 A11Y: Screen Reader & TTS Support
**Description**: 
Make the 3D/2D interface accessible for visually impaired users.
- Integrate a Text-to-Speech (TTS) engine for terminal output.
- Add ARIA-like labels for the Canvas-based UI elements.
- Refine the tremor-compensation filters based on community feedback.

---

## 🎨 Themes & Atmosphere: VHS Distortion Skill
**Description**: 
The ultimate Cyberpunk aesthetic upgrade.
- Implement an optional "Visual Glitch" skill using shaders or canvas effects.
- Add "Dirty Flag Glow" (window glow reacts to terminal activity).
