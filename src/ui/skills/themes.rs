use cosmic::iced::{Color, Rectangle, Point, Size, Pixels};
use cosmic::iced::widget::canvas::{Frame, Text};
use crate::ui::two_d::TerminalParams;
use crate::config::{Config, TerminalTheme};
use crate::ui::skill::TerminalSkill;

pub struct ThemesSkill;

impl TerminalSkill for ThemesSkill {
    fn id(&self) -> &'static str { "themes" }
    fn label(&self) -> &'static str { "Themes" }
    fn subtitle(&self) -> &'static str { "System Styles & Presets" }
    fn color(&self) -> Color { Color::from_rgba(0.0, 1.0, 0.8, 1.0) }

    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
        let y = rect.y + 80.0;
        frame.fill_text(Text {
            content: "SELECT SYSTEM STYLE".to_string(),
            position: Point::new(rect.x + 20.0, y),
            color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
            size: Pixels(18.0),
            ..Default::default()
        });
        
        let themes = [
            ("BladeRunner", TerminalTheme::BladeRunner),
            ("Apple Glass", TerminalTheme::AppleGlass),
            ("Deep Space", TerminalTheme::DeepSpace),
            ("Retro Amber", TerminalTheme::RetroAmber),
            ("Neon Cyber", TerminalTheme::NeonCyber),
        ];

        for (i, (name, theme)) in themes.iter().enumerate() {
            let chip_x = rect.x + 20.0 + (i as f32 * 105.0);
            let chip_y = y + 40.0;
            frame.fill_rectangle(Point::new(chip_x, chip_y), Size::new(95.0, 30.0), theme.color());
            frame.fill_text(Text {
                content: name.to_string(),
                position: Point::new(chip_x, chip_y + 45.0),
                color: Color::from_rgba(0.7, 0.7, 0.7, alpha),
                size: Pixels(11.0),
                ..Default::default()
            });
        }

        // --- POWER USER TOGGLE ---
        let toggle_y = rect.y + 220.0;
        let is_power = params.config.power_user_mode;
        let btn_color = if is_power { Color::from_rgba(0.0, 1.0, 0.8, alpha) } else { Color::from_rgba(0.3, 0.3, 0.3, alpha) };
        
        frame.fill_rectangle(Point::new(rect.x + 20.0, toggle_y), Size::new(200.0, 40.0), btn_color);
        frame.fill_text(Text {
            content: if is_power { "[ MODE: POWER USER ]" } else { "[ MODE: STANDARD ]" }.to_string(),
            position: Point::new(rect.x + 40.0, toggle_y + 12.0),
            color: if is_power { Color::BLACK } else { Color::WHITE },
            size: Pixels(14.0),
            ..Default::default()
        });

        if is_power {
            frame.fill_text(Text {
                content: "Note: Accessibility menu is hidden in Power User Mode.".to_string(),
                position: Point::new(rect.x + 20.0, toggle_y + 55.0),
                color: Color::from_rgba(1.0, 0.5, 0.0, alpha), // Warning Orange
                size: Pixels(11.0),
                ..Default::default()
            });
        }

        frame.fill_text(Text {
            content: "Note: Custom Environment Sliders are currently in planning.".to_string(),
            position: Point::new(rect.x + 20.0, rect.y + rect.height - 40.0),
            color: Color::from_rgba(0.4, 0.4, 0.4, alpha),
            size: Pixels(10.0),
            ..Default::default()
        });
    }

    fn on_click(&self, pos: Point, rect: Rectangle, config: &mut Config) -> bool {
        // 1. Check Theme Chips
        let y_start = rect.y + 120.0;
        let themes = [
            TerminalTheme::BladeRunner,
            TerminalTheme::AppleGlass,
            TerminalTheme::DeepSpace,
            TerminalTheme::RetroAmber,
            TerminalTheme::NeonCyber,
        ];

        for (i, theme) in themes.iter().enumerate() {
            let chip_x = rect.x + 20.0 + (i as f32 * 105.0);
            let chip_rect = Rectangle::new(Point::new(chip_x, y_start), Size::new(95.0, 30.0));
            if chip_rect.contains(pos) {
                config.theme = *theme;
                config.neon_color = theme.color();
                
                config.visuals = crate::config::VisualsConfig::default();
                match theme {
                    TerminalTheme::BladeRunner => config.visuals.rain_intensity = 1.0,
                    TerminalTheme::RetroAmber => config.visuals.scanline_opacity = 0.5,
                    TerminalTheme::DeepSpace => config.visuals.star_density = 1.0,
                    TerminalTheme::NeonCyber => config.visuals.grid_opacity = 0.2,
                    _ => {}
                }
                return true;
            }
        }

        // 2. Check Power User Toggle
        let toggle_y = rect.y + 220.0;
        let toggle_rect = Rectangle::new(Point::new(rect.x + 20.0, toggle_y), Size::new(200.0, 40.0));
        if toggle_rect.contains(pos) {
            config.power_user_mode = !config.power_user_mode;
            return true;
        }

        false
    }
}
