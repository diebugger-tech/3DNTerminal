use cosmic::iced::Color;
use cosmic::iced::keyboard::{Key, key::Named};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::env;
use crate::error::AppError;
use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicsConfig {
    pub breathe: bool,
    pub magnetic: bool,
    pub reduce_motion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VisualsConfig {
    pub enabled: bool,
    pub glow_intensity: f32,
    pub shell_alpha: f32,
}

impl Default for VisualsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            glow_intensity: 1.0,
            shell_alpha: 0.95,
        }
    }
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            breathe: false,
            magnetic: false,
            reduce_motion: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum ColorFilter {
    #[default] None,
    Protanopia,
    Deuteranopia,
    Tritanopia,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct A11yConfig {
    pub tremor_damping: f32, // 0.0 to 1.0
    pub color_filter: ColorFilter,
    pub reduce_motion: f32,  // 0.0 to 1.0 (override for speed/physics)
}

impl Default for A11yConfig {
    fn default() -> Self {
        Self {
            tremor_damping: 0.0,
            color_filter: ColorFilter::None,
            reduce_motion: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum TerminalTheme {
    #[default]
    Classic,
    NeonCyber,
    AppleGlass,
    DeepSpace,
    RetroAmber,
    BladeRunner,
    Transparent,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub max_scrollback: usize,
    pub blink_interval: u64,
    pub neon_color: Color,
    pub flip_key: Key,
    pub font_size: f32,
    pub physics: PhysicsConfig,
    pub a11y: A11yConfig,
    pub theme: TerminalTheme,
    pub visuals: VisualsConfig,
    pub power_user_mode: bool,
    pub glow_active: bool,
    pub saved_width: f32,
    pub saved_height: f32,
    pub theme_intensities: [f32; 7],
}

impl TerminalTheme {
    pub fn color(&self) -> Color {
        match self {
            Self::Classic => Color::from_rgba(0.0, 1.0, 0.4, 1.0), // Matrix Green
            Self::NeonCyber => Color::from_rgba(0.0, 1.0, 0.8, 1.0),
            Self::AppleGlass => Color::from_rgba(0.8, 0.8, 1.0, 1.0),
            Self::DeepSpace => Color::from_rgba(0.5, 0.2, 1.0, 1.0),
            Self::RetroAmber => Color::from_rgba(1.0, 0.6, 0.0, 1.0),
            Self::BladeRunner => Color::from_rgba(0.0, 1.0, 0.83, 1.0), // Starkes Teal
            Self::Transparent => Color::from_rgba(1.0, 1.0, 1.0, 0.5),
        }
    }

    pub fn bg_alpha(&self) -> f32 {
        match self {
            Self::AppleGlass => 0.1,
            Self::DeepSpace => 0.4,
            Self::BladeRunner => 0.15,
            Self::Transparent => 0.02, // Fast komplett durchsichtig
            _ => 0.25,
        }
    }

    pub fn glow_intensity(&self) -> f32 {
        match self {
            Self::BladeRunner => 3.5,
            Self::DeepSpace => 2.5,
            Self::AppleGlass => 1.2,
            Self::Transparent => 0.2,
            _ => 1.5,
        }
    }

    pub fn corner_radius(&self) -> f32 {
        match self {
            Self::Classic => 0.0,
            Self::AppleGlass => 12.0,
            Self::DeepSpace => 0.0,
            Self::RetroAmber => 2.0,
            Self::NeonCyber => 0.0,
            Self::BladeRunner => 4.0,
            Self::Transparent => 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    pub max_scrollback: Option<usize>,
    pub blink_interval: Option<u64>,
    pub font_size: Option<f32>,
    pub flip_key_name: Option<String>,
    pub neon_color_rgba: Option<[f32; 4]>,
    pub physics: Option<PhysicsConfig>,
    pub a11y: Option<A11yConfig>,
    pub theme: Option<TerminalTheme>,
    pub visuals: Option<VisualsConfig>,
    pub power_user_mode: Option<bool>,
    pub glow_active: Option<bool>,
    pub theme_intensities: Option<[f32; 7]>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_scrollback: DEFAULT_SCROLLBACK,
            blink_interval: DEFAULT_BLINK_MS,
            neon_color: Color::from_rgba(
                DEFAULT_NEON_COLOR[0], DEFAULT_NEON_COLOR[1], DEFAULT_NEON_COLOR[2], DEFAULT_NEON_COLOR[3]
            ),
            flip_key: Key::Named(Named::F12),
            font_size: DEFAULT_FONT_SIZE,
            physics: PhysicsConfig::default(),
            a11y: A11yConfig::default(),
            theme: TerminalTheme::NeonCyber,
            visuals: VisualsConfig::default(),
            power_user_mode: false,
            glow_active: true,
            saved_width: 800.0,
            saved_height: 600.0,
            theme_intensities: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
        }
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn max_scrollback(mut self, limit: usize) -> Self {
        self.config.max_scrollback = limit;
        self
    }

    pub fn blink_interval(mut self, ms: u64) -> Self {
        self.config.blink_interval = ms;
        self
    }

    pub fn neon_color(mut self, color: Color) -> Self {
        self.config.neon_color = color;
        self
    }

    pub fn flip_key(mut self, key: Key) -> Self {
        self.config.flip_key = key;
        self
    }
    
    pub fn font_size(mut self, size: f32) -> Self {
        self.config.font_size = size;
        self
    }

    pub fn validate(&self) -> Result<(), AppError> {
        if self.config.max_scrollback == 0 || self.config.max_scrollback > 100_000 {
            return Err(AppError::Config("max_scrollback must be between 1 and 100,000".into()));
        }
        if self.config.blink_interval < 50 {
            return Err(AppError::Config("blink_interval must be at least 50ms".into()));
        }
        Ok(())
    }

    pub fn build(self) -> Result<Config, AppError> {
        self.validate()?;
        Ok(self.config)
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn load(path: &str) -> Result<Self, AppError> {
        let mut builder = Self::builder();
        
        // 1. Try file
        if Path::new(path).exists() {
            let content = fs::read_to_string(path)?;
            let parsed: ConfigFile = toml::from_str(&content)
                .map_err(|e| AppError::Config(format!("Failed to parse TOML: {}", e)))?;
                
            if let Some(ms) = parsed.max_scrollback { builder = builder.max_scrollback(ms); }
            if let Some(bi) = parsed.blink_interval { builder = builder.blink_interval(bi); }
            if let Some(fs) = parsed.font_size { builder = builder.font_size(fs); }
            if let Some(rgba) = parsed.neon_color_rgba {
                builder = builder.neon_color(Color::from_rgba(rgba[0], rgba[1], rgba[2], rgba[3]));
            }
            if let Some(key_str) = parsed.flip_key_name {
                if key_str == "F12" { builder = builder.flip_key(Key::Named(Named::F12)); }
                // Expand key mapping later
            }
            if let Some(p) = parsed.physics { builder.config.physics = p; }
            if let Some(a) = parsed.a11y { builder.config.a11y = a; }
            if let Some(th) = parsed.theme { 
                builder.config.theme = th;
                builder.config.neon_color = th.color();
                // Apply saved intensity on load
                let idx = match th {
                    TerminalTheme::Classic => 0,
                    TerminalTheme::NeonCyber => 1,
                    TerminalTheme::AppleGlass => 2,
                    TerminalTheme::DeepSpace => 3,
                    TerminalTheme::RetroAmber => 4,
                    TerminalTheme::BladeRunner => 5,
                    TerminalTheme::Transparent => 6,
                };
                builder.config.visuals.glow_intensity = builder.config.theme_intensities[idx] * th.glow_intensity();
            }
            if let Some(v) = parsed.visuals { builder.config.visuals = v; }
            if let Some(p) = parsed.power_user_mode { builder.config.power_user_mode = p; }
            if let Some(ti) = parsed.theme_intensities { builder.config.theme_intensities = ti; }
        }
        
        // 2. Try Env-Vars
        if let Ok(val) = env::var("3DNTERM_MAX_SCROLLBACK") {
            if let Ok(limit) = val.parse::<usize>() {
                builder = builder.max_scrollback(limit);
            }
        }
        
        builder.build()
    }

    pub fn save(&self, path: &str) -> Result<(), AppError> {
        let file = ConfigFile {
            max_scrollback: Some(self.max_scrollback),
            blink_interval: Some(self.blink_interval),
            font_size: Some(self.font_size),
            flip_key_name: Some("F12".to_string()),
            neon_color_rgba: Some([self.neon_color.r, self.neon_color.g, self.neon_color.b, self.neon_color.a]),
            physics: Some(self.physics),
            a11y: Some(self.a11y),
            theme: Some(self.theme),
            visuals: Some(self.visuals),
            power_user_mode: Some(self.power_user_mode),
            glow_active: Some(self.glow_active),
            theme_intensities: Some(self.theme_intensities),
        };
        let toml = toml::to_string(&file)
            .map_err(|e| AppError::Config(format!("Failed to serialize TOML: {}", e)))?;
        fs::write(path, toml)?;
        Ok(())
    }
}
