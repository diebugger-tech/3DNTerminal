use cosmic::iced::{Rectangle, Color, Point, Size, Pixels};
use cosmic::iced::widget::canvas::{Frame, Path, Stroke, Text};
use crate::ui::two_d::TerminalParams;
use crate::ui::overlay::OverlayMode;

pub fn draw(
    frame: &mut Frame,
    rect: Rectangle,
    alpha: f32,
    params: &TerminalParams,
) {
    if params.active_overlay == OverlayMode::None { return; }

    let settings_w = 400.0;
    let settings_h = 320.0;
    let settings_rect = Rectangle {
        x: rect.x + (rect.width - settings_w) / 2.0,
        y: rect.y + (rect.height - settings_h) / 2.0,
        width: settings_w,
        height: settings_h,
    };

    let settings_path = Path::rounded_rectangle(settings_rect.position(), settings_rect.size(), 8.0.into());
    
    // Glass Effect
    frame.fill(&settings_path, Color::from_rgba(0.01, 0.01, 0.05, 0.95 * alpha));
    frame.stroke(&settings_path, Stroke::default()
        .with_color(params.active_overlay.title_color())
        .with_width(2.0));

    // Title
    frame.fill_text(Text {
        content: params.active_overlay.label().to_string(),
        position: Point::new(settings_rect.x + 20.0, settings_rect.y + 35.0),
        color: params.active_overlay.title_color(),
        size: Pixels(22.0),
        ..Default::default()
    });

    // Close Button [ X ]
    frame.fill_text(Text {
        content: "[ X ]".to_string(),
        position: Point::new(settings_rect.x + settings_w - 60.0, settings_rect.y + 30.0),
        color: Color::from_rgba(1.0, 0.3, 0.0, alpha),
        size: Pixels(18.0),
        ..Default::default()
    });

    // Delegate to Skill
    let id = match params.active_overlay {
        OverlayMode::Settings => "settings",
        OverlayMode::Physics => "physics",
        OverlayMode::Themes => "themes",
        _ => "",
    };

    if let Some(skill) = params.skills.iter().find(|s| s.id() == id) {
        skill.draw_overlay(frame, settings_rect, alpha, params);
    }
}

fn draw_physics_info(frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
    let physics_y = rect.y + 80.0;
    frame.fill_text(Text {
        content: format!("Physics Engine: {:?}", params.physics_mode),
        position: Point::new(rect.x + 20.0, physics_y),
        color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
        size: Pixels(16.0),
        ..Default::default()
    });
    
    frame.fill_text(Text {
        content: "Status: Active & Modularized".to_string(),
        position: Point::new(rect.x + 20.0, physics_y + 30.0),
        color: Color::from_rgba(0.4, 0.8, 0.4, alpha),
        size: Pixels(14.0),
        ..Default::default()
    });
}

fn draw_themes_info(frame: &mut Frame, rect: Rectangle, alpha: f32, _params: &TerminalParams) {
    let y = rect.y + 80.0;
    frame.fill_text(Text {
        content: "Select Theme (Cycle via menu for now)".to_string(),
        position: Point::new(rect.x + 20.0, y),
        color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
        size: Pixels(16.0),
        ..Default::default()
    });
    
    for (i, (name, color)) in [
        ("Amber", Color::from_rgba(1.0, 0.6, 0.0, 1.0)),
        ("Magenta", Color::from_rgba(1.0, 0.0, 0.8, 1.0)),
        ("Cyan", Color::from_rgba(0.0, 1.0, 0.8, 1.0)),
        ("Green", Color::from_rgba(0.0, 1.0, 0.2, 1.0)),
    ].iter().enumerate() {
        let chip_x = rect.x + 20.0 + (i as f32 * 90.0);
        let chip_y = y + 40.0;
        frame.fill_rectangle(Point::new(chip_x, chip_y), Size::new(80.0, 30.0), *color);
        frame.fill_text(Text {
            content: name.to_string(),
            position: Point::new(chip_x, chip_y + 45.0),
            color: Color::from_rgba(0.7, 0.7, 0.7, alpha),
            size: Pixels(12.0),
            ..Default::default()
        });
    }
}

fn draw_shortcuts_info(frame: &mut Frame, rect: Rectangle, alpha: f32, _params: &TerminalParams) {
    let y = rect.y + 80.0;
    let shortcuts = [
        ("F12", "Toggle Expand/Collapse"),
        ("Left-Click Header", "Drag Window"),
        ("Arrows (Menu)", "Jump to Corner"),
        ("+ (Menu)", "New Tab Session"),
        ("X (Overlay)", "Close this panel"),
    ];

    for (i, (key, desc)) in shortcuts.iter().enumerate() {
        let line_y = y + (i as f32 * 30.0);
        frame.fill_text(Text {
            content: format!("{:<15} {}", key, desc),
            position: Point::new(rect.x + 20.0, line_y),
            color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });
    }
}

fn draw_search_info(frame: &mut Frame, rect: Rectangle, alpha: f32, _params: &TerminalParams) {
    let y = rect.y + 80.0;
    frame.fill_text(Text {
        content: "SEARCH ENGINE".to_string(),
        position: Point::new(rect.x + 20.0, y),
        color: Color::from_rgba(0.5, 0.5, 1.0, alpha),
        size: Pixels(18.0),
        ..Default::default()
    });
    
    frame.fill_text(Text {
        content: "Type your query to filter terminal history...".to_string(),
        position: Point::new(rect.x + 20.0, y + 40.0),
        color: Color::from_rgba(0.6, 0.6, 0.6, alpha),
        size: Pixels(14.0),
        ..Default::default()
    });
    
    let input_rect = Rectangle::new(Point::new(rect.x + 20.0, y + 70.0), Size::new(rect.width - 40.0, 40.0));
    frame.stroke(&Path::rectangle(input_rect.position(), input_rect.size()), Stroke::default().with_color(Color::from_rgba(0.3, 0.3, 0.5, alpha)));
}
