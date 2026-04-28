use cosmic::iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum OverlayMode {
    #[default]
    None,
    Settings,
    Themes,
    Physics,
    Search,
    Shortcuts,
}

impl OverlayMode {
    pub fn label(&self) -> &str {
        match self {
            Self::None => "",
            Self::Settings => "ENGINE SETTINGS",
            Self::Themes => "THEME PICKER",
            Self::Physics => "PHYSICS ENGINE",
            Self::Search => "SEARCH TERMINAL",
            Self::Shortcuts => "KEYBOARD SHORTCUTS",
        }
    }

    pub fn title_color(&self) -> Color {
        match self {
            Self::Settings | Self::Physics => Color::from_rgba(1.0, 0.6, 0.0, 1.0), // Orange/Amber
            Self::Themes => Color::from_rgba(0.0, 1.0, 0.8, 1.0),                   // Cyan
            Self::Search | Self::Shortcuts => Color::from_rgba(0.0, 0.8, 1.0, 1.0), // Blue
            _ => Color::WHITE,
        }
    }
}
