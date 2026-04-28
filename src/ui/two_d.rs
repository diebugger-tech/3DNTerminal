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
    pub physics_mode: crate::config::PhysicsMode,
    pub hamburger_open: bool,
    pub notification: Option<&'a (String, Instant)>,
    pub settings_open: bool,
    pub tabs: &'a [String],
    pub active_tab: usize,
    pub action_flash: f32,
}

/// Reine 2D-Geometrie-Berechnung ohne Rotationswinkel
pub fn calculate_geometry(params: &TerminalParams) -> (Rectangle, f32) {
    match params.phase {
        AnimationPhase::Collapsed => {
            // Ein leichtes Schweben in der Ecke als 2D-Effekt
            let hover = match params.physics_mode {
                crate::config::PhysicsMode::Breathe => {
                    let time = params.start_time.elapsed().as_secs_f32();
                    (time * 2.0).sin() * 4.0
                }
                _ => 0.0
            };
            let mut rect = params.corner_rect;
            rect.y += hover;
            (rect, 0.6)
        }
        AnimationPhase::Expanded => (params.center_rect, 1.0),
        AnimationPhase::Expanding | AnimationPhase::Collapsing => {
            let t = if params.phase == AnimationPhase::Expanding { params.progress } else { 1.0 - params.progress };
            let eased_t = math::cubic_bezier(t);
            
            let start = params.corner_rect;
            let end = params.center_rect;
            
            let rect = Rectangle::new(
                Point::new(start.x + (end.x - start.x) * eased_t, start.y + (end.y - start.y) * eased_t),
                Size::new(start.width + (end.width - start.width) * eased_t, start.height + (end.height - start.height) * eased_t)
            );
            
            let alpha = 0.6 + (0.4 * eased_t);
            (rect, alpha)
        }
        AnimationPhase::Hidden => (params.corner_rect, 0.0),
    }
}

