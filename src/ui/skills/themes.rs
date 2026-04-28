use cosmic::iced::{Color, Rectangle, Point, Size, Pixels};
use cosmic::iced::widget::canvas::{Frame, Text, Path, Stroke};
use crate::ui::two_d::TerminalParams;
use crate::config::{Config, TerminalTheme};
use crate::ui::skill::TerminalSkill;

pub struct ThemesSkill;

impl ThemesSkill {
    fn draw_switch(&self, frame: &mut Frame, x: f32, y: f32, label: &str, active: bool, alpha: f32) {
        let color = if active { Color::from_rgba(0.0, 1.0, 0.8, alpha) } else { Color::from_rgba(0.3, 0.3, 0.3, alpha) };
        let rect = Rectangle::new(Point::new(x, y), Size::new(20.0, 20.0));
        
        frame.fill_rectangle(rect.position(), rect.size(), color);
        if active {
            // Draw a small checkmark or dot
            frame.fill_rectangle(Point::new(x + 5.0, y + 5.0), Size::new(10.0, 10.0), Color::BLACK);
        }

        frame.fill_text(Text {
            content: label.to_string(),
            position: Point::new(x + 30.0, y + 2.0),
            color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
            size: Pixels(13.0),
            ..Default::default()
        });
    }

    fn draw_slider(&self, frame: &mut Frame, x: f32, y: f32, w: f32, label: &str, value: f32, alpha: f32) {
        let label_color = Color::from_rgba(0.7, 0.7, 0.7, alpha);
        let track_color = Color::from_rgba(0.2, 0.2, 0.2, alpha);
        let thumb_color = Color::from_rgba(0.0, 1.0, 0.8, alpha);

        // Label
        frame.fill_text(Text {
            content: label.to_string(),
            position: Point::new(x, y - 12.0),
            color: label_color,
            size: Pixels(11.0),
            ..Default::default()
        });

        // Track
        let track_path = Path::line(Point::new(x, y + 10.0), Point::new(x + w, y + 10.0));
        frame.stroke(&track_path, Stroke::default().with_color(track_color).with_width(4.0));

        // Thumb
        let thumb_x = x + (value * w);
        frame.fill_rectangle(Point::new(thumb_x - 4.0, y), Size::new(8.0, 20.0), thumb_color);
        
        // Value Text
        frame.fill_text(Text {
            content: format!("{:.0}%", value * 100.0),
            position: Point::new(x + w + 10.0, y + 4.0),
            color: label_color,
            size: Pixels(10.0),
            ..Default::default()
        });
    }

    fn check_slider(&self, pos: Point, x: f32, y: f32, w: f32, value: &mut f32) -> bool {
        let hit_rect = Rectangle::new(Point::new(x, y - 10.0), Size::new(w, 30.0));
        if hit_rect.contains(pos) {
            let new_val = ((pos.x - x) / w).clamp(0.0, 1.0);
            *value = new_val;
            return true;
        }
        false
    }
}

impl TerminalSkill for ThemesSkill {
    fn id(&self) -> &'static str { "themes" }
    fn label(&self) -> &'static str { "Themes" }
    fn subtitle(&self) -> &'static str { "Style Engine & Environment" }
    fn color(&self) -> Color { Color::from_rgba(0.0, 1.0, 0.8, 1.0) }

    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
        let is_power = params.config.power_user_mode;

