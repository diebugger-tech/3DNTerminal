use rand::Rng;
use std::f32::consts::PI;
use tiny_skia::{Color, Paint, PathBuilder, PixmapMut, Stroke, Transform};

use super::Effect;

struct Node {
    x: f32, y: f32,
    vx: f32, vy: f32,
    r: f32,
    pulse: f32,
    pulse_speed: f32,
}

pub struct NeuralEffect {
    nodes:  Vec<Node>,
    width:  u32,
    height: u32,
}

impl NeuralEffect {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), width: 0, height: 0 }
    }
}

impl Effect for NeuralEffect {
    fn build(&mut self, width: u32, height: u32) {
        self.width  = width;
        self.height = height;
        let mut rng = rand::thread_rng();
        let w = width as f32;
        let h = height as f32;
        let count = 40.min(((w * h) / 22000.0) as usize + 12);
        self.nodes = (0..count).map(|_| Node {
            x:           rng.gen::<f32>() * w,
            y:           rng.gen::<f32>() * h,
            vx:          (rng.gen::<f32>() - 0.5) * 0.35,
            vy:          (rng.gen::<f32>() - 0.5) * 0.35,
            r:           1.8 + rng.gen::<f32>() * 2.2,
            pulse:       rng.gen::<f32>() * PI * 2.0,
            pulse_speed: 0.015 + rng.gen::<f32>() * 0.025,
        }).collect();
    }

    fn draw(&mut self, pixmap: &mut PixmapMut<'_>, dt: f32) {
        let w = self.width as f32;
        let h = self.height as f32;
        pixmap.fill(Color::TRANSPARENT);

        for n in &mut self.nodes {
            n.x += n.vx;
            n.y += n.vy;
            n.pulse += n.pulse_speed;
            if n.x < 0.0 || n.x > w { n.vx *= -1.0; }
            if n.y < 0.0 || n.y > h { n.vy *= -1.0; }
        }

        let max_dist = w.min(h) * 0.22;
        let mut paint = Paint::default();
        let stroke = Stroke { width: 0.7, ..Default::default() };

        for i in 0..self.nodes.len() {
            for j in (i + 1)..self.nodes.len() {
                let dx = self.nodes[i].x - self.nodes[j].x;
                let dy = self.nodes[i].y - self.nodes[j].y;
                let d  = (dx * dx + dy * dy).sqrt();
                if d < max_dist {
                    let a = (1.0 - d / max_dist) * 0.55;
                    let mut pb = PathBuilder::new();
                    pb.move_to(self.nodes[i].x, self.nodes[i].y);
                    pb.line_to(self.nodes[j].x, self.nodes[j].y);
                    if let Some(path) = pb.finish() {
                        paint.set_color(Color::from_rgba(6.0/255.0, 182.0/255.0, 212.0/255.0, a).unwrap_or(Color::TRANSPARENT));
                        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                    }
                }
            }
        }

        for n in &self.nodes {
            let glow = 0.55 + 0.45 * n.pulse.sin();
            let r    = n.r * (0.85 + 0.3 * glow);

            for ring in 1..=4 {
                let ring_r = r * ring as f32 * 1.1;
                let alpha  = (0.22 * glow) / ring as f32;
                let mut pb = PathBuilder::new();
                pb.push_circle(n.x, n.y, ring_r);
                if let Some(path) = pb.finish() {
                    paint.set_color(Color::from_rgba(6.0/255.0, 182.0/255.0, 212.0/255.0, alpha).unwrap_or(Color::TRANSPARENT));
                    pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                }
            }

            let mut pb = PathBuilder::new();
            pb.push_circle(n.x, n.y, r);
            if let Some(path) = pb.finish() {
                paint.set_color(Color::from_rgba(6.0/255.0, 182.0/255.0, 212.0/255.0, 0.7 + 0.3 * glow).unwrap_or(Color::WHITE));
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
            }
        }
    }
}
