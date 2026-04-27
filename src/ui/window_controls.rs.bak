use cosmic::iced::{Point, Color, widget::canvas::{Frame, Path, Stroke}, Pixels, Size};
use crate::app::state::CornerPosition;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonAction {
    Minimize,
    Maximize,
    Close,
    SetCorner(CornerPosition),
}

pub struct WindowControls;

impl WindowControls {
    pub fn new() -> Self {
        Self
    }

    // Button-Positionen: rechts-verankert an anchor (p2), von rechts nach links:
    // ×  □  −  [sep]  ↘  ↙  ↗  ↖
    fn button_positions(anchor: Point, btn_size: f32) -> [(f32, &'static str, ButtonAction); 7] {
        let gap = btn_size * 0.3;
        let sep = btn_size * 0.8; // extra Abstand zwischen Corner-Buttons und Window-Controls

        let close_x    = anchor.x - btn_size;
        let max_x      = anchor.x - btn_size * 2.0 - gap;
        let min_x      = anchor.x - btn_size * 3.0 - gap * 2.0;
        let br_x       = anchor.x - btn_size * 4.0 - gap * 2.0 - sep;
        let bl_x       = anchor.x - btn_size * 5.0 - gap * 3.0 - sep;
        let tr_x       = anchor.x - btn_size * 6.0 - gap * 4.0 - sep;
        let tl_x       = anchor.x - btn_size * 7.0 - gap * 5.0 - sep;

        [
            (close_x, "×",  ButtonAction::Close),
            (max_x,   "□",  ButtonAction::Maximize),
            (min_x,   "−",  ButtonAction::Minimize),
            (br_x,    "↘",  ButtonAction::SetCorner(CornerPosition::BottomRight)),
            (bl_x,    "↙",  ButtonAction::SetCorner(CornerPosition::BottomLeft)),
            (tr_x,    "↗",  ButtonAction::SetCorner(CornerPosition::TopRight)),
            (tl_x,    "↖",  ButtonAction::SetCorner(CornerPosition::TopLeft)),
        ]
    }

    pub fn draw(&self, frame: &mut Frame, alpha: f32, anchor: Point, btn_size: f32, active_corner: CornerPosition) {
        if alpha <= 0.0 { return; }

        let by = anchor.y + btn_size * 1.2;
        let cyan = Color::from_rgba(0.4, 1.0, 0.8, alpha);
        let cyan_fill = Color::from_rgba(0.4, 1.0, 0.8, alpha * 0.25);

        for (bx, icon, action) in Self::button_positions(anchor, btn_size) {
            let path = Path::rectangle(Point::new(bx, by), Size::new(btn_size, btn_size));

            // Aktive Ecke: gefüllter Hintergrund
            let is_active_corner = matches!(action, ButtonAction::SetCorner(pos) if pos == active_corner);
            if is_active_corner {
                frame.fill(&path, cyan_fill);
            }

            frame.stroke(&path, Stroke::default()
                .with_color(cyan)
                .with_width(1.0));

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

    pub fn hit_test(&self, click: Point, anchor: Point, btn_size: f32) -> Option<ButtonAction> {
        let by = anchor.y + btn_size * 1.2;

        tracing::debug!(
            "hit_test: click=({:.1},{:.1}) anchor=({:.1},{:.1}) btn_size={:.1}",
            click.x, click.y, anchor.x, anchor.y, btn_size
        );

        for (bx, _, action) in Self::button_positions(anchor, btn_size) {
            let hit = click.x >= bx && click.x <= bx + btn_size
                   && click.y >= by && click.y <= by + btn_size;

            tracing::debug!(
                "  {:?}: rect ({:.1},{:.1})→({:.1},{:.1}) hit={}",
                action, bx, by, bx + btn_size, by + btn_size, hit
            );

            if hit {
                return Some(action);
            }
        }
        None
    }
}
