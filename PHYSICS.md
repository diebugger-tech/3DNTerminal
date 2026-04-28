# PHYSICS.md — 3DNTerminal Physik-Engine

---

## Bereits implementierte Physik (v0.3.5)

### Modular Physics (Neu)
Alle Effekte sind nun einzeln schaltbar über den `PhysicsSkill` im Hamburger-Menü.

### Breathe-Physik (Sinus-Hover)
Organisches Schweben im Collapsed-State via Sinus-Welle. 
- Status: ✅ Implementiert (Modular schaltbar)

### A11Y-Integration (Reduce Motion)
Globaler Master-Switch für Barrierefreiheit. Deaktiviert alle autonomen Animationen.
- Status: ✅ Implementiert (Modular schaltbar)

### Dynamisches Button-Anchoring
Buttons kleben physikalisch korrekt an der Terminal-Kante.
- Status: ✅ Implementiert

### Transition-Physik (Corner-Flip)
Zwei-phasige Bewegung beim State-Wechsel via Cubic-Bezier Easing.
- Status: ✅ Implementiert

---

## Geplante Physik-Engine (Phase 6)

### Feder-Physik / Spring Drag
Fenster federt leicht nach beim Loslassen.
Interpolierte Animation mit konfigurierbarer Dämpfung.
Im A11Y-Mode: sofort, kein Überschießen.

### Momentum Scroll
Terminal-Output scrollt mit Schwung nach Loslassen.
`momentum_factor`: 0.0 = sofort stopp, 1.0 = sehr langer Auslauf.

### Snap-to-Edge
Fenster rastet an Bildschirmkanten ein.
Optional mit weicher Animation (`snap_animation`) oder sofort.
Threshold in Pixeln konfigurierbar.

### Audio-Feedback
Subtiler Click-Sound beim Snap-to-Edge.
Ton beim Tab-Wechsel.
Via `rodio`-crate.

---

## Augensteuerung (Eye Tracking)

### Konzept
Externe Eye-Tracker Tools steuern 3DNTerminal über IPC Socket.
Kein eigener Eye-Tracking Code in 3DNTerminal nötig.

### IPC Integration
```
Blick auf Terminal → { "action": "set_state", "value": "expanded" }
Blick weg          → { "action": "set_state", "value": "collapsed" }
Dwell auf Button   → { "action": "toggle" }
```

---

## Hologramm-verknüpfte Physik (Phase 6 Erweiterung)

### Adaptive Tilt / Billboarding
Aktuell: Text schrumpft bei Kippen.
Neu: Text lehnt sich gegen die Neigung → immer maximal lesbar.
**Priorität 1 — höchster A11Y-Impact**

### Magnetic Cursor / Button-Fokus
Buttons strecken sich dem Cursor entgegen (Z-Achse) wenn Cursor in Nähe.
Fitts's Law direkt implementiert — kleine Ziele werden größer wahrgenommen.
**Priorität 1 — höchster A11Y-Impact**

### Spring Physics / Nachfedern
Ersetzt feste 600ms-Animation durch Feder-Physik (Stiffness/Damping).

### Fokus-Ring als Quad-Outline
WCAG-Pflicht — sichtbarer Fokus-Indikator.
Dicker im A11Y-Mode (4px+), hoher Kontrast.
**Priorität 1 — höchster A11Y-Impact**
