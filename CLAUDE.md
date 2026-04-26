---
project: 3DNTerminal
tags: [rust, cosmic, terminal, rules]
agent: claude
type: rules
---

# Arbeitsregeln für Claude in diesem Projekt

## Build-Regeln
- **NIEMALS automatisch bauen** — vor jedem `cargo build`, `cargo check`, `cargo run` oder `nix develop` explizit fragen:
  > "Soll ich jetzt bauen? (cargo build / nix develop)"
- **NIEMALS automatisch `cargo run`** oder `nix develop --command cargo run` starten ohne Bestätigung.

## Git-Regeln
- **Keine automatischen Commits** — vor jedem `git commit` Bestätigung abwarten.
- **Kein `git push`** ohne explizite Aufforderung.
- **IMMER den vollständigen Befehl zeigen** — nie nur "ich committe jetzt":

```bash
git add src/ui/hologram.rs src/ui/window_controls.rs
git commit -m "feat: kurze beschreibung

- was geändert
- warum"
```

## Agent-Routing — Andere Agents bevorzugen
Bei komplexen Tasks immer den optimalen Agent vorschlagen statt selbst zu implementieren:

| Task | Agent | Vorbereiten |
|------|-------|-------------|
| Komplexe Bugs (Wayland, Rendering, Animationen) | **DeepSeek V4-Pro** (`ai-ds`) | Prompt vorbereiten |
| Lange Coding-Tasks (neue Features, Refactoring) | **Kimi K2.6** (`ai-kimi`) | Prompt vorbereiten |
| Bug-Recherche / Analyse | **Kimi K2.5** (`ai-kimi`) | Prompt vorbereiten |
| NixOS / Flake Probleme | **agent-nix** | Direkt starten |

**Regel:** Vor jeder größeren Implementierung fragen:
> "Soll ich das selbst machen oder einen Prompt für [Agent] vorbereiten?"

## Stack
- NixOS + Cosmic Desktop (Wayland)
- Rust, libcosmic, iced, wgpu
- Build: `nix develop --command cargo run --bin 3dnterm`
- Alias: `3dn`

---

# Agent Framework

## Was ist das?
Multi-Agent Orchestrator mit automatischem Wechsel zwischen:
- **Coding Agent** (DeepSeek V3) – Code schreiben
- **Test Agent** (DeepSeek R1) – Fehler debuggen
- **Review Agent** (Kimi K2) – Code Review
- **DevOps Agent** (DeepSeek V3) – Deploy vorbereiten

## Verwendung

```bash
# Im Projektverzeichnis:
python ~/agent-framework/orchestrator.py "Implementiere CRUD für Kabeltypen" --project .

# Mit eigener Config:
python ~/agent-framework/orchestrator.py "Fix auth bug" --config ./config.yaml
```

## Config anpassen
Kopiere `config.yaml` ins Projektverzeichnis und passe an:
- `test_command` – z.B. `pytest tests/ -v`
- `max_iterations` – wie oft soll der Fix-Loop laufen
- `run_devops` – DevOps Agent nach Tests aktivieren

## API Keys
Werden automatisch aus Umgebungsvariablen gelesen (via agenix):
- `DEEPSEEK_API_KEY`
- `NVIDIA_API_KEY`
- `GOOGLE_API_KEY`
