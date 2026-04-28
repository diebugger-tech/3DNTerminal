use cosmic::iced::{Point, Color, widget::canvas::{Frame, Path, Stroke}, Size};
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

    match icon {
        IconType::Hamburger => {
            let h = size * 0.6;
            let spacing = h / 2.0;
            let y_start = pos.y + (size - h) / 2.0;
            for i in 0..3 {
                let y = y_start + i as f32 * spacing;
                frame.stroke(&Path::line(Point::new(pos.x, y), Point::new(pos.x + size, y)), stroke);
            }
        }
        IconType::NewTab => {
            let m = size * 0.2;
            frame.stroke(&Path::line(Point::new(pos.x + size/2.0, pos.y + m), Point::new(pos.x + size/2.0, pos.y + size - m)), stroke);
            frame.stroke(&Path::line(Point::new(pos.x + m, pos.y + size/2.0), Point::new(pos.x + size - m, pos.y + size/2.0)), stroke);
        }
        IconType::Close => {
            let m = size * 0.25;
            let p1 = Path::line(Point::new(pos.x + m, pos.y + m), Point::new(pos.x + size - m, pos.y + size - m));
            let p2 = Path::line(Point::new(pos.x + size - m, pos.y + m), Point::new(pos.x + m, pos.y + size - m));
            frame.stroke(&p1, stroke);
            frame.stroke(&p2, stroke);
        }
        IconType::Maximize => {
            let m = size * 0.25;
            let rect = Path::rectangle(Point::new(pos.x + m, pos.y + m), Size::new(size - 2.0*m, size - 2.0*m));
            frame.stroke(&rect, stroke);
        }
        IconType::Minimize => {
            let m = size * 0.25;
            frame.stroke(&Path::line(Point::new(pos.x + m, pos.y + size/2.0), Point::new(pos.x + size - m, pos.y + size/2.0)), stroke);
        }
        IconType::CornerTL => draw_arrow(frame, pos, size, -1.0, -1.0, stroke),
        IconType::CornerTR => draw_arrow(frame, pos, size, 1.0, -1.0, stroke),
        IconType::CornerBL => draw_arrow(frame, pos, size, -1.0, 1.0, stroke),
        IconType::CornerBR => draw_arrow(frame, pos, size, 1.0, 1.0, stroke),
        IconType::Anchor => {
            let m = size * 0.2;
            // Simplified anchor
            frame.stroke(&Path::line(Point::new(pos.x + size/2.0, pos.y + m), Point::new(pos.x + size/2.0, pos.y + size - m)), stroke);
            frame.stroke(&Path::circle(Point::new(pos.x + size/2.0, pos.y + m + 2.0), 3.0), stroke);
        }
        IconType::Resize => {
            let m = size * 0.2;
            let p1 = Path::line(Point::new(pos.x + m, pos.y + size - m), Point::new(pos.x + size - m, pos.y + m));
            frame.stroke(&p1, stroke);
            // Small arrows
            draw_arrow_head(frame, Point::new(pos.x + size - m, pos.y + m), 1.0, -1.0, size * 0.3, stroke);
            draw_arrow_head(frame, Point::new(pos.x + m, pos.y + size - m), -1.0, 1.0, size * 0.3, stroke);
        }
        IconType::Settings => {
            let m = size * 0.2;
            let radius = size / 2.0 - m;
            let center = Point::new(pos.x + size/2.0, pos.y + size/2.0);
            frame.stroke(&Path::circle(center, radius), stroke);
            // Gear teeth
            for i in 0..8 {
                let angle = i as f32 * std::f32::consts::PI / 4.0;
                let p1 = Point::new(center.x + angle.cos() * radius, center.y + angle.sin() * radius);
                let p2 = Point::new(center.x + angle.cos() * (radius + 4.0), center.y + angle.sin() * (radius + 4.0));
                frame.stroke(&Path::line(p1, p2), stroke);
            }
        }
        IconType::Physics => {
            let m = size * 0.2;
            let mut points = Vec::new();
            for i in 0..10 {
                let x = pos.x + m + (i as f32 * (size - 2.0*m) / 10.0);
                let y = pos.y + size/2.0 + (i as f32 * 0.8).sin() * (size * 0.2);
                points.push(Point::new(x, y));
            }
            for i in 0..points.len()-1 {
                frame.stroke(&Path::line(points[i], points[i+1]), stroke);
            }
        }
        IconType::Search => {
            let m = size * 0.25;
            let radius = size * 0.25;
            let center = Point::new(pos.x + m + radius, pos.y + m + radius);
            frame.stroke(&Path::circle(center, radius), stroke);
            frame.stroke(&Path::line(Point::new(center.x + radius * 0.7, center.y + radius * 0.7), Point::new(pos.x + size - m, pos.y + size - m)), stroke);
        }
        IconType::Keyboard => {
            let m = size * 0.2;
            let rect = Path::rectangle(Point::new(pos.x + m, pos.y + m + 5.0), Size::new(size - 2.0*m, size - 2.0*m - 5.0));
            frame.stroke(&rect, stroke);
            // Keys
            for i in 1..3 {
                let y = pos.y + m + 5.0 + i as f32 * 4.0;
                frame.stroke(&Path::line(Point::new(pos.x + m + 4.0, y), Point::new(pos.x + size - m - 4.0, y)), stroke.with_width(1.0));
            }
        }
        IconType::Palette => {
            let m = size * 0.2;
            let center = Point::new(pos.x + size/2.0, pos.y + size/2.0);
            frame.stroke(&Path::circle(center, size/2.0 - m), stroke);
            // Color spots
            for i in 0..3 {
                let angle = i as f32 * 2.0 * std::f32::consts::PI / 3.0;
                let spot = Point::new(center.x + angle.cos() * 4.0, center.y + angle.sin() * 4.0);
                frame.stroke(&Path::circle(spot, 2.0), stroke.with_width(1.0));
            }
        }
        IconType::A11y => {
            let m = size * 0.2;
            let center = Point::new(pos.x + size/2.0, pos.y + size/2.0);
            frame.stroke(&Path::circle(center, size/2.0 - m), stroke);
            // Simple stick figure
            frame.stroke(&Path::line(Point::new(center.x, center.y - 4.0), Point::new(center.x, center.y + 4.0)), stroke.with_width(1.0));
            frame.stroke(&Path::line(Point::new(center.x - 4.0, center.y), Point::new(center.x + 4.0, center.y)), stroke.with_width(1.0));
        }
    }
}

fn draw_arrow(frame: &mut Frame, pos: Point, size: f32, dx: f32, dy: f32, stroke: Stroke) {
    let m = size * 0.25;
    let center = Point::new(pos.x + size/2.0, pos.y + size/2.0);
    let end = Point::new(center.x + dx * (size/2.0 - m), center.y + dy * (size/2.0 - m));
    frame.stroke(&Path::line(center, end), stroke);
    draw_arrow_head(frame, end, dx, dy, size * 0.3, stroke);
}

fn draw_arrow_head(frame: &mut Frame, end: Point, dx: f32, dy: f32, len: f32, stroke: Stroke) {
    frame.stroke(&Path::line(end, Point::new(end.x - dx * len, end.y)), stroke);
    frame.stroke(&Path::line(end, Point::new(end.x, end.y - dy * len)), stroke);
}
