# 3DNTerminal — AGENTS.md
# Globale Projektdokumentation für alle AI-Agents

## System
- OS: NixOS + Cosmic Desktop (Wayland)
- Alias: `3dn` = `nix develop --command cargo run --bin 3dnterm`

## Stack
- Rust (iced + libcosmic + alacritty_terminal)
- Skill-Architektur (Modularer Trait-basiert)

---

## 🛡️ STABILITY MODE (VALIDIERT)
- **Status**: Phase 3.6 - Advanced A11Y & Fine-grained Physics implementiert.
- **Architektur**: 
  - Modularer `TerminalSkill`-Trait für alle internen Module.
  - **Clean Navigation**: Hamburger-Menü als minimalistischer Starter (ohne Widgets).
  - **Dashboards**: Feingliedrige Steuerung via Toggle-Switches und Slidern in den Skill-Overlays.
  - **A11ySkill**: Inklusions-Zentrum mit Tremor-Filter und Farbtransformationen.
  - **Config v3**: Granulare Steuerung für jeden Effekt.

---

## ✅ BEHOBENE BUGS (PHASE 3)

### Bug 6: Starre Overlay-Logik
- **Fix**: Migration von hartcodierten Overlays auf ein dynamisches Skill-System.
- **Interaktivität**: Granulare Steuerung via Canvas-Widgets (Slider/Toggles) in den Skill-Overlays.
- **Physik**: Vollständige Modularisierung von Breathe- und A11Y-Effekten.

---

## 🎮 UI-LOGIK (PHASE 3)

### 1. Skill-System (Plugins)
- **Trait**: `TerminalSkill` definiert `draw_overlay`, `draw_menu_extension` und Klick-Handler.
- **Dynamik**: Das Hamburger-Menü baut sich automatisch aus allen registrierten Skills auf.
- **Widgets**: Skills nutzen `draw_slider` und `draw_toggle` für feingliedrige Einstellungen.

### 2. Tremor-Kompensation
- **Logik**: Low-Pass-Filter in `main.rs` glättet `cursor_pos` basierend auf `a11y.tremor_damping`.
- **Effekt**: Verhindert unabsichtliche Klicks und stabilisiert das Hover-Feedback.

---

## Regeln für alle Agents
- **NICHT automatisch bauen** ohne Bestätigung.
- **STOP after edit** — keine Verifikations-Loops ohne Feedback.
- **Diff-First** — Änderungen immer erst vorschlagen.