        // 1. Header
        let y_start = rect.y + 70.0;
        frame.fill_text(Text {
            content: "THEME ENGINE & STYLE INTENSITY".to_string(),
            position: Point::new(rect.x + 20.0, y_start),
            color: Color::from_rgba(0.0, 1.0, 0.8, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });
        
        let themes = [
            ("Classic (Matrix)", TerminalTheme::Classic),
            ("Neon Cyber", TerminalTheme::NeonCyber),
            ("Apple Glass", TerminalTheme::AppleGlass),
            ("Deep Space", TerminalTheme::DeepSpace),
            ("Retro Amber", TerminalTheme::RetroAmber),
            ("Blade Runner", TerminalTheme::BladeRunner),
        ];

        let mut current_y = y_start + 40.0;
        for (i, (name, theme)) in themes.iter().enumerate() {
            let is_active = params.config.theme == *theme;
            let intensity = params.config.theme_intensities[i];
            
            // Row Background
            if is_active {
                frame.fill_rectangle(
                    Point::new(rect.x + 15.0, current_y - 5.0),
                    Size::new(rect.width - 30.0, 45.0),
                    Color::from_rgba(0.0, 1.0, 0.8, 0.05 * alpha)
                );
            }

            // A. Switch
            self.draw_switch(frame, rect.x + 25.0, current_y + 8.0, "", is_active, alpha);

            // B. Color Chip
            frame.fill_rectangle(
                Point::new(rect.x + 65.0, current_y + 8.0),
                Size::new(12.0, 12.0),
                theme.color()
            );

            // C. Name
            frame.fill_text(Text {
                content: name.to_string(),
                position: Point::new(rect.x + 85.0, current_y + 8.0),
                color: if is_active { Color::WHITE } else { Color::from_rgba(0.6, 0.6, 0.6, alpha) },
                size: Pixels(13.0),
                ..Default::default()
            });

            // D. Regler (Slider)
            let slider_x = rect.x + 220.0;
            let slider_w = rect.width - 280.0;
            self.draw_slider(frame, slider_x, current_y + 10.0, slider_w, "Intensity", intensity, alpha);

            current_y += 50.0;
        }

        // 4. Power User Toggle
        let power_y = rect.y + rect.height - 60.0;
        let btn_color = if is_power { Color::from_rgba(0.0, 1.0, 0.8, alpha) } else { Color::from_rgba(0.2, 0.2, 0.2, alpha) };
        frame.fill_rectangle(Point::new(rect.x + 20.0, power_y), Size::new(200.0, 35.0), btn_color);
        frame.fill_text(Text {
            content: if is_power { "[ MODE: POWER USER ]" } else { "[ MODE: STANDARD ]" }.to_string(),
            position: Point::new(rect.x + 45.0, power_y + 10.0),
            color: if is_power { Color::BLACK } else { Color::WHITE },
            size: Pixels(13.0),
            ..Default::default()
        });
    }

    fn on_click(&self, pos: Point, rect: Rectangle, config: &mut Config) -> bool {
        let y_start = rect.y + 70.0;
        let mut current_y = y_start + 40.0;
        
        let themes = [
            TerminalTheme::Classic,
            TerminalTheme::NeonCyber,
            TerminalTheme::AppleGlass,
            TerminalTheme::DeepSpace,
            TerminalTheme::RetroAmber,
            TerminalTheme::BladeRunner,
        ];

        for (i, theme) in themes.iter().enumerate() {
            // Check for Switch Click
            let switch_rect = Rectangle::new(Point::new(rect.x + 25.0, current_y + 8.0), Size::new(25.0, 25.0));
            if switch_rect.contains(pos) {
                config.theme = *theme;
                config.neon_color = theme.color();
                // Apply current theme's intensity to the active visuals
                config.visuals.glow_intensity = config.theme_intensities[i] * theme.glow_intensity();
                return true;
            }

            // Check for Slider Click
            let slider_x = rect.x + 220.0;
            let slider_w = rect.width - 280.0;
            if self.check_slider(pos, slider_x, current_y + 10.0, slider_w, &mut config.theme_intensities[i]) {
                if config.theme == *theme {
                    config.visuals.glow_intensity = config.theme_intensities[i] * theme.glow_intensity();
                }
                return true;
            }

            current_y += 50.0;
        }

        // 3. Power Toggle
        let power_y = rect.y + rect.height - 60.0;
        if Rectangle::new(Point::new(rect.x + 20.0, power_y), Size::new(200.0, 35.0)).contains(pos) {
            config.power_user_mode = !config.power_user_mode;
            return true;
        }

        false
    }
}
