use cosmic::iced::Point;

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

    pub fn hit_test(&self, pos: Point, left_anchor: Point, menu_w: f32) -> bool {
        if !self.is_open { return false; }
        pos.x >= left_anchor.x && pos.x <= left_anchor.x + menu_w
    }

    pub fn on_click(&self, pos: Point, left_anchor: Point, menu_h: f32, skills: &[Box<dyn crate::ui::skill::TerminalSkill>]) -> Option<MenuAction> {
        if !self.is_open { return None; }
        let menu_x = left_anchor.x + 5.0;
        let menu_y = left_anchor.y + 45.0;
        let menu_w = 280.0;

        if pos.x >= menu_x && pos.x <= menu_x + menu_w {
            let rel_y = pos.y - menu_y;
            if rel_y >= 0.0 && rel_y <= menu_h {
                let index = (rel_y / 60.0) as usize;
                let items = Self::items(skills);
                if let Some(item) = items.get(index) {
                    return Some(item.action);
                }
            }
        }
        None
    }
}
