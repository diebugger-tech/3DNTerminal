Erstelle EINE README.md im Root von ~/3DNTerminal sowie eine LICENSE-Datei 
und aktualisiere die Cargo.toml-Dateien. Sprache der README: Englisch.

═══════════════════════════════════════════════════════════════════════════
README.md — STRUKTUR
═══════════════════════════════════════════════════════════════════════════

1. TITLE & TAGLINE:

   # 3DNTerminal

   *Less app, more atmosphere.*

   A floating, translucent 3D terminal that fades in when you call it 
   and out when you don't — and a reusable GUI engine to build your own.

2. WHY?

   Terminals shouldn't feel like flat rectangles glued to a screen corner. 
   They should belong to the desktop — float, fade, and stay out of the 
   way until you need them.

3. SCREENSHOTS (Platzhalter):

   *[GIF: terminal expanding from corner — TODO]*
   *[Screenshot: collapsed corner state — TODO]*
   *[Screenshot: flip animation in progress — TODO]*

4. WHAT THE GUI DOES:
   - 3D perspective rendering of arbitrary content
   - Window states: Expanded (centered), Collapsed (corner, semi-transparent), 
     Hidden (planned: icon-only)
   - 3D-precise hit-testing — clicks land on the distorted quad, not the 
     bounding box
   - Edge-snapping in all four screen corners
   - Breathe animation, tilt effects, flip toggle
   - Wayland-native, transparent, undecorated

5. ARCHITECTURE — REUSABLE BY DESIGN:

   Cargo Workspace with two crates:
   - `threedn-shell`: The reusable 3D GUI engine. Backend-agnostic.
   - `threedn-terminal`: The reference implementation. Uses threedn-shell 
     with a concrete terminal backend.

   The shell knows nothing about PTY, ANSI, or terminal logic. It renders 
   what you give it.

   **Cross-platform note:** Currently Linux/Wayland only (developed on 
   COSMIC Desktop). Porting to other platforms is left to those who want 
   to. The shell is built on iced/wgpu, so the technical foundation is 
   portable — but libcosmic and Wayland-specific event handling are not.

6. DEVELOPER ROADMAP — BUILD YOUR OWN:

   Examples of what you could build with `threedn-shell`:
   - A holographic SSH/mosh client
   - A floating Docker/Kubernetes pod viewer
   - A REPL frontend for your own language

   The shell renders. You provide the content.

   *Note: Public API is unstable. The TerminalBackend trait is being 
   designed during Phase 2. Star the repo to follow progress.*

7. PROJECT ROADMAP:

   ✅ Workspace split complete
   ✅ 3D-precise hit-testing (buttons + flip working)
   ✅ Wayland-specific quirks resolved (ButtonReleased, opacity layer, 
      static anchors)

   🔜 Phase 1: Implement Hidden state (invisible with icon indicator), 
      remove quad-click toggle
   🔜 Phase 2: iced_term + alacritty_terminal integration (replaces custom 
      VTE stack)
   🔜 Phase 3: Daily-driver polish

8. BUILD & RUN:

   - NixOS / Nix environment required
   - `nix develop` mandatory (otherwise: cc not found)
   - Run command: VERIFY the correct binary name first via 
     `cat ~/3DNTerminal/Cargo.toml` or `ls crates/`. Do not invent names.
   - For debug output: `RUST_LOG=warn,threednterminal=debug cargo run --bin <name>`
   - F12 as toggle key (NOT ESC — breaks vim/nano/htop)

9. THE NAME:

   Started as **3DNeuroTerminal**. Got shortened to **3DNTerminal** because 
   typing the full name three times convinced me of nothing. The "N" still 
   means what it always meant.

10. LICENSE:

    ## License

    Licensed under the [Apache License, Version 2.0](LICENSE).

    Unless you explicitly state otherwise, any contribution intentionally 
    submitted for inclusion in this project shall be licensed as above, 
    without any additional terms or conditions.

11. FOOTER:

    GitHub: https://github.com/doko1975/3DNTerminal

═══════════════════════════════════════════════════════════════════════════
WEITERE DATEIEN
═══════════════════════════════════════════════════════════════════════════

A) Erstelle `LICENSE` im Repo-Root (~/3DNTerminal/LICENSE) mit dem 
   vollständigen, offiziellen Apache 2.0-Lizenztext.
   Quelle: https://www.apache.org/licenses/LICENSE-2.0.txt
   Den vollständigen Text einfügen, nicht nur einen Verweis.

B) Aktualisiere alle drei Cargo.toml-Dateien:
   - ~/3DNTerminal/Cargo.toml (Workspace-Root)
   - ~/3DNTerminal/crates/threedn-shell/Cargo.toml
   - ~/3DNTerminal/crates/threedn-terminal/Cargo.toml
   
   In jeder unter [package] (oder [workspace.package] im Root):
   license = "Apache-2.0"
   
   Bei der Workspace-Root prüfen, ob [workspace.package] existiert und 
   die Crates über `license.workspace = true` darauf verweisen können — 
   das ist eleganter als Duplikation. Falls nicht vorhanden: einfach in 
   jeder Crate-Cargo.toml einzeln setzen.

═══════════════════════════════════════════════════════════════════════════
RULES
═══════════════════════════════════════════════════════════════════════════

- VERIFY all concrete code claims (crate names, binary names) by reading 
  the actual files. Invent nothing.
- NO invented trait signatures or API examples.
- NO words like "cyberpunk", "futuristic", "next-gen", "revolutionary", 
  "Year 2099".
- Do NOT explain what the "N" stands for beyond what's written in 
  section 9. The reader figures it out or doesn't.
- No emoji inflation. Roadmap checkmarks are fine, otherwise sparse.
- Style: calm, precise, developer-friendly. Linux-kernel-doc, not 
  startup-pitch.
- English throughout (README + code comments).
- Apache 2.0 LICENSE file must contain the FULL official text — no 
  abbreviations, no summaries.

═══════════════════════════════════════════════════════════════════════════
WORKFLOW
═══════════════════════════════════════════════════════════════════════════

1. Show me the complete README content for review BEFORE writing it to disk.
2. Show me the planned Cargo.toml changes (diff or full sections) BEFORE 
   applying them.
3. The LICENSE file can be written directly (it's a standard text).
4. After review approval, write all files.
