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
    ExecuteSkill(&'static str),
    ChangeTheme(crate::config::TerminalTheme),
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

    pub fn items(skills: &[Box<dyn crate::ui::skill::TerminalSkill>]) -> Vec<MenuItem> {
        let mut menu = Vec::new();
        
        for skill in skills {
            menu.push(MenuItem {
                label: skill.label(),
                subtitle: skill.subtitle(),
                action: MenuAction::ExecuteSkill(skill.id()),
            });
        }
        
        // Add hardcoded global actions at the end
        menu.push(MenuItem { label: "➕ New Tab", subtitle: "Ollama Chat / System Monitor", action: MenuAction::NewTab });
        menu.push(MenuItem { label: "🔍 Search", subtitle: "Search Terminal Output", action: MenuAction::SearchOutput });
        
        // Themes
        menu.push(MenuItem { label: "🎬 BladeRunner", subtitle: "Teal & Red Neon / High Glow", action: MenuAction::ChangeTheme(crate::config::TerminalTheme::BladeRunner) });
        menu.push(MenuItem { label: "🍏 Apple Glass", subtitle: "Clean & Transparent / Minimal", action: MenuAction::ChangeTheme(crate::config::TerminalTheme::AppleGlass) });
        menu.push(MenuItem { label: "🌌 Deep Space", subtitle: "Dark Purple / Heavy Blur", action: MenuAction::ChangeTheme(crate::config::TerminalTheme::DeepSpace) });
        menu.push(MenuItem { label: "📟 Retro Amber", subtitle: "Classic Phosphor / CRT Vibe", action: MenuAction::ChangeTheme(crate::config::TerminalTheme::RetroAmber) });

        menu.push(MenuItem { label: "⌨ Shortcuts", subtitle: "Keybindings & Aliases", action: MenuAction::ShowShortcuts });
        
        menu
    }
}
