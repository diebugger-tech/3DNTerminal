use cosmic::iced::{Point, Rectangle, Size, Color, widget::canvas::{Frame, Path, Stroke}, Pixels};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use crate::terminal::grid::TerminalGrid;
use crate::{AnimationPhase, CornerPosition};
use super::math;

/// 2D-Optimierte Parameter ohne 3D-Ballast
pub struct TerminalParams<'a> {
    pub phase: AnimationPhase,
    pub progress: f32,
    pub start_time: Instant,
    pub corner_rect: Rectangle,
    pub center_rect: Rectangle,
    pub cursor_visible: bool,
    pub window_controls: Option<&'a crate::ui::window_controls::WindowControls>,
    pub active_corner: CornerPosition,
    pub cursor_pos: Point,
    pub physics: crate::config::PhysicsConfig,
    pub a11y: crate::config::A11yConfig,
    pub hamburger_open: bool,
    pub notification: Option<&'a (String, Instant)>,
    pub active_overlay: crate::ui::overlay::OverlayMode,
    pub skills: &'a [Box<dyn crate::ui::skill::TerminalSkill>],
    pub glow_active: bool,
    pub tabs: &'a [String],
    pub active_tab: usize,
    pub action_flash: f32,
    pub neon_color: Color,
}

/// Reine 2D-Geometrie-Berechnung ohne Rotationswinkel
pub fn calculate_geometry(params: &TerminalParams) -> (Rectangle, f32) {
    let (mut rect, alpha) = match params.phase {
        AnimationPhase::Collapsed => {
            let is_static = params.physics.reduce_motion || params.a11y.reduce_motion > 0.8;
            let hover = if params.physics.breathe && !is_static {
                let time = params.start_time.elapsed().as_secs_f32();
                let strength = 1.0 - params.a11y.reduce_motion;
                (time * 2.0).sin() * 4.0 * strength
            } else {
                0.0
            };
            let mut r = params.corner_rect;
            r.y += hover;
            (r, 0.6)
        }
        AnimationPhase::Expanded => (params.center_rect, 1.0),
        AnimationPhase::Expanding | AnimationPhase::Collapsing => {
            let t = if params.phase == AnimationPhase::Expanding { params.progress } else { 1.0 - params.progress };
            let eased_t = math::cubic_bezier(t);
            let r = math::lerp_rect(params.corner_rect, params.center_rect, eased_t);
            let a = 0.6 + (0.4 * eased_t);
            (r, a)
        }
        AnimationPhase::Hidden => (params.corner_rect, 0.0),
    };

    // Magnetic Focus: Leichter Zug in Richtung Cursor
    if params.physics.magnetic && params.phase == AnimationPhase::Expanded {
        let dist_x = params.cursor_pos.x - (rect.x + rect.width / 2.0);
        let dist_y = params.cursor_pos.y - (rect.y + rect.height / 2.0);
        let max_dist = 500.0;
        let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
        
        if dist < max_dist {
            let strength = (1.0 - dist / max_dist).powi(2) * 15.0;
            rect.x += (dist_x / dist) * strength;
            rect.y += (dist_y / dist) * strength;
        }
    }

    (rect, alpha)
}

pub fn apply_color_filter(color: Color, filter: crate::config::ColorFilter) -> Color {
    use crate::config::ColorFilter;
    match filter {
        ColorFilter::None => color,
        ColorFilter::Protanopia => {
            let r = color.r * 0.567 + color.g * 0.433;
            let g = color.r * 0.558 + color.g * 0.442;
            let b = color.g * 0.242 + color.b * 0.758;
            Color::from_rgb(r, g, b)
        }
        ColorFilter::Deuteranopia => {
            let r = color.r * 0.625 + color.g * 0.375;
            let g = color.r * 0.7 + color.g * 0.3;
            let b = color.g * 0.3 + color.b * 0.7;
            Color::from_rgb(r, g, b)
        }
        ColorFilter::Tritanopia => {
            let r = color.r * 0.95 + color.g * 0.05;
            let g = color.g * 0.433 + color.b * 0.567;
            let b = color.g * 0.475 + color.b * 0.525;
            Color::from_rgb(r, g, b)
        }
    }
}

