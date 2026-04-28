pub mod settings;
pub mod physics;
pub mod themes;

use crate::ui::skill::TerminalSkill;

pub fn get_all_skills() -> Vec<Box<dyn TerminalSkill>> {
    vec![
        Box::new(settings::SettingsSkill),
        Box::new(physics::PhysicsSkill),
        Box::new(themes::ThemesSkill),
    ]
}
