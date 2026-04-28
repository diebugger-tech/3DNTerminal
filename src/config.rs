use cosmic::iced::Color;
use cosmic::iced::keyboard::{Key, key::Named};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::env;
use crate::error::AppError;
use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Config {
    pub max_scrollback: usize,
    pub blink_interval: u64,
    pub neon_color: Color,
    pub flip_key: Key,
    pub font_size: f32,
    pub physics_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    pub max_scrollback: Option<usize>,
    pub blink_interval: Option<u64>,
    pub font_size: Option<f32>,
    pub flip_key_name: Option<String>,
    pub neon_color_rgba: Option<[f32; 4]>,
    pub physics_enabled: Option<bool>,
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
            physics_enabled: true,
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
            if let Some(pe) = parsed.physics_enabled { builder.config.physics_enabled = pe; }
        }
        
        // 2. Try Env-Vars
        if let Ok(val) = env::var("3DNTERM_MAX_SCROLLBACK") {
            if let Ok(limit) = val.parse::<usize>() {
                builder = builder.max_scrollback(limit);
            }
        }
        
        builder.build()
    }
}
