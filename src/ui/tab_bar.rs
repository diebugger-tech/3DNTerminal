use cosmic::iced::{Rectangle, Color, Point, Size, Pixels};
use cosmic::iced::widget::canvas::{Frame, Path, Text};
use crate::ui::two_d::TerminalParams;

pub fn draw(frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
    let tab_start_x = rect.x + 100.0;
    
    for (i, tab) in params.tabs.iter().enumerate() {
        let is_active = i == params.active_tab;
        let tab_x = tab_start_x + (i as f32 * 110.0);
        let tab_rect = Rectangle {
            x: tab_x,
            y: rect.y + 12.0,
            width: 100.0,
            height: 26.0,
        };
        
        let tab_path = Path::rectangle(Point::new(tab_rect.x, tab_rect.y), Size::new(tab_rect.width, tab_rect.height));
        if is_active {
            frame.fill(&tab_path, Color::from_rgba(1.0, 0.6, 0.0, 0.2 * alpha));
            frame.stroke(&tab_path, cosmic::iced::widget::canvas::Stroke::default()
                .with_color(Color::from_rgba(1.0, 0.6, 0.0, 0.6 * alpha))
                .with_width(1.0));
        }
        
        frame.fill_text(Text {
            content: tab.clone(),
            position: Point::new(tab_rect.x + 10.0, tab_rect.y + 18.0),
            color: Color::from_rgba(1.0, 0.8, 0.4, if is_active { 1.0 } else { 0.5 } * alpha),
            size: Pixels(13.0),
            ..Default::default()
        });
    }
}
