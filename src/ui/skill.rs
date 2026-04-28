use cosmic::iced::{Color, Rectangle, Point};
use cosmic::iced::widget::canvas::Frame;
use crate::ui::two_d::TerminalParams;
use crate::config::Config;

/// Das Interface für alle Terminal-Funktionen (Settings, Physics, Tools)
pub trait TerminalSkill: Send + Sync {
    /// Eindeutige ID des Skills
    fn id(&self) -> &'static str;
    
    /// Anzeigename im Menü
    fn label(&self) -> &'static str;
    
    /// Untertitel/Beschreibung im Menü
    fn subtitle(&self) -> &'static str;
    
    /// Primärfarbe für das UI
    fn color(&self) -> Color;
    
    /// Zeichnet den Inhalt des Overlays
    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams);
    
    /// Zeichnet eine Erweiterung im Hamburger-Menü (z.B. ein Toggle oder Slider)
    fn draw_menu_extension(&self, _frame: &mut Frame, _rect: Rectangle, _alpha: f32, _params: &TerminalParams) {}

    /// Verarbeitet Klicks innerhalb des Overlays
    fn on_click(&self, _pos: Point, _rect: Rectangle, _config: &mut Config) -> bool {
        false
    }

    /// Verarbeitet Klicks auf die Menü-Erweiterung
    fn on_menu_click(&self, _pos: Point, _rect: Rectangle, _config: &mut Config) -> bool {
        false
    }
}
