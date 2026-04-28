# 3DNTerminal — TODO List

## ✅ Erledigt (Aktuelle Session)
- [x] **Sticky Buttons (COSMIC Fix)**: Robuste Klick-Erkennung durch State-Tracking (fix für Wayland/COSMIC).
- [x] **Physik-Defaults**: Standardwerte für `breathe` und `magnetic` auf `false` gesetzt (ruhigeres Standard-Look).
- [x] **BladeRunner Polish**: "Rainy Night" Hologramm-Regen-Effekt implementiert.
- [x] **Menu Layout**: Dynamische Menühöhe passt sich Fenstergröße an.
- [x] **Unpinned Interaction**: Ziehen und Resizen (Corner-Grip) direkt aus jeder Ecke möglich.
- [x] **Mouse Feedback**: Visuelle Cursor-Icons (Grab, Grabbing, Pointer) integriert.
- [x] **Full Resizing**: Resizing von allen 4 Seiten und Ecken implementiert.
- [x] **Draggable Area**: Fix für Window-Dragging mit sauberen Hit-Test-Grenzen.

## 🎨 Visuals & UI
- [x] **Classic Design Restore**: Rückkehr zu Emojis und puristischem Layout.
- [x] **Clean Project**: Alle Hintergrund-Effekte (Regen, Sterne etc.) wurden entfernt.
- [ ] **Glow Toggle**: Dynamisches Ein/Ausschalten des Neon-Glows via Menü.
- [ ] **Classic Themes**: Weitere schlichte Farbvarianten (Amber, White, Green) ohne Effekte.

## 🚀 Performance & Backend
- [ ] **Dirty Flag**: Neuzeichnen nur bei Terminal-Output triggern.
- [ ] **PTY Finalization**: Vollständiger Wechsel auf das Alacritty-Backend für alle Input-Events.
- [ ] **Cursor Blink**: Implementierung des Cursor-Blinkens im Alacritty-Grid.
- [ ] **Arrow Navigation**: Ecken-Wechsel per Pfeiltasten implementieren.

## 🌌 Phase 4: Final Vision
- [ ] **Holographic Theme**: Maximal transparente Shell, nur Schrift und Rahmen sichtbar.
- [ ] **3D Theme Engine**: Räumliche Darstellung des Terminals im Desktop-Space.
- [ ] **Command Flow FX**: Visuelle Effekte beim schnellen Scrolling.
