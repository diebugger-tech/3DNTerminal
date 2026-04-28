# 3DNTerminal — AGENTS.md
# Globale Projektdokumentation für alle AI-Agents

## System
- OS: NixOS + Cosmic Desktop (Wayland)
- Alias: `3dn` = `nix develop --command cargo run --bin 3dnterm`
- Hashtags: `#rust`, `#cosmic`, `#terminal`, `#barrierefrei`, `#security`

## Stack
- Rust (iced + libcosmic + alacritty_terminal)
- Skill-Architektur (Modularer Trait-basiert)

---

## 🛡️ STABILITY MODE (AKTIV)
- **Status**: Phase 3.6.1 — Adaptive UI Update.
- **Security**: 
  - **SecuritySkill** implementiert (AI-Firewall Konzept).
  - **Human-in-the-Loop** Prinzip als Architektur-Standard.

### ⚠️ Bekannte Wayland-Bugs & Fixes
- **Button-Interaktion**: Auf Wayland/Cosmic kann `ButtonPressed` vom Compositor verschluckt werden. 
  - **Regel**: Klick-Events (Hit-Tests) MÜSSEN auf `ButtonReleased` reagieren.
- **Hit-Test Synchronität**: Alle Hit-Tests MÜSSEN `self.cursor_pos` (gedämpft) verwenden, um mit dem visuellen Crosshair übereinzustimmen.

---

## 🎮 UI-LOGIK (PHASE 3)

### 1. Adaptive Navigation
- **Corner-Jumping**: Direkter Wechsel zwischen Ecken ohne Zwischenschritt.
- **Toggle-Expand**: Klick auf die aktuelle Ecke (oder Pfeiltaste in Richtung der Ecke) vergrößert das Fenster.
- **Dynamic Menu**: Das Hamburger-Menü passt seine Höhe (`menu_h`) dynamisch an die Fensterhöhe an.

### 2. Tremor-Kompensation
- **Logik**: Low-Pass-Filter in `main.rs` glättet `cursor_pos`.

### 3. Magnetic Focus
- **Logik**: `lerp_rect` in `math.rs` für organische Fensterbewegung.

---

## 🤖 AGENT PROTOCOL (MANDATORY)
- **Tool usage**: Tools (`replace_file_content`, `run_command` etc.) MUST NOT be called autonomously.
- **Workflow**: 
  1. Analyze and Explain the problem.
  2. Propose a Diff in the chat.
  3. Wait for explicit "OK" before executing ANY tool.
- **Single File Rule**: One file = one commit. STOP after edit and wait for feedback.

## Regeln für alle Agents
- **NICHT automatisch bauen** ohne Bestätigung.
- **STOP after edit** — keine Verifikations-Loops ohne Feedback.
- **Security-First**: Alle AI-Interaktionen müssen das Security-Modul berücksichtigen.
