use cosmic::iced::{Point, Rectangle, Size, Color, widget::canvas::{Frame, Path, Stroke}, Pixels};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use crate::terminal::grid::TerminalGrid;
use crate::AnimationPhase;
use super::math;

pub struct HologramParams<'a> {
    pub phase: AnimationPhase,
    pub progress: f32,
    pub start_time: Instant,
    pub corner_rect: Rectangle,
    pub center_rect: Rectangle,
    pub cursor_visible: bool,
    pub window_controls: Option<&'a crate::ui::window_controls::WindowControls>,
}

pub fn calculate_3d_geometry(params: &HologramParams) -> (Rectangle, f32, f32) {
    let switch_t = 0.416; // 250ms / 600ms mark
    
    match params.phase {
        AnimationPhase::Collapsed => {
            let time = params.start_time.elapsed().as_secs_f32();
            let hover = (time * 2.0).sin() * 8.0;
            let mut rect = params.corner_rect;
            rect.y += hover;
            (rect, -18.0, 0.4)
        }
        AnimationPhase::Expanded => (params.center_rect, 0.0, 1.0),
        AnimationPhase::Expanding | AnimationPhase::Collapsing => {
            let t = if params.phase == AnimationPhase::Expanding { params.progress } else { 1.0 - params.progress };
            let eased_t = math::cubic_bezier(t);
            let alpha = 0.4 + (0.6 * eased_t);
            
            if eased_t < switch_t {
                let p = eased_t / switch_t;
                let angle = p * 90.0;
                (params.corner_rect, angle, alpha)
            } else {
                let p = (eased_t - switch_t) / (1.0 - switch_t);
                let angle = 90.0 * (1.0 - p);
                (params.center_rect, angle, alpha)
            }
        }
    }
}

pub fn get_quad(params: &HologramParams) -> [Point; 4] {
    let (rect, angle_y, _) = calculate_3d_geometry(params);
    let center = rect.center();
    let rad = angle_y.to_radians();
    let cos_a = rad.cos();
    let w = rect.width * cos_a;
    let h = rect.height;
    let perspective = (rad.sin() * 40.0).abs();
    
    let p1 = Point::new(center.x - w/2.0, center.y - h/2.0 + perspective);
    let p2 = Point::new(center.x + w/2.0, center.y - h/2.0 - perspective);
    let p3 = Point::new(center.x + w/2.0, center.y + h/2.0 + perspective);
    let p4 = Point::new(center.x - w/2.0, center.y + h/2.0 - perspective);
    [p1, p2, p3, p4]
}

