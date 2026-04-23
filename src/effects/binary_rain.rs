use rand::Rng;
use tiny_skia::{Color, Paint, PathBuilder, PixmapMut, Rect, Transform};

use super::Effect;

const CHAR_W: f32 = 14.0;
const CHAR_H: f32 = 16.0;

struct Column {
    x:      f32,
    y:      f32,
    speed:  f32,
    chars:  Vec<u8>,
    length: usize,
    reset:  f32,
}

pub struct BinaryRainEffect {
    columns: Vec<Column>,
    width:   u32,
    height:  u32,
}

impl BinaryRainEffect {
    pub fn new() -> Self {
        Self { columns: Vec::new(), width: 0, height: 0 }
    }
}

impl Effect for BinaryRainEffect {
    fn build(&mut self, width: u32, height: u32) {
        self.width  = width;
        self.height = height;
        let mut rng = rand::thread_rng();
        let w = width as f32;
        let h = height as f32;
        let col_count = (w / CHAR_W) as usize;
        self.columns = (0..col_count).map(|i| Column {
            x:      i as f32 * CHAR_W + 4.0,
            y:      -(rng.gen::<f32>() * h),
            speed:  0.04 + rng.gen::<f32>() * 0.07,
            chars:  Vec::new(),
            length: 8 + rng.gen_range(0..18),
            reset:  0.0,
        }).collect();
    }

    fn draw(&mut self, pixmap: &mut PixmapMut<'_>, dt: f32) {
        let w = self.width as f32;
        let h = self.height as f32;

        // Fade-Overlay: rgba(13,11,30,0.18)
        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba(13.0/255.0, 11.0/255.0, 30.0/255.0, 0.18).unwrap_or(Color::TRANSPARENT));
        let mut pb = PathBuilder::new();
        pb.push_rect(Rect::from_xywh(0.0, 0.0, w, h).unwrap());
        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
        }

        let mut rng = rand::thread_rng();

        for col in &mut self.columns {
            if col.reset > 0.0 { col.reset -= dt; continue; }
            col.y += col.speed * dt;
            let head_row = (col.y / CHAR_H) as i32;

            while col.chars.len() <= (head_row + col.length as i32).max(0) as usize {
                col.chars.push(if rng.gen::<bool>() { 1 } else { 0 });
            }

            let start_row = (head_row - col.length as i32).max(0);
            for row in start_row..=head_row {
                let py = row as f32 * CHAR_H;
                if py > h { continue; }
                let dist  = (head_row - row) as f32;
                let frac  = 1.0 - dist / col.length as f32;
                let alpha = frac * frac * 0.85;

                let color = if dist < 1.0 {
                    Color::from_rgba(220.0/255.0, 1.0, 1.0, alpha).unwrap_or(Color::WHITE)
                } else if dist < 3.0 {
                    Color::from_rgba(6.0/255.0, 182.0/255.0, 212.0/255.0, alpha).unwrap_or(Color::TRANSPARENT)
                } else {
                    Color::from_rgba(6.0/255.0, 182.0/255.0, 212.0/255.0, alpha * 0.6).unwrap_or(Color::TRANSPARENT)
                };

                // Zeichen als kleines Rechteck (7x11px)
                let mut pb = PathBuilder::new();
                if let Some(rect) = Rect::from_xywh(col.x, py - 11.0, 7.0, 11.0) {
                    pb.push_rect(rect);
                    if let Some(path) = pb.finish() {
                        paint.set_color(color);
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                    }
                }
            }

            if (head_row - col.length as i32) as f32 * CHAR_H > h {
                col.y      = -(rng.gen::<f32>() * h * 0.4);
                col.chars  = Vec::new();
                col.speed  = 0.04 + rng.gen::<f32>() * 0.07;
                col.length = 8 + rng.gen_range(0..18);
                col.reset  = rng.gen::<f32>() * 4000.0;
            }
        }
    }
}
