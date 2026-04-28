# 3DNTerminal — AGENTS.md
# Globale Projektdokumentation für alle AI-Agents

## System
- OS: NixOS + Cosmic Desktop (Wayland)
- Alias: `3dn` = `nix develop --command cargo run --bin 3dnterm`
- Hashtags: `#rust`, `#cosmic`, `#terminal`, `#barrierefrei`

## Stack
- Rust (iced + libcosmic + alacritty_terminal)
- Skill-Architektur (Modularer Trait-basiert)

---

## 🛡️ STABILITY MODE (VALIDIERT)
- **Status**: Phase 3.6 — Finaler Release Kandidat (RC1).
- **Architektur**: 
  - Modularer `TerminalSkill`-Trait für alle internen Module.
  - **Clean Navigation**: Hamburger-Menü als minimalistischer Starter.
  - **A11ySkill (Vollständig)**: Tremor-Filter, Farbblindheits-Transformationen, Motion-Reduction.
  - **Physics Engine**: Magnetischer Fokus + Breathe-Physik.
  - **Global Filter**: Alle UI-Elemente werden konsistent durch das A11Y-System transformiert.

---

## ✅ BEHOBENE BUGS (PHASE 3)

### Bug 6: Starre Overlay-Logik
- **Fix**: Dynamisches Skill-System mit Dashboard-Overlays implementiert.
- **Interaktivität**: Moderne Toggle-Switches (Slider-Look) in den Untermenüs.

---

## 🎮 UI-LOGIK (PHASE 3)

### 1. Skill-System (Plugins)
- **Trait**: `TerminalSkill` definiert das Verhalten der Dashboards.
- **Widgets**: Eigene Canvas-Widgets für Slider und Schalter.

### 2. Tremor-Kompensation
- **Logik**: Low-Pass-Filter in `main.rs` glättet `cursor_pos` via `a11y.tremor_damping`.

### 3. Magnetic Focus
- **Logik**: `lerp_rect` in `math.rs` für sanfte Fensterbewegungen zum Cursor hin.

---

## Regeln für alle Agents
- **NICHT automatisch bauen** ohne Bestätigung.
- **STOP after edit** — keine Verifikations-Loops ohne Feedback.
- **Diff-First** — Änderungen immer erst vorschlagen.
