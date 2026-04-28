use cosmic::iced::{Point, Color, widget::canvas::{Frame, Path, Stroke}, Pixels, Size};
use crate::app::state::CornerPosition;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonAction {
    Minimize,
    Maximize,
    Close,
    RestoreLast,
    SetCorner(CornerPosition),
}

pub struct WindowControls;

impl WindowControls {
    pub fn new() -> Self {
        Self
    }

    // Button-Positionen: rechts-verankert an anchor (p2), von rechts nach links:
    // ×  □  −  [sep]  ↩  [sep]  ↘  ↙  ↗  ↖
    fn button_positions(anchor: Point, btn_size: f32) -> [(f32, &'static str, ButtonAction); 8] {
        let gap = 12.0; // Fester, größerer Abstand
        let sep = 24.0; // Deutlicher Trenner

        let close_x    = anchor.x - btn_size;
        let max_x      = anchor.x - btn_size * 2.0 - gap;
        let min_x      = anchor.x - btn_size * 3.0 - gap * 2.0;
        let restore_x  = anchor.x - btn_size * 4.0 - gap * 2.0 - sep;
        let br_x       = anchor.x - btn_size * 5.0 - gap * 2.0 - sep * 2.0;
        let bl_x       = anchor.x - btn_size * 6.0 - gap * 3.0 - sep * 2.0;
        let tr_x       = anchor.x - btn_size * 7.0 - gap * 4.0 - sep * 2.0;
        let tl_x       = anchor.x - btn_size * 8.0 - gap * 5.0 - sep * 2.0;

        [
            (close_x,   "×",  ButtonAction::Close),
            (max_x,     "□",  ButtonAction::Maximize),
            (min_x,     "−",  ButtonAction::Minimize),
            (restore_x, "↩",  ButtonAction::RestoreLast),
            (br_x,      "↘",  ButtonAction::SetCorner(CornerPosition::BottomRight)),
            (bl_x,      "↙",  ButtonAction::SetCorner(CornerPosition::BottomLeft)),
            (tr_x,      "↗",  ButtonAction::SetCorner(CornerPosition::TopRight)),
            (tl_x,      "↖",  ButtonAction::SetCorner(CornerPosition::TopLeft)),
        ]
    }

    pub fn draw(&self, frame: &mut Frame, alpha: f32, anchor: Point, btn_size: f32, active_corner: CornerPosition, cursor_pos: Point) {
        if alpha <= 0.0 { return; }

        let by = anchor.y + btn_size * 1.2;
        let cyan = Color::from_rgba(0.4, 1.0, 0.8, alpha);
        let hover_bg = Color::from_rgba(0.4, 1.0, 0.8, alpha * 0.4);

        for (bx, icon, action) in Self::button_positions(anchor, btn_size) {
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

    pub fn hit_test(&self, click: Point, anchor: Point, btn_size: f32) -> Option<ButtonAction> {
        let by = anchor.y + btn_size * 1.2;

        tracing::debug!(
            "hit_test: click=({:.1},{:.1}) anchor=({:.1},{:.1}) btn_size={:.1}",
            click.x, click.y, anchor.x, anchor.y, btn_size
        );

        for (bx, _, action) in Self::button_positions(anchor, btn_size) {
            let margin = 2.0; // Minimaler Puffer, um Überlappungen zu vermeiden
            let hit = click.x >= bx - margin && click.x <= bx + btn_size + margin
                   && click.y >= by - margin && click.y <= by + btn_size + margin;

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
