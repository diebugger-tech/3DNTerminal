# Modularität im 3DNTerminal

Das Projekt folgt einer strikten modularen Philosophie, um die Komplexität des holografischen Terminals beherrschbar zu halten.

## Dateistruktur

- `src/ui/mod.rs`: Zentrale Registrierung aller UI-Komponenten.
- `src/ui/skill.rs`: Definition des Skill-Traits (Das Interface).
- `src/ui/skills/`: Das "Plugin"-Verzeichnis. Jede Datei hier ist ein eigenständiges Modul.
- `src/ui/two_d.rs`: Der Orchestrator. Er zeichnet das Terminal und delegiert Aufgaben an die Module.

## Warum Modularität?

1.  **Isolation**: Ein Bug in der Physik-Steuerung legt nicht das ganze Menü lahm.
2.  **Erweiterbarkeit**: Neue Funktionen (Skills) können hinzugefügt werden, ohne die `main.rs` zu verändern.
3.  **Testbarkeit**: Module können (zukünftig) unabhängig voneinander getestet werden.
4.  **Blade Runner Design**: Erlaubt uns, für jeden Aspekt der "Engine" ein eigenes, spezialisiertes UI-Panel zu bauen.

## Best Practices

- Vermeide direkte Abhängigkeiten zwischen Skills.
- Nutze die `TerminalParams`, um auf globale Zustände (wie Farben oder Animationen) zuzugreifen.
- Halte die `draw`-Funktionen rein (keine Zustandsänderungen beim Zeichnen).
