#[derive(Debug, Clone, Default)]
pub struct HamburgerMenu {
    pub is_open: bool,
    pub animation_t: f32, // 0.0 = geschlossen, 1.0 = offen
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    OpenSettings,
    TogglePhysics,
    OpenThemePicker,
    NewTab,
    SearchOutput,
    ShowShortcuts,
}

pub struct MenuItem {
    pub label: &'static str,
    pub subtitle: &'static str,
    pub action: MenuAction,
}

impl HamburgerMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items() -> Vec<MenuItem> {
        vec![
            MenuItem { label: "⚙ Settings", subtitle: "Modulare Engine Config", action: MenuAction::OpenSettings },
            MenuItem { label: "⚛ Physics", subtitle: "Toggle breathing & effects", action: MenuAction::TogglePhysics },
            MenuItem { label: "🎨 Themes", subtitle: "Amber / Magenta / Cobalt", action: MenuAction::OpenThemePicker },
            MenuItem { label: "➕ New Tab", subtitle: "Ollama Chat / System Monitor", action: MenuAction::NewTab },
            MenuItem { label: "🔍 Search", subtitle: "Search Terminal Output", action: MenuAction::SearchOutput },
            MenuItem { label: "⌨ Shortcuts", subtitle: "Keybindings & Aliases", action: MenuAction::ShowShortcuts },
        ]
    }
}
