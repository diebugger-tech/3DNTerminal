use cosmic::iced::{Color, Rectangle, Point, Size, Pixels};
use cosmic::iced::widget::canvas::{Frame, Text};
use crate::ui::two_d::TerminalParams;
use crate::config::{Config, TerminalTheme};
use crate::ui::skill::TerminalSkill;

pub struct ThemesSkill;

impl TerminalSkill for ThemesSkill {
    fn id(&self) -> &'static str { "themes" }
    fn label(&self) -> &'static str { "Themes" }
    fn subtitle(&self) -> &'static str { "Amber / Magenta / Cobalt" }
    fn color(&self) -> Color { Color::from_rgba(0.0, 1.0, 0.8, 1.0) }

    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, _params: &TerminalParams) {
        let y = rect.y + 80.0;
        frame.fill_text(Text {
            content: "SELECT SYSTEM COLOR".to_string(),
            position: Point::new(rect.x + 20.0, y),
            color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
            size: Pixels(18.0),
            ..Default::default()
        });
        
        for (i, (name, theme)) in [
            ("Amber", TerminalTheme::Amber),
            ("Magenta", TerminalTheme::Magenta),
            ("Cyan", TerminalTheme::Cyan),
            ("Green", TerminalTheme::Green),
        ].iter().enumerate() {
            let chip_x = rect.x + 20.0 + (i as f32 * 90.0);
            let chip_y = y + 40.0;
            frame.fill_rectangle(Point::new(chip_x, chip_y), Size::new(80.0, 30.0), theme.color());
            frame.fill_text(Text {
                content: name.to_string(),
                position: Point::new(chip_x, chip_y + 45.0),
                color: Color::from_rgba(0.7, 0.7, 0.7, alpha),
                size: Pixels(12.0),
                ..Default::default()
            });
        }
    }

    fn on_click(&self, pos: Point, rect: Rectangle, config: &mut Config) -> bool {
        let y_start = rect.y + 120.0;
        let themes = [
            TerminalTheme::Amber,
            TerminalTheme::Magenta,
            TerminalTheme::Cyan,
            TerminalTheme::Green,
        ];

        for (i, theme) in themes.iter().enumerate() {
            let chip_x = rect.x + 20.0 + (i as f32 * 90.0);
            let chip_rect = Rectangle::new(Point::new(chip_x, y_start), Size::new(80.0, 30.0));
            if chip_rect.contains(pos) {
                tracing::info!("ThemesSkill: Selected theme {:?}", theme);
                config.theme = *theme;
                config.neon_color = theme.color();
                return true;
            }
        }
        false
    }
}
