# 3DNTerminal — AGENTS.md
# Globale Projektdokumentation für alle AI-Agents
# (Claude, Kimi, DeepSeek, IronClaw, Goose, pi)

## System
- OS: NixOS + Cosmic Desktop (Wayland)
- Zweit-System: Ubuntu
- Shell: antigravity-nix FHS
- Alias: `3dn` = `nix develop --command cargo run --bin 3dnterm`

## Stack
- Rust (egui/eframe + ratatui + vte + alacritty_terminal)
- libcosmic / iced
- Ollama (lokal)
- DeepSeek V4-Flash/Pro (API)
- Kimi K2.5 (Bug-Recherche) + K2.6 Terminal (Long-Session)
- IronClaw + OpenCode + Claw Code
- pi (pi.dev, RPC-Modus)
- Goose (Block, MCP)
- MiniMax M2.7 (Speech 2.8 für TTS)

## Binaries
Das Projekt hat 4 Binaries — immer `--bin` angeben:
```bash
cargo run --bin 3dnterm   # Haupt-Terminal
cargo run --bin bevy
cargo run --bin nannou
cargo run --bin wgpu
```

## Projekt-Ziele
1. Hologramm-Terminal mit Corner-Flip (egui + ratatui)
2. AI-Sidebar (pi / Goose / Claude / DeepSeek / Kimi)
3. TTS via piper-tts oder qwen3_tts_rs (lokal)
4. Ollama als lokales LLM-Backend
5. Später: Forum/Web via WASM (ratzilla oder eframe WASM)

---

## 🛡️ STABILITY MODE (ERFOLGREICH VALIDIERT)
- **Status**: 2D-Basis und Hit-Testing stabilisiert.
- **Resultat**: 100% zuverlässige Interaktion durch Canvas-lokale Koordinaten.
- **Wiederherstellung**: Die Basis sitzt. Reaktivierung des 3D-Effekts ist der nächste Schritt (Phase 3).

---

## 🎮 UI-LOGIK (PHASE 2 - COMPLETED)

### 1. Die Pfeile (↖ ↗ ↙ ↘)
- **Funktion**: Schicken das Terminal sofort in die jeweilige Ecke (`Collapsed`-Modus).
- **Corner-Jumping**: Man kann direkt von Ecke zu Ecke springen.
- **Toggle**: Klick auf die *aktuelle* Ecke schickt das Terminal zurück in die Mitte (`Expanded`).

### 2. Minimieren (−)
- **Aktion**: Nutzt `core.minimize(None)` für echtes OS-Docking.
- **Visual**: Fenster verschwindet in das Cosmic/Wayland Dock.
- **Restore**: Über das System-Dock wiederherstellbar.

### 3. Maximieren (□)
- **Aktion**: Toggle zwischen Mitte (`Expanded`) und der letzten Ecke (`Collapsed`).

### 4. Schließen (×)
- **Aktion**: Beendet die Anwendung vollständig (`std::process::exit(0)`).

---

## 🛡️ STRENGES PROTOKOLL (MANDATORY)

1. **Diff-First**: Jede Code-Änderung MUSS zuerst als Text-Diff (Markdown) im Chat vorgeschlagen werden.
2. **Keine autonomen Änderungen**: Tools wie `replace_file_content` dürfen ERST gerufen werden, wenn der User den Diff explizit bestätigt hat.
3. **Erklären vor Handeln**: Erst das "Was" und "Warum" im Chat klären, dann das "Wie" als Diff zeigen.
4. **Backup-Pflicht**: Vor jeder Dateiänderung: `cp <datei> <datei>.bak`.
5. **STOP after edit**: Nach einer Änderung sofort anhalten und Feedback abwarten.

---

## ⚠️ CORNER-FLIP — NICHT ÜBERSCHREIBEN!

Das Fenster springt per 4 Buttons in jede Ecke. Das ist ein Kern-Feature.

### State
```rust
#[derive(Clone, Copy, PartialEq, Default)]
enum Corner {
    #[default] Free,
    TopLeft, TopRight, BottomLeft, BottomRight,
}
```

### Buttons (im Fenster)
```rust
for (lbl, c) in [
    ("↖", Corner::TopLeft),
    ("↗", Corner::TopRight),
    ("↙", Corner::BottomLeft),
    ("↘", Corner::BottomRight),
] {
    if ui.button(lbl).clicked() {
        self.corner = if self.corner == c { Corner::Free } else { c };
    }
}
```

### Rendering
```rust
let base_win = egui::Window::new("Terminal")
    .movable(self.corner == Corner::Free);

let base_win = match self.corner {
    Corner::Free        => base_win.default_pos(self.pos),
    Corner::TopLeft     => base_win.anchor(Align2::LEFT_TOP,     Vec2::new( 8.0,  8.0)),
    Corner::TopRight    => base_win.anchor(Align2::RIGHT_TOP,    Vec2::new(-8.0,  8.0)),
    Corner::BottomLeft  => base_win.anchor(Align2::LEFT_BOTTOM,  Vec2::new( 8.0, -8.0)),
    Corner::BottomRight => base_win.anchor(Align2::RIGHT_BOTTOM, Vec2::new(-8.0, -8.0)),
};
```

