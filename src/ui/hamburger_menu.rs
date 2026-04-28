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

        menu.push(MenuItem { label: "⌨ Shortcuts", subtitle: "Keybindings & Aliases", action: MenuAction::ShowShortcuts });
        
        menu
    }
}
