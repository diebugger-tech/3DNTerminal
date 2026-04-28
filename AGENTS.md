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

## 🛡️ STABILITY MODE (VALIDIERT)
- **Status**: Phase 3.6 — Finaler Release Kandidat (RC1).
- **Security**: 
  - **SecuritySkill** implementiert (AI-Firewall Konzept).
  - **Human-in-the-Loop** Prinzip als Architektur-Standard.
- **Features**: 
  - A11ySkill (Tremor, Color Filters).
  - Physics Engine (Magnetic, Breathe).
  - Clean Navigation Dashboards.

---

## 🎮 UI-LOGIK (PHASE 3)

### 1. Skill-System (Plugins)
- **Trait**: `TerminalSkill` definiert das Verhalten der Dashboards.
- **Security**: AI-Firewall prüft alle automatisierten Tasks vor Ausführung.

### 2. Tremor-Kompensation
- **Logik**: Low-Pass-Filter in `main.rs` glättet `cursor_pos`.

### 3. Magnetic Focus
- **Logik**: `lerp_rect` in `math.rs` für organische Fensterbewegung.

---

## Regeln für alle Agents
- **NICHT automatisch bauen** ohne Bestätigung.
- **STOP after edit** — keine Verifikations-Loops ohne Feedback.
- **Security-First**: Alle AI-Interaktionen müssen das Security-Modul berücksichtigen.