pub fn draw(
    frame: &mut Frame,
    grid_mutex: &Arc<Mutex<TerminalGrid>>,
    params: &TerminalParams,
) {
    let (rect, alpha) = calculate_geometry(params);
    let filter = params.a11y.color_filter;
    let neon_color = apply_color_filter(params.neon_color, filter);
    
    // Glassmorphism-Background
    let bg_color = apply_color_filter(Color::from_rgba(0.02, 0.02, 0.05, 0.92 * alpha), filter);

    if matches!(params.phase, AnimationPhase::Hidden) {
        let size = 16.0;
        let pos = match params.active_corner {
            CornerPosition::TopLeft     => Point::new(params.corner_rect.x + 8.0, params.corner_rect.y + 8.0),
            CornerPosition::TopRight    => Point::new(params.corner_rect.x + params.corner_rect.width - 24.0, params.corner_rect.y + 8.0),
            CornerPosition::BottomLeft  => Point::new(params.corner_rect.x + 8.0, params.corner_rect.y + params.corner_rect.height - 24.0),
            CornerPosition::BottomRight => Point::new(params.corner_rect.x + params.corner_rect.width - 24.0, params.corner_rect.y + params.corner_rect.height - 24.0),
            _ => Point::new(params.corner_rect.x + 8.0, params.corner_rect.y + 8.0),
        };
        frame.fill_rectangle(pos, Size::new(size, size), Color::from_rgba(neon_color.r, neon_color.g, neon_color.b, 0.4));
        return;
    }

    let path = Path::rounded_rectangle(rect.position(), rect.size(), 4.0.into());

    // Hintergrund-Dimming wenn Overlay offen
    if params.active_overlay != crate::ui::overlay::OverlayMode::None {
        frame.fill(&path, Color::from_rgba(0.0, 0.0, 0.0, 0.4 * alpha));
    }

    // Hintergrund
    frame.fill(&path, bg_color);
    
    // Glow-Rahmen
    for i in 1..=4 {
        let glow_width = i as f32 * 2.0;
        let glow_alpha = (0.3 / i as f32) * alpha;
        frame.stroke(&path, Stroke::default()
            .with_color(apply_color_filter(Color::from_rgba(params.neon_color.r, params.neon_color.g, params.neon_color.b, glow_alpha), filter))
            .with_width(glow_width));
    }

    // Header & Buttons
    let margin_x = 10.0;
    let margin_y = 8.0;
    let font_size = 14.0;

    let header_rect = Rectangle::new(
        Point::new(rect.x + margin_x, rect.y + margin_y),
        Size::new(rect.width - (margin_x * 2.0), font_size * 1.5)
    );

    frame.fill_text(cosmic::iced::widget::canvas::Text {
        content: "3DNTerminal".to_string(),
        position: Point::new(header_rect.x + 35.0, header_rect.y + 12.0),
        color: apply_color_filter(Color::from_rgba(0.0, 1.0, 0.8, alpha), filter),
        size: Pixels(font_size),
        ..Default::default()
    });

    if let Some(controls) = params.window_controls {
        let btn_size = 28.0;
        let left_anchor = Point::new(rect.x, rect.y);
        let right_anchor = Point::new(rect.x + rect.width, rect.y);
        controls.draw(frame, alpha, left_anchor, right_anchor, btn_size, params.cursor_pos);
    }

    // Grid Rendering
    if let Ok(grid) = grid_mutex.lock() {
        let start_x = rect.x + margin_x;
        let start_y = rect.y + margin_y + (font_size * 2.5);
        
        for y in 0..grid.rows {
            let y_pos = start_y + (y as f32 * 16.0);
            if y_pos > rect.y + rect.height - margin_y { break; }

            if let Some(row) = grid.get_visible_row(y) {
                for (col_idx, cell) in row.iter().enumerate() {
                    frame.fill_text(cosmic::iced::widget::canvas::Text {
                        content: cell.char.to_string(),
                        position: Point::new(start_x + (col_idx as f32 * 8.0), y_pos),
                        color: apply_color_filter(Color::from_rgba(0.9, 0.9, 0.9, alpha), filter),
                        size: Pixels(13.0),
                        ..Default::default()
                    });
                }
            }
        }

        if params.cursor_visible && alpha > 0.0 {
            let char_width = 8.0; 
            let line_height = 16.0;
            let cursor_x = start_x + (grid.cursor_x as f32 * char_width);
            let cursor_y = start_y + (grid.cursor_y as f32 * line_height);
            
            if cursor_x < rect.x + rect.width - margin_x && cursor_y < rect.y + rect.height - margin_y {
                frame.fill_rectangle(
                    Point::new(cursor_x, cursor_y - 12.0),
                    Size::new(char_width, 14.0),
                    Color::from_rgba(0.4, 1.0, 0.8, 0.8 * alpha)
                );
            }
        }
    }

    if params.hamburger_open {
        let menu_x = rect.x + 5.0;
        let menu_y = rect.y + 45.0;
        let menu_w = 280.0;
        let menu_h = 420.0;

        let menu_path = Path::rectangle(Point::new(menu_x, menu_y), Size::new(menu_w, menu_h));
        frame.fill(&menu_path, Color::from_rgba(0.02, 0.02, 0.05, 0.95 * alpha));
        frame.stroke(&menu_path, Stroke::default().with_color(apply_color_filter(Color::from_rgba(1.0, 0.6, 0.0, 0.4 * alpha), filter)).with_width(1.5));

        let items = crate::ui::hamburger_menu::HamburgerMenu::items(params.skills);
        for (i, item) in items.iter().enumerate() {
            let item_y = menu_y + (i as f32 * 60.0);
            let is_hovered = params.cursor_pos.x >= menu_x && params.cursor_pos.x <= menu_x + menu_w 
                          && params.cursor_pos.y >= item_y && params.cursor_pos.y <= item_y + 60.0;
            
            if is_hovered {
                frame.fill(&Path::rectangle(Point::new(menu_x, item_y), Size::new(menu_w, 60.0)), apply_color_filter(Color::from_rgba(1.0, 0.6, 0.0, 0.1 * alpha), filter));
            }

            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: item.label.to_string(),
                position: Point::new(menu_x + 15.0, item_y + 25.0),
                color: apply_color_filter(Color::from_rgba(1.0, 0.8, 0.2, alpha), filter),
                size: Pixels(18.0),
                ..Default::default()
            });

            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: item.subtitle.to_string(),
                position: Point::new(menu_x + 15.0, item_y + 45.0),
                color: apply_color_filter(Color::from_rgba(1.0, 0.6, 0.0, 0.6 * alpha), filter),
                size: Pixels(12.0),
                ..Default::default()
            });
        }
    }

    if params.active_overlay != crate::ui::overlay::OverlayMode::None {
        let overlay_id = match params.active_overlay {
            crate::ui::overlay::OverlayMode::Settings => "settings",
            crate::ui::overlay::OverlayMode::Physics => "physics",
            crate::ui::overlay::OverlayMode::Themes => "themes",
            crate::ui::overlay::OverlayMode::A11y => "a11y",
            _ => "",
        };

        if let Some(skill) = params.skills.iter().find(|s| s.id() == overlay_id) {
            let overlay_w = 400.0;
            let overlay_h = 350.0;
            let overlay_rect = Rectangle::new(
                Point::new(rect.x + (rect.width - overlay_w) / 2.0, rect.y + (rect.height - overlay_h) / 2.0),
                Size::new(overlay_w, overlay_h)
            );
            skill.draw_overlay(frame, overlay_rect, alpha, params);
        }
    }

    crate::ui::notification::draw(frame, rect, alpha, params);
}
