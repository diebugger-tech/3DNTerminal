use cosmic::iced::{Point, Color, widget::canvas::{Frame}, Pixels, Size, Rectangle};
use crate::app::state::CornerPosition;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonAction {
    Minimize,
    Maximize,
    Close,
    SetCorner(CornerPosition),
    Hamburger,
    NewTab,
    SaveSize,
    Resize,
}

pub struct WindowControls;

impl WindowControls {
    pub fn new() -> Self {
        Self
    }

    /// Berechnet alle Button-Positionen basierend auf zwei Ankerpunkten.
    fn button_positions(_left_anchor: Point, right_anchor: Point, btn_size: f32) -> Vec<(f32, f32, &'static str, ButtonAction)> {
        let gap = 4.0;
        let sep = 16.0;
        let group_sep = 14.0;
        let mut btns = Vec::new();

        // --- Linke Gruppe (ab left_anchor) ---
        let lx = _left_anchor.x + 12.0;
        let ly = _left_anchor.y + 8.0; 
        btns.push((lx, ly, "☰", ButtonAction::Hamburger));
        btns.push((lx + btn_size + 20.0, ly, "+", ButtonAction::NewTab));

        // --- Rechte Gruppe (ab right_anchor) ---
        let ry = right_anchor.y + 8.0;
        
        // System-Gruppe
        let close_x    = right_anchor.x - btn_size * 1.0 - gap * 0.0 - sep;
        let max_x      = right_anchor.x - btn_size * 2.0 - gap * 1.0 - sep;
        let min_x      = right_anchor.x - btn_size * 3.0 - gap * 2.0 - sep;
        
        // Corner-Gruppe (Pfeile)
        let br_x       = right_anchor.x - btn_size * 4.0 - gap * 3.0 - sep - group_sep;
        let bl_x       = right_anchor.x - btn_size * 5.0 - gap * 4.0 - sep - group_sep;
        let tr_x       = right_anchor.x - btn_size * 6.0 - gap * 5.0 - sep - group_sep;
        let tl_x       = right_anchor.x - btn_size * 7.0 - gap * 6.0 - sep - group_sep;
        
        // Utility-Gruppe (Anchor & Resize)
        let anchor_x   = right_anchor.x - btn_size * 8.0 - gap * 7.0 - sep - group_sep * 2.0;
        let resize_x   = right_anchor.x - btn_size * 9.0 - gap * 8.0 - sep - group_sep * 2.0;

        btns.push((close_x,   ry, "×", ButtonAction::Close));
        btns.push((max_x,     ry, "□", ButtonAction::Maximize));
        btns.push((min_x,     ry, "−", ButtonAction::Minimize));
        btns.push((br_x,      ry, "↘", ButtonAction::SetCorner(CornerPosition::BottomRight)));
        btns.push((bl_x,      ry, "↙", ButtonAction::SetCorner(CornerPosition::BottomLeft)));
        btns.push((tr_x,      ry, "↗", ButtonAction::SetCorner(CornerPosition::TopRight)));
        btns.push((tl_x,      ry, "↖", ButtonAction::SetCorner(CornerPosition::TopLeft)));
        btns.push((anchor_x,  ry, "⚓", ButtonAction::SaveSize));
        btns.push((resize_x,  ry, "⤢", ButtonAction::Resize));

        btns
    }

    pub fn draw(&self, frame: &mut Frame, alpha: f32, left_anchor: Point, right_anchor: Point, btn_size: f32, cursor_pos: Point) {
        for (bx, by, label, action) in Self::button_positions(left_anchor, right_anchor, btn_size) {
            let rect = Rectangle::new(Point::new(bx, by), Size::new(btn_size, btn_size));
            let is_hovered = self.hit_test(cursor_pos, left_anchor, right_anchor, btn_size) == Some(action);
            
            let bg_color = if is_hovered {
                Color::from_rgba(1.0, 1.0, 1.0, 0.15 * alpha)
            } else {
                Color::from_rgba(1.0, 1.0, 1.0, 0.05 * alpha)
            };
            
            frame.fill_rectangle(rect.position(), rect.size(), bg_color);
            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: label.to_string(),
                position: Point::new(bx + btn_size / 2.0, by + btn_size / 2.0),
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.8 * alpha),
                size: Pixels(16.0),
                align_x: cosmic::iced::alignment::Horizontal::Center.into(),
                align_y: cosmic::iced::alignment::Vertical::Center.into(),
                ..Default::default()
            });
        }
    }

    pub fn hit_test(&self, click: Point, left_anchor: Point, right_anchor: Point, btn_size: f32) -> Option<ButtonAction> {
        for (bx, by, _, action) in Self::button_positions(left_anchor, right_anchor, btn_size) {
            let margin = 10.0; // Erhöht für bessere Trefferrate
            let hit = click.x >= bx - margin && click.x <= bx + btn_size + margin
                   && click.y >= by - margin && click.y <= by + btn_size + margin;
            
            if hit {
                return Some(action);
            }
        }
        None
    }
}