pub fn draw(
    frame: &mut Frame,
    grid_mutex: &Arc<Mutex<TerminalGrid>>,
    params: &TerminalParams,
) {
    if matches!(params.phase, AnimationPhase::Hidden) {
        // Kleiner Indikator in der Ecke
        let size = 16.0;
        let pos = match params.active_corner {
            CornerPosition::TopLeft     => Point::new(params.corner_rect.x + 8.0, params.corner_rect.y + 8.0),
            CornerPosition::TopRight    => Point::new(params.corner_rect.x + params.corner_rect.width - size - 8.0, params.corner_rect.y + 8.0),
            CornerPosition::BottomLeft  => Point::new(params.corner_rect.x + 8.0, params.corner_rect.y + params.corner_rect.height - size - 8.0),
            CornerPosition::BottomRight => Point::new(params.corner_rect.x + params.corner_rect.width - size - 8.0, params.corner_rect.y + params.corner_rect.height - size - 8.0),
            CornerPosition::Free        => Point::new(params.corner_rect.x + params.corner_rect.width/2.0 - size/2.0, params.corner_rect.y + params.corner_rect.height - size - 8.0),
        };
        
        frame.fill_rectangle(pos, Size::new(size, size), Color::from_rgba(0.4, 1.0, 0.8, 0.8));
        return;
    }

    let (rect, alpha) = calculate_geometry(params);
    let path = Path::rounded_rectangle(rect.position(), rect.size(), 4.0.into());

    // Hintergrund
    frame.fill(&path, Color::from_rgba(0.05, 0.1, 0.2, 0.8 * alpha));
    
    // Glow-Rahmen
    for i in 1..=4 {
        let glow_width = i as f32 * 2.0;
        let glow_alpha = (0.3 / i as f32) * alpha;
        frame.stroke(&path, Stroke::default()
            .with_color(Color::from_rgba(0.4, 1.0, 0.8, glow_alpha))
            .with_width(glow_width));
    }

    // Header & Buttons
    let margin_x = 10.0;
    let margin_y = 8.0;
    let font_size = 14.0;

    // Header-Balken oben
    let header_rect = Rectangle::new(
        Point::new(rect.x + margin_x, rect.y + margin_y),
        Size::new(rect.width - (margin_x * 2.0), font_size * 1.5)
    );
    frame.fill_rectangle(header_rect.position(), header_rect.size(), Color::from_rgba(0.4, 1.0, 0.8, 0.1 * alpha));

    // Titel
    frame.fill_text(cosmic::iced::widget::canvas::Text {
        content: "3DNTerminal".to_string(),
        position: Point::new(rect.x + margin_x + 85.0, rect.y + margin_y + 2.0), // Platz für ☰ und +
        color: Color::from_rgba(0.4, 1.0, 0.8, alpha),
        size: Pixels(font_size),
        ..Default::default()
    });

    // Action Flash (Glow Effekt)
    if params.action_flash > 0.0 {
        let flash_rect = Rectangle {
            x: rect.x - 2.0,
            y: rect.y - 2.0,
            width: rect.width + 4.0,
            height: rect.height + 4.0,
        };
        let flash_path = Path::rectangle(Point::new(flash_rect.x, flash_rect.y), Size::new(flash_rect.width, flash_rect.height));
        frame.stroke(&flash_path, Stroke::default().with_color(Color::from_rgba(1.0, 0.6, 0.0, 0.4 * params.action_flash * alpha)).with_width(3.0));
    }

    // Window Controls & TabBar
    if let Some(controls) = params.window_controls {
        let btn_size = (rect.width * 0.03).clamp(12.0, 26.0);
        let left_anchor = Point::new(rect.x, rect.y);
        let right_anchor = Point::new(rect.x + rect.width, rect.y);
        controls.draw(frame, alpha, left_anchor, right_anchor, btn_size, params.cursor_pos);
        
        // Modularer TabBar Aufruf
        crate::ui::tab_bar::draw(frame, rect, alpha, params);
    }

    // Terminal Grid
    if let Ok(grid) = grid_mutex.lock() {
        let line_height = 16.0;
        let start_y = rect.y + margin_y + (font_size * 2.5);
        let start_x = rect.x + margin_x + 5.0;

        for y in 0..grid.rows {
            let current_y = start_y + (y as f32 * line_height);
            if current_y > rect.y + rect.height - margin_y { break; }

            if let Some(row) = grid.get_visible_row(y) {
                let mut row_text = String::new();
                for cell in row.iter() {
                    row_text.push(cell.char);
                }
                
                frame.fill_text(cosmic::iced::widget::canvas::Text {
                    content: row_text,
                    position: Point::new(start_x, current_y),
                    color: Color::from_rgba(0.9, 0.9, 0.9, alpha),
                    size: Pixels(13.0),
                    ..Default::default()
                });
            }
        }

        // Cursor zeichnen (wenn sichtbar)
        if params.cursor_visible && alpha > 0.0 {
            let char_width = 8.0; 
            let line_height = 16.0;
            let cursor_x = start_x + (grid.cursor_x as f32 * char_width);
            let cursor_y = rect.y + margin_y + (font_size * 2.5) + (grid.cursor_y as f32 * line_height);
            
            if cursor_x < rect.x + rect.width - margin_x && cursor_y < rect.y + rect.height - margin_y {
                frame.fill_rectangle(
                    Point::new(cursor_x, cursor_y - 12.0),
                    Size::new(char_width, 14.0),
                    Color::from_rgba(0.4, 1.0, 0.8, 0.8 * alpha)
                );
            }
        }
    }

    // Hamburger Menu (Blade Runner Style)
    if params.hamburger_open {
        let menu_x = rect.x + 5.0;
        let menu_y = rect.y + 45.0;
        let menu_w = 280.0;
        let menu_h = 420.0;

        let menu_path = Path::rectangle(Point::new(menu_x, menu_y), Size::new(menu_w, menu_h));
        frame.fill(&menu_path, Color::from_rgba(0.02, 0.02, 0.05, 0.95 * alpha));
        frame.stroke(&menu_path, Stroke::default().with_color(Color::from_rgba(1.0, 0.6, 0.0, 0.4 * alpha)).with_width(1.5));

        let items = crate::ui::hamburger_menu::HamburgerMenu::items();
        for (i, item) in items.iter().enumerate() {
            let item_y = menu_y + (i as f32 * 60.0);
            let is_hovered = params.cursor_pos.x >= menu_x && params.cursor_pos.x <= menu_x + menu_w 
                          && params.cursor_pos.y >= item_y && params.cursor_pos.y <= item_y + 60.0;
            
            if is_hovered {
                frame.fill(&Path::rectangle(Point::new(menu_x, item_y), Size::new(menu_w, 60.0)), Color::from_rgba(1.0, 0.6, 0.0, 0.1 * alpha));
            }

            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: item.label.to_string(),
                position: Point::new(menu_x + 15.0, item_y + 25.0),
                color: Color::from_rgba(1.0, 0.8, 0.2, alpha),
                size: Pixels(18.0),
                ..Default::default()
            });

            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: item.subtitle.to_string(),
                position: Point::new(menu_x + 15.0, item_y + 45.0),
                color: Color::from_rgba(1.0, 0.6, 0.0, 0.6 * alpha),
                size: Pixels(12.0),
                ..Default::default()
            });
        }
    }

    // Modularer Settings Aufruf
    crate::ui::settings::draw(frame, rect, alpha, params);

    // Modularer Notification Aufruf
    crate::ui::notification::draw(frame, rect, alpha, params);
}
