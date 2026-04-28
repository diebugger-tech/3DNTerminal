use cosmic::iced::{Rectangle, Color, Point, Size, Pixels};
use cosmic::iced::widget::canvas::{Frame, Path, Stroke, Text};
use crate::ui::two_d::TerminalParams;

pub fn draw(frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
    if let Some((text, start)) = params.notification {
        let elapsed = start.elapsed().as_secs_f32();
        if elapsed < 3.0 {
            let toast_alpha = if elapsed > 2.5 { (3.0 - elapsed) * 2.0 } else { 1.0 } * alpha;
            let toast_w = 320.0;
            let toast_h = 50.0;
            let toast_rect = Rectangle {
                x: rect.x + rect.width - toast_w - 20.0,
                y: rect.y + rect.height - toast_h - 20.0,
                width: toast_w,
                height: toast_h,
            };

            let toast_path = Path::rectangle(Point::new(toast_rect.x, toast_rect.y), Size::new(toast_rect.width, toast_rect.height));
            frame.fill(&toast_path, Color::from_rgba(0.02, 0.02, 0.1, 0.9 * toast_alpha));
            frame.stroke(&toast_path, Stroke::default().with_color(Color::from_rgba(1.0, 0.6, 0.0, 0.6 * toast_alpha)).with_width(2.0));

            frame.fill_text(Text {
                content: text.clone(),
                position: Point::new(toast_rect.x + 20.0, toast_rect.y + 15.0),
                color: Color::from_rgba(1.0, 0.7, 0.2, toast_alpha),
                size: Pixels(16.0),
                ..Default::default()
            });
        }
    }
}
