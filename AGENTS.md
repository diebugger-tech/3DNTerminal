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
- **Status**: Phase 3.7 — Deep Theming & Unified Icons.
- **Security**: 
  - **SecuritySkill** implementiert (AI-Firewall Konzept).
  - **Human-in-the-Loop** Prinzip als Architektur-Standard.

### ⚠️ Bekannte Wayland-Bugs & Fixes
- **Button-Interaktion**: Auf Wayland/Cosmic kann `ButtonPressed` vom Compositor verschluckt werden. 
  - **Regel**: Klick-Events (Hit-Tests) MÜSSEN auf `ButtonReleased` reagieren.
- **Hit-Test Synchronität**: Alle Hit-Tests MÜSSEN `self.cursor_pos` (gedämpft) verwenden, um mit dem visuellen Crosshair übereinzustimmen.

---

## 🎮 UI-LOGIK (PHASE 3)

### 1. Unified Icon System
- **Konzept**: Themes wie Classic, Apple und Transparent nutzen ausschließlich **Unicode-Symbole** (☰, +, ✕, ↗) für absolute Schärfe und einheitliche Strichstärken.
- **Hybrid-Engine**: Technische Themes (Blade Runner, Retro) nutzen spezialisierte Vektoren mit themenspezifischen Geometrien (Targeting-Boxen, Brackets).

### 2. Smart UI Management
- **Auto-Close**: Menüs und Overlays schließen sich automatisch, wenn das Fenster schrumpft oder unter eine kritische Größe (500x400) fällt.
- **Deep Theming**: Buttons ändern ihre Form (Achteck, Kreis, Klammer) dynamisch basierend auf dem gewählten Skill-Theme.

### 3. Adaptive Navigation & Physics
- **Corner-Jumping**: Direkter Wechsel zwischen Ecken ohne Zwischenschritt.
- **Magnetic Focus**: `lerp_rect` in `math.rs` für organische Fensterbewegung.

---

## 🤖 AGENT PROTOCOL (MANDATORY)
- **Planning First**: Analyze and explain the problem + propose a Diff. 
- **Tool usage**: Tools (`replace_file_content`, `run_command` etc.) MUST NOT be called until the user says **"bau"** or **"build"**.
- **Single File Rule**: One file = one commit. STOP after edit and wait for feedback.

## Regeln für alle Agents
- **NICHT automatisch bauen** ohne Bestätigung ("bau").
- **STOP after edit** — keine Verifikations-Loops ohne Feedback.
- **Security-First**: Alle AI-Interaktionen müssen das Security-Modul berücksichtigen.
