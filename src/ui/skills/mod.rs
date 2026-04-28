pub mod settings;
pub mod physics;
pub mod themes;
pub mod a11y;
pub mod security;

use crate::ui::skill::TerminalSkill;

pub fn get_all_skills() -> Vec<Box<dyn TerminalSkill>> {
    vec![
        Box::new(settings::SettingsSkill),
        Box::new(physics::PhysicsSkill),
        Box::new(themes::ThemesSkill),
        Box::new(a11y::A11ySkill),
        Box::new(security::SecuritySkill),
    ]
}
