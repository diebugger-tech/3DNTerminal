# 3DNTerminal — TODO List

## 🛠️ Aktuelle Priorität (Stabilität & Bugfixes)
- [ ] **Sticky Buttons (COSMIC Fix)**: Implementierung der `pressed_button` Logik in `src/main.rs`, um verschluckte Klicks unter COSMIC/Wayland zu verhindern.
- [ ] **Physik-Defaults**: In `src/config.rs` die Standardwerte für `breathe` und `magnetic` auf `false` setzen.

## 🎨 Visuals & UI
- [ ] **BladeRunner Polish**: Finalisierung des "Rainy Night" Looks (Hologramm-Regen-Effekt-Mockup).
- [ ] **Menu Layout**: Dynamische Anpassung der Menühöhe (`menu_h`) an die aktuelle Fenstergröße.

## 🚀 Performance & Backend
- [ ] **Dirty Flag**: Neuzeichnen nur bei Terminal-Output triggern.
- [ ] **PTY Finalization**: Vollständiger Wechsel auf das Alacritty-Backend für alle Input-Events.
