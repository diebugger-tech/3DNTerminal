use rand::Rng;
use tiny_skia::{Color, LineCap, Paint, PathBuilder, PixmapMut, Stroke, Transform};

use super::Effect;

#[derive(Clone)]
struct Segment {
    x1: f32, y1: f32,
    x2: f32, y2: f32,
}

impl Segment {
    fn length(&self) -> f32 {
        (self.x2 - self.x1).abs() + (self.y2 - self.y1).abs()
    }
}

#[derive(Clone)]
struct Trace {
    segments: Vec<Segment>,
    length:   f32,
}

struct Packet {
    trace_idx: usize,
    t:         f32,
    speed:     f32,
    is_orange: bool,
}

pub struct CircuitEffect {
    traces:  Vec<Trace>,
    packets: Vec<Packet>,
    width:   u32,
    height:  u32,
}

impl CircuitEffect {
    pub fn new() -> Self {
        Self { traces: Vec::new(), packets: Vec::new(), width: 0, height: 0 }
    }

    fn point_on_trace(trace: &Trace, t: f32) -> (f32, f32) {
        let target = t * trace.length;
        let mut acc = 0.0_f32;
        for seg in &trace.segments {
            let seg_len = seg.length();
            if acc + seg_len >= target {
                let frac = (target - acc) / seg_len;
                return (seg.x1 + (seg.x2 - seg.x1) * frac, seg.y1 + (seg.y2 - seg.y1) * frac);
            }
            acc += seg_len;
        }
        let last = trace.segments.last().unwrap();
        (last.x2, last.y2)
    }

    fn spawn_packet(&mut self) {
        if self.traces.is_empty() { return; }
        let mut rng = rand::thread_rng();
        self.packets.push(Packet {
            trace_idx: rng.gen_range(0..self.traces.len()),
            t:         rng.gen::<f32>(),
            speed:     0.0004 + rng.gen::<f32>() * 0.0006,
            is_orange: rng.gen::<f32>() >= 0.7,
        });
    }
}

impl Effect for CircuitEffect {
    fn build(&mut self, width: u32, height: u32) {
        self.width   = width;
        self.height  = height;
        self.traces  = Vec::new();
        self.packets = Vec::new();
        let mut rng = rand::thread_rng();
        let w = width as f32;
        let h = height as f32;
        let grid = 60.0_f32;
        let cols = (w / grid).ceil() as i32 + 1;
        let rows = (h / grid).ceil() as i32 + 1;

        for _ in 0..80 {
            let c1 = rng.gen_range(0..cols);
            let r1 = rng.gen_range(0..rows);
            let c2 = rng.gen_range(0..cols);
            let r2 = rng.gen_range(0..rows);
            if c1 == c2 && r1 == r2 { continue; }
            let x1 = c1 as f32 * grid; let y1 = r1 as f32 * grid;
            let x2 = c2 as f32 * grid; let y2 = r2 as f32 * grid;
            let cx = x2; let cy = y1;
            let mut segs = Vec::new();
            if x1 != cx || y1 != cy { segs.push(Segment { x1, y1, x2: cx, y2: cy }); }
            if cx != x2 || cy != y2 { segs.push(Segment { x1: cx, y1: cy, x2, y2 }); }
            if segs.is_empty() { continue; }
            let length: f32 = segs.iter().map(|s| s.length()).sum();
            self.traces.push(Trace { segments: segs, length });
        }
        for _ in 0..18 { self.spawn_packet(); }
    }

    fn draw(&mut self, pixmap: &mut PixmapMut<'_>, dt: f32) {
        pixmap.fill(Color::TRANSPARENT);
        let mut paint = Paint::default();
        let thin = Stroke { width: 1.2, line_cap: LineCap::Round, ..Default::default() };

        for trace in &self.traces {
            for seg in &trace.segments {
                let mut pb = PathBuilder::new();
                pb.move_to(seg.x1, seg.y1);
                pb.line_to(seg.x2, seg.y2);
                if let Some(path) = pb.finish() {
                    paint.set_color(Color::from_rgba(6.0/255.0, 182.0/255.0, 212.0/255.0, 0.18).unwrap_or(Color::TRANSPARENT));
                    pixmap.stroke_path(&path, &paint, &thin, Transform::identity(), None);
                }
                for pt in [(seg.x1, seg.y1), (seg.x2, seg.y2)] {
                    let mut pb = PathBuilder::new();
                    pb.push_circle(pt.0, pt.1, 2.5);
                    if let Some(path) = pb.finish() {
                        paint.set_color(Color::from_rgba(249.0/255.0, 115.0/255.0, 22.0/255.0, 0.35).unwrap_or(Color::TRANSPARENT));
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                    }
                }
            }
        }

        let mut to_respawn = Vec::new();
        for (i, packet) in self.packets.iter_mut().enumerate() {
            packet.t += packet.speed * dt;
            if packet.t > 1.0 { to_respawn.push(i); continue; }
            let trace    = &self.traces[packet.trace_idx];
            let pos      = Self::point_on_trace(trace, packet.t);
            let tail_pos = Self::point_on_trace(trace, (packet.t - 0.08).max(0.0));
            let thick = Stroke { width: 2.5, line_cap: LineCap::Round, ..Default::default() };
            let mut pb = PathBuilder::new();
            pb.move_to(tail_pos.0, tail_pos.1);
            pb.line_to(pos.0, pos.1);
            if let Some(path) = pb.finish() {
                let (r, g, b) = if packet.is_orange { (249.0/255.0, 115.0/255.0, 22.0/255.0) } else { (6.0/255.0, 182.0/255.0, 212.0/255.0) };
                paint.set_color(Color::from_rgba(r, g, b, 0.9).unwrap_or(Color::WHITE));
                pixmap.stroke_path(&path, &paint, &thick, Transform::identity(), None);
            }
            for (ring, alpha) in [(6.0_f32, 0.25_f32), (4.0, 0.5), (2.5, 1.0)] {
                let mut pb = PathBuilder::new();
                pb.push_circle(pos.0, pos.1, ring);
                if let Some(path) = pb.finish() {
                    let (r, g, b) = if packet.is_orange { (249.0/255.0, 115.0/255.0, 22.0/255.0) } else { (6.0/255.0, 182.0/255.0, 212.0/255.0) };
                    paint.set_color(Color::from_rgba(r, g, b, alpha).unwrap_or(Color::WHITE));
                    pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                }
            }
        }
        for i in to_respawn.into_iter().rev() {
            self.packets.remove(i);
            self.spawn_packet();
        }
    }
}