### Regeln
- `movable(false)` wenn Corner aktiv
- `movable(true)` wenn `Corner::Free`
- Aktiver Button = blau hervorgehoben
- Toggle: nochmal klicken = zurück zu Free

---

## Hologramm-Rotation (hologram.rs)

### get_quad_for_corner() — Pflicht-Parameter
- `get_quad_for_corner(corner: CornerPosition)` statt `get_quad()` — corner ist immer Pflicht
- Die Quad-Geometrie (p1–p4) muss pro Ecke korrekt gespiegelt werden

### Quad-Achsen pro Ecke
| Ecke | X-Achse | Y-Achse | Tilt-Winkel |
|------|---------|---------|-------------|
| BottomRight | normal | normal | −18° (Referenz, funktioniert) |
| BottomLeft | **vertauscht** | normal | −18° |
| TopLeft | normal | **invertiert** | +18° |
| TopRight | **vertauscht** | **invertiert** | +18° |

- `p1` = oben-links, `p2` = oben-rechts (Definition für BottomRight als Referenz)
- `collapsed tilt`: Bottom-Ecken = −18°, Top-Ecken = +18°

### Button-Anker (hit_test + draw)
- **Right-Ecken** (BottomRight, TopRight): Buttons **rechtsbündig** → Anker = `p2`
- **Left-Ecken** (BottomLeft, TopLeft): Buttons **linksbündig** → Anker = `p1`
- Anker muss nach jedem `SetCorner` und `WindowResized` neu berechnet werden

### Hidden-Icon — eckenbewusst
Das kleine Cyan-Rechteck muss in der aktiven Ecke erscheinen:
```rust
match active_corner {
    BottomRight => (corner_rect.x + corner_rect.width - 20, corner_rect.y + corner_rect.height - 20),
    BottomLeft  => (corner_rect.x + 4,                      corner_rect.y + corner_rect.height - 20),
    TopRight    => (corner_rect.x + corner_rect.width - 20, corner_rect.y + 4),
    TopLeft     => (corner_rect.x + 4,                      corner_rect.y + 4),
}
```

---

## Regeln für alle Agents

- **NICHT automatisch bauen** ohne Bestätigung — vor `cargo build/run` oder `nix develop` immer fragen
- **Keine automatischen git commits** — immer Bestätigung abwarten
- **Beim Committen IMMER den vollständigen Befehl zeigen** zum Kopieren, nie nur "ich committe jetzt":
  ```bash
  git add <dateien>
  git commit -m "typ: kurze beschreibung

  - was geändert
  - warum"
  ```
- **Bei komplexen Bugs** → Prompt für **DeepSeek V4-Pro** vorbereiten (1M Context, ganzer Codebase)
- **Bei langen Coding-Tasks** → Prompt für **Kimi K2.6** vorbereiten (300 Sub-Agenten, Long-Session)
- **Bei Bug-Recherche** → Prompt für **Kimi K2.5** vorbereiten (DeepSearch, Log-Analyse)
- Immer den **optimalen Agent vorschlagen** statt selbst zu bauen

---

## ✅ BEHOBENE BUGS (PHASE 2)

### Bug 1: Wayland schluckt Events / Koordinaten-Offset
- **Ursache**: Globale Wayland-Events vs. lokale Iced-Bounds + 65px Hardcoded-Offset.
- **Fix**: Komplette Migration auf `iced::canvas::Program::update`. Nutzung von `cursor.position_in(bounds)` für native, relative Koordinaten.

### Bug 2: Klick-Swallowing (Multi-Click-Problem)
- **Fix**: Ausführung von Button-Aktionen direkt bei `CanvasButtonPressed` statt erst bei `Released`. Verhindert das "Abrutschen" des Cursors während der Animation.

---

## Bug-Workflow
1. **Kimi K2.5** → Log + Codebase analysieren, Bug lokalisieren
2. **DeepSeek V4-Pro** → Fix schreiben (1M Context, ganzen Codebase lesen)
3. **Kimi K2.6** → automatisch testen + committen

## AI-Sidebar Agents
| Agent | Zweck |
|---|---|
| pi (pi.dev) | RPC via stdin/stdout, minimaler Coding-Agent |
| Goose (Block) | 70+ MCP Extensions, autonome Tasks |
| Claude | Planung, Architektur, Debugging-Diskussion |
| DeepSeek V4-Flash | Schnelle Fragen, billig |
| DeepSeek V4-Pro | Komplexes Rust-Debugging, 1M Context |
| Kimi K2.5 | Bug-Recherche, DeepSearchQA |
| Kimi K2.6 | Long-Session Coding, 300 Sub-Agenten |

## Obsidian Vault
- Vault-Pfad: TODO (vom User eintragen)
- Ordner: 3DNTerminal/, NixOS-Config/, AI-Agents/, Bug-Log/
- AGENTS.md liegt neben flake.nix

## Symlinks (nach Vault-Pfad eintragen)
```bash
ln -s ~/path/to/AGENTS.md ~/.claude/AGENTS.md
ln -s ~/path/to/AGENTS.md ~/.openclaw/SOUL.md
ln -s ~/path/to/AGENTS.md ~/.ironclaw/AGENTS.md
ln -s ~/path/to/AGENTS.md ~/.config/goose/AGENTS.md
```
