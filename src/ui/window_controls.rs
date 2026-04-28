use cosmic::iced::{Point, Color, widget::canvas::{Frame, Path, Stroke}, Pixels, Size};
use crate::app::state::CornerPosition;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonAction {
    Minimize,
    Maximize,
    Close,
    SetCorner(CornerPosition),
    Hamburger,
    NewTab,
}

pub struct WindowControls;

impl WindowControls {
    pub fn new() -> Self {
        Self
    }

    /// Berechnet alle Button-Positionen basierend auf zwei Ankerpunkten.
    fn button_positions(left_anchor: Point, right_anchor: Point, btn_size: f32) -> Vec<(f32, f32, &'static str, ButtonAction)> {
        let gap = 12.0;
        let sep = 24.0;
        let mut btns = Vec::new();

        // --- Linke Gruppe (ab left_anchor) ---
        let lx = left_anchor.x + gap;
        let ly = left_anchor.y + 8.0; // Leichtes Offset nach unten
        btns.push((lx, ly, "☰", ButtonAction::Hamburger));
        btns.push((lx + btn_size + gap, ly, "+", ButtonAction::NewTab));

        // --- Rechte Gruppe (ab right_anchor) ---
        let ry = right_anchor.y + 8.0;
        let close_x    = right_anchor.x - btn_size - gap;
        let max_x      = right_anchor.x - btn_size * 2.0 - gap * 2.0;
        let min_x      = right_anchor.x - btn_size * 3.0 - gap * 3.0;
        
        let br_x       = right_anchor.x - btn_size * 4.0 - gap * 3.0 - sep;
        let bl_x       = right_anchor.x - btn_size * 5.0 - gap * 4.0 - sep;
        let tr_x       = right_anchor.x - btn_size * 6.0 - gap * 5.0 - sep;
        let tl_x       = right_anchor.x - btn_size * 7.0 - gap * 6.0 - sep;

        btns.push((close_x,   ry, "×", ButtonAction::Close));
        btns.push((max_x,     ry, "□", ButtonAction::Maximize));
        btns.push((min_x,     ry, "−", ButtonAction::Minimize));
        btns.push((br_x,      ry, "↘", ButtonAction::SetCorner(CornerPosition::BottomRight)));
        btns.push((bl_x,      ry, "↙", ButtonAction::SetCorner(CornerPosition::BottomLeft)));
        btns.push((tr_x,      ry, "↗", ButtonAction::SetCorner(CornerPosition::TopRight)));
        btns.push((tl_x,      ry, "↖", ButtonAction::SetCorner(CornerPosition::TopLeft)));

        btns
    }

    pub fn draw(&self, frame: &mut Frame, alpha: f32, left_anchor: Point, right_anchor: Point, btn_size: f32, cursor_pos: Point) {
        if alpha <= 0.0 { return; }

        let cyan = Color::from_rgba(0.4, 1.0, 0.8, alpha);
        let hover_bg = Color::from_rgba(0.4, 1.0, 0.8, alpha * 0.4);

        for (bx, by, icon, _action) in Self::button_positions(left_anchor, right_anchor, btn_size) {
            let path = Path::rectangle(Point::new(bx, by), Size::new(btn_size, btn_size));

            let margin = 2.0;
            let hit = cursor_pos.x >= bx - margin && cursor_pos.x <= bx + btn_size + margin
                   && cursor_pos.y >= by - margin && cursor_pos.y <= by + btn_size + margin;

            if hit {
                frame.fill(&path, hover_bg);
            }

            frame.stroke(&path, Stroke::default()
                .with_color(cyan)
                .with_width(1.2));

            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: icon.to_string(),
                position: Point::new(bx + btn_size * 0.5, by + btn_size * 0.5),
                color: cyan,
                size: Pixels(btn_size * 0.7),
                align_x: cosmic::iced::alignment::Horizontal::Center.into(),
                align_y: cosmic::iced::alignment::Vertical::Center.into(),
                ..Default::default()
            });
        }
    }

    pub fn hit_test(&self, click: Point, left_anchor: Point, right_anchor: Point, btn_size: f32) -> Option<ButtonAction> {
        for (bx, by, _, action) in Self::button_positions(left_anchor, right_anchor, btn_size) {
            let margin = 12.0; // Verdoppelter Puffer für Magnetismus-Kompensation
            let hit = click.x >= bx - margin && click.x <= bx + btn_size + margin
                   && click.y >= by - margin && click.y <= by + btn_size + margin;

            if hit {
                tracing::debug!("Hit-Test: Success for {:?} at {:?}", action, click);
                return Some(action);
            }
        }
        None
    }
}
