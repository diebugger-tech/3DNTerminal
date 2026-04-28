use cosmic::iced::{Point, Color, widget::canvas::{Frame, Path, Stroke}, Size, Pixels};
use crate::config::TerminalTheme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconType {
    Hamburger,
    NewTab,
    Close,
    Maximize,
    Minimize,
    CornerTL,
    CornerTR,
    CornerBL,
    CornerBR,
    Anchor,
    Resize,
    Settings,
    Physics,
    Search,
    Keyboard,
    Palette,
    A11y,
}

pub fn draw(frame: &mut Frame, icon: IconType, theme: TerminalTheme, pos: Point, size: f32, color: Color) {
    let stroke_width = match theme {
        TerminalTheme::AppleGlass => 1.0,
        TerminalTheme::Classic => 1.5,
        TerminalTheme::RetroAmber => 2.0,
        _ => 1.8,
    };

    let mut stroke = Stroke::default().with_color(color).with_width(stroke_width);
    
    // Theme-specific stroke modifications
    if theme == TerminalTheme::RetroAmber {
        stroke = stroke.with_width(2.5); // Thicker for pixel look
    }

    let is_clean_theme = theme == TerminalTheme::Classic || theme == TerminalTheme::AppleGlass || theme == TerminalTheme::Transparent;

    let is_clean_theme = theme == TerminalTheme::Classic || theme == TerminalTheme::AppleGlass || theme == TerminalTheme::Transparent;

    match icon {
        IconType::Hamburger => {
            if is_clean_theme { draw_unicode(frame, "☰", pos, size, color); }
            else {
                let h = size * 0.6;
                let spacing = h / 2.0;
                let y_start = pos.y + (size - h) / 2.0;
                for i in 0..3 {
                    let y = y_start + i as f32 * spacing;
                    frame.stroke(&Path::line(Point::new(pos.x, y), Point::new(pos.x + size, y)), stroke);
                }
            }
        }
        IconType::NewTab => {
            if is_clean_theme { draw_unicode(frame, "+", pos, size, color); }
            else {
                let m = size * 0.2;
                frame.stroke(&Path::line(Point::new(pos.x + size/2.0, pos.y + m), Point::new(pos.x + size/2.0, pos.y + size - m)), stroke);
                frame.stroke(&Path::line(Point::new(pos.x + m, pos.y + size/2.0), Point::new(pos.x + size - m, pos.y + size/2.0)), stroke);
            }
        }
        IconType::Close => {
            if is_clean_theme { draw_unicode(frame, "✕", pos, size, color); }
            else {
                let m = size * 0.25;
                frame.stroke(&Path::line(Point::new(pos.x + m, pos.y + m), Point::new(pos.x + size - m, pos.y + size - m)), stroke);
                frame.stroke(&Path::line(Point::new(pos.x + size - m, pos.y + m), Point::new(pos.x + m, pos.y + size - m)), stroke);
            }
        }
        IconType::Maximize => {
            if is_clean_theme { draw_unicode(frame, "□", pos, size, color); }
            else {
                let m = size * 0.25;
                frame.stroke(&Path::rectangle(Point::new(pos.x + m, pos.y + m), Size::new(size - 2.0*m, size - 2.0*m)), stroke);
            }
        }
        IconType::Minimize => {
            if is_clean_theme { draw_unicode(frame, "—", pos, size, color); }
            else {
                let m = size * 0.25;
                frame.stroke(&Path::line(Point::new(pos.x + m, pos.y + size/2.0), Point::new(pos.x + size - m, pos.y + size/2.0)), stroke);
            }
        }
        IconType::CornerTL => draw_arrow(frame, pos, size, -1.0, -1.0, theme, color, stroke),
        IconType::CornerTR => draw_arrow(frame, pos, size, 1.0, -1.0, theme, color, stroke),
        IconType::CornerBL => draw_arrow(frame, pos, size, -1.0, 1.0, theme, color, stroke),
        IconType::CornerBR => draw_arrow(frame, pos, size, 1.0, 1.0, theme, color, stroke),
        IconType::Anchor => {
            if is_clean_theme { draw_unicode(frame, "⚓", pos, size, color); }
            else {
                let m = size * 0.2;
                frame.stroke(&Path::line(Point::new(pos.x + size/2.0, pos.y + m), Point::new(pos.x + size/2.0, pos.y + size - m)), stroke);
                frame.stroke(&Path::circle(Point::new(pos.x + size/2.0, pos.y + m + 2.0), 3.0), stroke);
            }
        }
        IconType::Resize => {
            if is_clean_theme { draw_unicode(frame, "⤢", pos, size, color); }
            else {
                let m = size * 0.3;
                frame.stroke(&Path::line(Point::new(pos.x + m, pos.y + size - m), Point::new(pos.x + size - m, pos.y + m)), stroke);
            }
        }
        IconType::Settings => {
            if is_clean_theme { draw_unicode(frame, "⚙", pos, size, color); }
            else {
                let center = Point::new(pos.x + size/2.0, pos.y + size/2.0);
                frame.stroke(&Path::circle(center, size/4.0), stroke);
            }
        }
        IconType::Physics => {
            if is_clean_theme { draw_unicode(frame, "♒", pos, size, color); }
            else {
                let m = size * 0.2;
                frame.stroke(&Path::line(Point::new(pos.x + m, pos.y + size/2.0), Point::new(pos.x + size - m, pos.y + size/2.0)), stroke);
            }
        }
        IconType::Search => {
            if is_clean_theme { draw_unicode(frame, "🔍", pos, size, color); }
            else {
                let m = size * 0.3;
                frame.stroke(&Path::circle(Point::new(pos.x + m, pos.y + m), size/4.0), stroke);
            }
        }
        IconType::Keyboard => {
            if is_clean_theme { draw_unicode(frame, "⌨", pos, size, color); }
            else {
                frame.stroke(&Path::rectangle(Point::new(pos.x + 2.0, pos.y + 4.0), Size::new(size - 4.0, size - 8.0)), stroke);
            }
        }
        IconType::Palette => {
            if is_clean_theme { draw_unicode(frame, "🎨", pos, size, color); }
            else {
                frame.stroke(&Path::circle(Point::new(pos.x + size/2.0, pos.y + size/2.0), size/3.0), stroke);
            }
        }
        IconType::A11y => {
            if is_clean_theme { draw_unicode(frame, "♿", pos, size, color); }
            else {
                frame.stroke(&Path::circle(Point::new(pos.x + size/2.0, pos.y + size/2.0), size/3.0), stroke);
            }
        }
    }
}