pub fn draw(
    frame: &mut Frame,
    grid_mutex: &Arc<Mutex<TerminalGrid>>,
    params: &HologramParams,
) {
    let (target_rect, angle_y, alpha) = calculate_3d_geometry(params);
    
    let quad = get_quad(params);
    let p1 = quad[0];
    let p2 = quad[1];
    let p3 = quad[2];
    let p4 = quad[3];
    
    let rad = angle_y.to_radians();
    let cos_a = rad.cos();
    let w = target_rect.width * cos_a;
    let h = target_rect.height;

    let path = Path::new(|b| {
        b.move_to(p1);
        b.line_to(p2);
        b.line_to(p3);
        b.line_to(p4);
        b.close();
    });

    let bg_alpha = 0.8 * alpha;
    let border_alpha = alpha;

    if bg_alpha > 0.0 {
        frame.fill(&path, Color::from_rgba(0.05, 0.1, 0.2, bg_alpha));
        
        for i in 1..=6 {
            let glow_width = i as f32 * 4.0;
            let glow_alpha = (0.4 / i as f32) * border_alpha;
            frame.stroke(&path, Stroke::default()
                .with_color(Color::from_rgba(0.4, 1.0, 0.8, glow_alpha))
                .with_width(glow_width));
        }

        frame.stroke(&path, Stroke::default()
            .with_color(Color::from_rgba(0.4, 1.0, 0.8, border_alpha))
            .with_width(2.0)); 
    }

    let base_font_size = (target_rect.height / 30.0).clamp(10.0, 18.0); 
    let font_size = base_font_size * cos_a.max(0.3);
    let line_height = font_size * 1.5;
    
    let margin_x = (w * 0.05).clamp(5.0, 20.0);
    let margin_y = (h * 0.05).clamp(10.0, 30.0);
    
    let flip_alpha = (cos_a * 2.5).clamp(0.0, 1.0);
    let text_alpha = flip_alpha; 

    if text_alpha > 0.0 {
        if border_alpha > 0.0 {
            let top_y = p1.y + margin_y + font_size;
            frame.fill_text(cosmic::iced::widget::canvas::Text {
                content: "SYSTEM: NEURAL_LINK ACTIVE".to_string(),
                position: Point::new(p1.x + margin_x, top_y),
                color: Color::from_rgba(0.4, 1.0, 0.8, border_alpha * flip_alpha),
                size: Pixels(font_size),
                ..Default::default()
            });
            
            let box_w = 120.0 * (target_rect.width / 1126.0) * cos_a;
            frame.fill_rectangle(
                Point::new(p2.x - box_w - margin_x, p2.y + margin_y),
                Size::new(box_w, font_size * 1.5),
                Color::from_rgba(0.4, 1.0, 0.8, 0.2 * border_alpha * flip_alpha)
            );

            if let Some(controls) = params.window_controls {
                // We need to update button positions based on current 3D geometry
                // For simplicity, we draw them in the projected space
                // This is a bit tricky with the current Button::draw which expects absolute coords.
                // However, since we are in the canvas frame, we can just draw them.
                controls.draw(frame, border_alpha * flip_alpha);
            }
        }

        if let Ok(grid) = grid_mutex.lock() {
            let start_y = p1.y + margin_y + (font_size * 2.0);
            
            for y in 0..grid.rows {
                let current_y = start_y + (y as f32 * line_height);
                
                if current_y > p4.y - margin_y {
                    break;
                }
                
                if let Some(row) = grid.get_visible_row(y) {
                    for x in 0..grid.cols {
                        if x >= row.len() { break; }
                        let cell = row[x];
                        if cell.char == ' ' && cell.bg == Color::TRANSPARENT {
                            continue;
                        }
                    
                        let char_width = font_size * 0.6;
                        let pos = Point::new(p1.x + margin_x + (x as f32 * char_width), current_y);
                        
                        if cell.bg != Color::TRANSPARENT {
                            let mut bg_c = cell.bg;
                            bg_c.a *= text_alpha;
                            frame.fill_rectangle(
                                Point::new(pos.x, pos.y - font_size),
                                Size::new(char_width, line_height),
                                bg_c
                            );
                        }
                        
                        if cell.char != ' ' {
                            let mut fg_c = cell.fg;
                            fg_c.a *= text_alpha;
                            frame.fill_text(cosmic::iced::widget::canvas::Text {
                                content: cell.char.to_string(),
                                position: pos,
                                color: fg_c,
                                size: Pixels(font_size),
                                font: cosmic::iced::Font::MONOSPACE,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            
            if params.cursor_visible && border_alpha > 0.0 {
                let total_lines = grid.scrollback.len() + grid.rows;
                let start_index = total_lines.saturating_sub(grid.rows + grid.viewport_offset);
                let abs_cursor_y = grid.scrollback.len() + grid.cursor_y;
                
                if abs_cursor_y >= start_index && abs_cursor_y < start_index + grid.rows {
                    let screen_y = abs_cursor_y - start_index;
                    let current_y = start_y + (screen_y as f32 * line_height);
                    
                    if current_y <= p4.y - margin_y {
                        let char_width = font_size * 0.6;
                        let cursor_pos = Point::new(
                            p1.x + margin_x + (grid.cursor_x as f32 * char_width),
                            current_y
                        );
                        
                        frame.fill_rectangle(
                            Point::new(cursor_pos.x, cursor_pos.y - font_size),
                            Size::new(char_width, line_height),
                            Color::from_rgba(0.4, 1.0, 0.8, text_alpha * 0.8)
                        );
                    }
                }
            }
        }
    }
}
