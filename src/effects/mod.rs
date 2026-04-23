pub mod neural;
pub mod circuit;
pub mod binary_rain;
pub mod crossfade;

use tiny_skia::PixmapMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Neural     = 0,
    Circuit    = 1,
    BinaryRain = 2,
}

impl Mode {
    pub fn next(self) -> Self {
        match self {
            Mode::Neural     => Mode::Circuit,
            Mode::Circuit    => Mode::BinaryRain,
            Mode::BinaryRain => Mode::Neural,
        }
    }
}

pub trait Effect: Send {
    fn build(&mut self, width: u32, height: u32);
    fn draw(&mut self, pixmap: &mut PixmapMut<'_>, dt: f32);
}

pub fn create_effect(mode: Mode, width: u32, height: u32) -> Box<dyn Effect> {
    let mut effect: Box<dyn Effect> = match mode {
        Mode::Neural     => Box::new(neural::NeuralEffect::new()),
        Mode::Circuit    => Box::new(circuit::CircuitEffect::new()),
        Mode::BinaryRain => Box::new(binary_rain::BinaryRainEffect::new()),
    };
    effect.build(width, height);
    effect
}
