use std::time::{Duration, Instant};
use tiny_skia::{Color, Pixmap, PixmapMut};
use super::{create_effect, Effect, Mode};

const MODE_DURATION: Duration = Duration::from_millis(8000);
const FADE_DURATION: f32      = 2000.0;

pub struct CrossfadeManager {
    current_mode:   Mode,
    next_mode:      Mode,
    current_effect: Box<dyn Effect>,
    next_effect:    Box<dyn Effect>,
    buf_a:          Pixmap,
    buf_b:          Pixmap,
    is_fading:      bool,
    fade_alpha:     f32,
    fade_start:     Option<Instant>,
    mode_start:     Instant,
    width:          u32,
    height:         u32,
}

impl CrossfadeManager {
    pub fn new(width: u32, height: u32) -> Self {
        let current_mode = Mode::Neural;
        let next_mode    = current_mode.next();
        Self {
            current_effect: create_effect(current_mode, width, height),
            next_effect:    create_effect(next_mode,    width, height),
            current_mode,
            next_mode,
            buf_a:      Pixmap::new(width, height).expect("Pixmap A"),
            buf_b:      Pixmap::new(width, height).expect("Pixmap B"),
            is_fading:  false,
            fade_alpha: 0.0,
            fade_start: None,
            mode_start: Instant::now(),
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width  = width;
        self.height = height;
        self.buf_a  = Pixmap::new(width, height).expect("Resize A");
        self.buf_b  = Pixmap::new(width, height).expect("Resize B");
        self.current_effect.build(width, height);
        self.next_effect.build(width, height);
    }

    pub fn tick(&mut self, dt: f32, output: &mut PixmapMut<'_>) {
        if !self.is_fading && self.mode_start.elapsed() >= MODE_DURATION {
            self.is_fading  = true;
            self.fade_start = Some(Instant::now());
            self.next_effect.build(self.width, self.height);
        }

        {
            let mut a = self.buf_a.as_mut();
            self.current_effect.draw(&mut a, dt);
        }

        if self.is_fading {
            let elapsed = self.fade_start
                .map(|s| s.elapsed().as_millis() as f32)
                .unwrap_or(0.0);
            self.fade_alpha = (elapsed / FADE_DURATION).min(1.0);

            {
                let mut b = self.buf_b.as_mut();
                self.next_effect.draw(&mut b, dt);
            }

            if self.fade_alpha >= 1.0 {
                self.current_mode   = self.next_mode;
                self.next_mode      = self.current_mode.next();
                self.is_fading      = false;
                self.fade_alpha     = 0.0;
                self.fade_start     = None;
                self.mode_start     = Instant::now();
                self.current_effect = create_effect(self.current_mode, self.width, self.height);
                self.next_effect    = create_effect(self.next_mode,    self.width, self.height);
                self.buf_b.as_mut().fill(Color::TRANSPARENT);
            }
        }

        output.fill(Color::TRANSPARENT);
        blit(self.buf_a.data(), output, 1.0);
        if self.is_fading && self.fade_alpha > 0.0 {
            blit(self.buf_b.data(), output, self.fade_alpha);
        }
    }

    pub fn current_mode(&self) -> Mode { self.current_mode }
}

fn blit(src: &[u8], dst: &mut PixmapMut<'_>, alpha: f32) {
    let dst_data  = dst.data_mut();
    let alpha_u8  = (alpha * 255.0) as u32;
    for (s, d) in src.chunks_exact(4).zip(dst_data.chunks_exact_mut(4)) {
        let sa = (s[3] as u32 * alpha_u8) / 255;
        if sa == 0 { continue; }
        let da    = d[3] as u32;
        let out_a = sa + da * (255 - sa) / 255;
        if out_a == 0 { d[0]=0; d[1]=0; d[2]=0; d[3]=0; continue; }
        d[0] = ((s[0] as u32 * sa + d[0] as u32 * da * (255 - sa) / 255) / out_a) as u8;
        d[1] = ((s[1] as u32 * sa + d[1] as u32 * da * (255 - sa) / 255) / out_a) as u8;
        d[2] = ((s[2] as u32 * sa + d[2] as u32 * da * (255 - sa) / 255) / out_a) as u8;
        d[3] = out_a as u8;
    }
}
