use cosmic::iced::Color;

/// Zentrale Design-Tokens für das 3DNTerminal
pub struct Style;

impl Style {
    // Neon-Farben
    pub const NEON_CYAN: Color = Color::from_rgb(0.0, 1.0, 0.8);
    pub const NEON_ORANGE: Color = Color::from_rgb(1.0, 0.6, 0.0);
    pub const NEON_YELLOW: Color = Color::from_rgb(1.0, 0.8, 0.2);
    
    // Hintergrund & Glassmorphism
    pub const BG_DARK: Color = Color::from_rgb(0.02, 0.02, 0.05);
    pub const BG_DIM: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.4);
    
    // Abstände & Größen
    pub const MARGIN_X: f32 = 10.0;
    pub const MARGIN_Y: f32 = 8.0;
    pub const HEADER_FONT_SIZE: f32 = 14.0;
    pub const TERMINAL_FONT_SIZE: f32 = 13.0;
    pub const BUTTON_SIZE: f32 = 28.0;
    
    // Grid-Metriken
    pub const CHAR_WIDTH: f32 = 8.0;
    pub const LINE_HEIGHT: f32 = 16.0;
}