fn draw_arrow(frame: &mut Frame, pos: Point, size: f32, dx: f32, dy: f32, theme: TerminalTheme, color: Color, stroke: Stroke) {
    match theme {
        TerminalTheme::Classic | TerminalTheme::AppleGlass | TerminalTheme::Transparent => {
            let symbol = if dx < 0.0 && dy < 0.0 { "↖" }
                        else if dx > 0.0 && dy < 0.0 { "↗" }
                        else if dx < 0.0 && dy > 0.0 { "↙" }
                        else { "↘" };
            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: symbol.to_string(),
                position: Point::new(pos.x + size/4.0, pos.y),
                color,
                size: Pixels(size * 0.9),
                ..Default::default()
            });
        }
        TerminalTheme::RetroAmber => {
            // Blocky Arrow
            let m = size * 0.2;
            let center = Point::new(pos.x + size/2.0, pos.y + size/2.0);
            let end = Point::new(center.x + dx * (size/2.0 - m), center.y + dy * (size/2.0 - m));
            frame.stroke(&Path::line(center, end), stroke.with_width(3.0));
            draw_arrow_head(frame, end, dx, dy, size * 0.4, stroke.with_width(3.0));
        }
        TerminalTheme::BladeRunner => {
            // Technical Target Arrow
            let m = size * 0.2;
            let center = Point::new(pos.x + size/2.0, pos.y + size/2.0);
            let end = Point::new(center.x + dx * (size/2.0 - m), center.y + dy * (size/2.0 - m));
            frame.stroke(&Path::circle(center, 2.0), stroke);
            frame.stroke(&Path::line(center, end), stroke.with_width(0.5));
            draw_arrow_head(frame, end, dx, dy, size * 0.3, stroke);
        }
        _ => {
            let m = size * 0.25;
            let center = Point::new(pos.x + size/2.0, pos.y + size/2.0);
            let end = Point::new(center.x + dx * (size/2.0 - m), center.y + dy * (size/2.0 - m));
            frame.stroke(&Path::line(center, end), stroke);
            draw_arrow_head(frame, end, dx, dy, size * 0.3, stroke);
        }
    }
}

fn draw_arrow_head(frame: &mut Frame, end: Point, dx: f32, dy: f32, len: f32, stroke: Stroke) {
    frame.stroke(&Path::line(end, Point::new(end.x - dx * len, end.y)), stroke);
    frame.stroke(&Path::line(end, Point::new(end.x, end.y - dy * len)), stroke);
}

fn draw_unicode(frame: &mut Frame, content: &str, pos: Point, size: f32, color: Color) {
    frame.fill_text(cosmic::iced::widget::canvas::Text {
        content: content.to_string(),
        position: Point::new(pos.x + size/4.0, pos.y),
        color,
        size: Pixels(size * 0.9),
        ..Default::default()
    });
}
