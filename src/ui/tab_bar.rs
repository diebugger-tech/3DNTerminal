use cosmic::iced::{Rectangle, Color, Point, Size, Pixels};
use cosmic::iced::widget::canvas::{Frame, Path, Text};
use crate::ui::two_d::TerminalParams;

pub fn draw(frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
    let tab_start_x = rect.x + 100.0;
    
    for (i, tab) in params.tabs.iter().enumerate() {
        let tab_w = 100.0;
        let tab_x = tab_start_x + (i as f32 * (tab_w + 5.0));
        let tab_rect = Rectangle {
            x: tab_x,
            y: rect.y + 8.0,
            width: tab_w,
            height: 25.0,
        };

        let tab_color = if i == params.active_tab {
            params.neon_color
        } else {
            Color::from_rgba(0.5, 0.5, 0.5, 0.5 * alpha)
        };

        // Tab Background & Border
        frame.fill_rectangle(tab_rect.position(), tab_rect.size(), Color::from_rgba(tab_color.r, tab_color.g, tab_color.b, 0.1 * alpha));
        frame.stroke(&Path::rectangle(tab_rect.position(), tab_rect.size()), cosmic::iced::widget::canvas::Stroke::default().with_color(tab_color).with_width(1.0));
        
        // Tab Title
        frame.fill_text(Text {
            content: tab.to_string(),
            position: Point::new(tab_x + 10.0, rect.y + 12.0),
            color: tab_color,
            size: Pixels(13.0),
            ..Default::default()
        });

        // Close button [x]
        if params.tabs.len() > 1 {
            frame.fill_text(Text {
                content: "x".to_string(),
                position: Point::new(tab_x + tab_w - 18.0, rect.y + 10.0),
                color: if i == params.active_tab { Color::from_rgba(1.0, 0.2, 0.2, alpha) } else { Color::from_rgba(0.5, 0.2, 0.2, 0.3 * alpha) },
                size: Pixels(10.0),
                ..Default::default()
            });
        }
    }
}
