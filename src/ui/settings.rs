use cosmic::iced::{Rectangle, Color, Point, Pixels};
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
