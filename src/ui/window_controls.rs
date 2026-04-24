use cosmic::iced::{Point, Color, widget::canvas::{Frame, Path, Stroke}, Pixels, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonAction {
    Minimize,
    Maximize,
    Close,
}

pub struct WindowControls;

impl WindowControls {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, frame: &mut Frame, alpha: f32, anchor: Point, btn_size: f32) {
        if alpha <= 0.0 { return; }
        let gap = btn_size * 0.3;
        
        // anchor = p2 (top-right des Quads)
        // Buttons links von anchor, von rechts nach links: Close, Maximize, Minimize
        let buttons = [
            (anchor.x - btn_size,                     "×"),
            (anchor.x - btn_size * 2.0 - gap,         "□"),
            (anchor.x - btn_size * 3.0 - gap * 2.0,   "−"),
        ];

        for (bx, icon) in buttons {
            let by = anchor.y + btn_size * 1.2; // leicht unter p2
            let path = Path::rectangle(
                Point::new(bx, by),
                Size::new(btn_size, btn_size),
            );
            frame.stroke(&path, Stroke::default()
                .with_color(Color::from_rgba(0.4, 1.0, 0.8, alpha))
                .with_width(1.0));
            
            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: icon.to_string(),
                position: Point::new(bx + btn_size * 0.5, by + btn_size * 0.5),
                color: Color::from_rgba(0.4, 1.0, 0.8, alpha),
                size: Pixels(btn_size * 0.7),
                align_x: cosmic::iced::alignment::Horizontal::Center.into(),
                align_y: cosmic::iced::alignment::Vertical::Center.into(),
                ..Default::default()
            });
        }
    }

    pub fn hit_test(&self, click: Point, anchor: Point, btn_size: f32) -> Option<ButtonAction> {
        let gap = btn_size * 0.3;
        let by = anchor.y + btn_size * 1.2;
        let buttons = [
            (anchor.x - btn_size,               ButtonAction::Close),
            (anchor.x - btn_size * 2.0 - gap,   ButtonAction::Maximize),
            (anchor.x - btn_size * 3.0 - gap * 2.0, ButtonAction::Minimize),
        ];

        tracing::debug!(
            "hit_test: click=({:.1},{:.1}) anchor=({:.1},{:.1}) btn_size={:.1}",
            click.x, click.y, anchor.x, anchor.y, btn_size
        );

        for (bx, action) in buttons {
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
