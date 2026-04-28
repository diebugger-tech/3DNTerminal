#[derive(Debug, Clone, Default)]
pub struct HamburgerMenu {
    pub is_open: bool,
    pub animation_t: f32, // 0.0 = geschlossen, 1.0 = offen
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    OpenSettings,
    ToggleA11Y,
    OpenThemePicker,
    NewTab,
    SearchOutput,
    ShareSession,
    ShowShortcuts,
}
