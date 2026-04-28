# 3DNTerminal Skill System

Das Skill-System ermöglicht es, das Terminal modular zu erweitern. Jeder Skill ist eine abgeschlossene Einheit, die im Hamburger-Menü erscheint und ein eigenes Einstellungs-Overlay besitzen kann.

## Das TerminalSkill Trait

Jeder neue Skill muss das `TerminalSkill`-Trait implementieren:

```rust
pub trait TerminalSkill: Send + Sync {
    fn id(&self) -> &'static str;           // Eindeutige ID (z.B. "physics")
    fn label(&self) -> &'static str;        // Text im Menü
    fn subtitle(&self) -> &'static str;     // Beschreibung im Menü
    fn color(&self) -> Color;               // Akzentfarbe des Skills
    
    // Zeichnet das Haupt-Fenster (Overlay)
    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams);
    
    // Zeichnet interaktive Elemente direkt im Hamburger-Menü (Slider/Toggles)
    fn draw_menu_extension(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {}

    // Click-Handler für das Overlay
    fn on_click(&self, pos: Point, rect: Rectangle, config: &mut Config) -> bool { false }

    // Click-Handler für die Menü-Erweiterung (Sidebar)
    fn on_menu_click(&self, pos: Point, rect: Rectangle, config: &mut Config) -> bool { false }
}
```

## Integration

1.  Erstelle eine neue Datei in `src/ui/skills/`.
2.  Implementiere den Trait für dein Struct.
3.  Registriere den Skill in `src/ui/skills/mod.rs` in der Funktion `get_all_skills()`.

## Beispiel: Toggle-Switch im Menü

Um einen Schalter im Menü anzuzeigen (wie der Glow-Toggle), nutze `draw_menu_extension`. Der `rect`-Parameter gibt dir den Bereich auf der rechten Seite des Menü-Items vor.

```rust
fn draw_menu_extension(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
    // Zeichne hier einen Schalter oder Slider
}
```
