mod logging;

use cosmic::{
    app::{Core, Settings, Task},
    iced::{
        Length, Subscription,
        mouse,
        keyboard,
        Event,
        widget::{
            canvas::{self, Cache, Canvas, Frame, Geometry},
        },
        Rectangle, Point, Size, Color,
    },
    Application, Element, Theme,
    widget::container,
};
use threednterminal::terminal::traits::Terminal;
use threednterminal::{terminal, ui, config, AnimationPhase, CornerPosition};
use threednterminal::ui::overlay::OverlayMode;
use threednterminal::app::events::Message;

use std::time::{Instant, Duration};

// EffectEngine removed as requested - running directly on desktop

pub struct App {
    core: Core,
    cache: Cache,
    // --- Terminal State ---
    terminal_engine: terminal::TerminalEngine,
    config: config::Config,

    // --- Animation State ---
    phase: AnimationPhase,
    progress: f32, // 0.0 to 1.0
    last_update: Instant,
    start_time: Instant,
    
    // --- Input State ---
    cursor_pos: Point,
    cursor_visible: bool,
    
    // Config
    corner_rect: Rectangle,
    center_rect: Rectangle,
    window_width: f32,
    window_height: f32,

    // UI Components
    window_controls: ui::window_controls::WindowControls,
    active_corner: CornerPosition,
    last_corner: CornerPosition,
    is_dragging: bool,
    drag_start_pos: Point,
    hovered_action: Option<ui::window_controls::ButtonAction>,
    hamburger_menu: ui::hamburger_menu::HamburgerMenu,
    notification: Option<(String, Instant)>,
    active_overlay: OverlayMode,
    tabs: Vec<String>,
    active_tab: usize,
    action_flash: f32,
}

impl canvas::Program<Message, Theme> for App {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &cosmic::iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let params = ui::two_d::TerminalParams {
                phase: self.phase,
                progress: self.progress,
                start_time: self.start_time,
                corner_rect: self.corner_rect,
                center_rect: self.center_rect,
                cursor_visible: self.cursor_visible,
                window_controls: Some(&self.window_controls),
                active_corner: self.active_corner,
                cursor_pos: self.cursor_pos,
                physics_mode: self.config.physics_mode,
                hamburger_open: self.hamburger_menu.is_open,
                notification: self.notification.as_ref(),
                active_overlay: self.active_overlay,
                tabs: &self.tabs,
                active_tab: self.active_tab,
                action_flash: self.action_flash,
                neon_color: self.config.neon_color,
            };
            
            let (_, alpha) = ui::two_d::calculate_geometry(&params);
            
            if alpha > 0.0 {
                frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgba(0.02, 0.02, 0.05, alpha));
            }

            ui::two_d::draw(frame, &self.terminal_engine.grid, &params);
        });
        vec![geometry]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: &cosmic::iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        if let Some(pos) = cursor.position_in(bounds) {
            match event {
                cosmic::iced::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    return Some(canvas::Action::publish(Message::CursorMoved(pos)));
                }
                cosmic::iced::Event::Mouse(mouse::Event::ButtonPressed(btn)) => {
                    return Some(canvas::Action::publish(Message::CanvasButtonPressed(*btn, pos)));
                }
                cosmic::iced::Event::Mouse(mouse::Event::ButtonReleased(btn)) => {
                    return Some(canvas::Action::publish(Message::CanvasButtonReleased(*btn, pos)));
                }
                cosmic::iced::Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                    return Some(canvas::Action::publish(Message::CanvasWheelScrolled(*delta)));
                }
                _ => {}
            }
        }
        None
    }
}

impl App {
    fn current_hit_test(&self, pos: Point) -> Option<ui::window_controls::ButtonAction> {
        let params = ui::two_d::TerminalParams {
            phase: self.phase,
            progress: self.progress,
            start_time: self.start_time,
            corner_rect: self.corner_rect,
            center_rect: self.center_rect,
            cursor_visible: false,
            window_controls: None,
            active_corner: self.active_corner,
            cursor_pos: self.cursor_pos,
            physics_mode: self.config.physics_mode,
            hamburger_open: self.hamburger_menu.is_open,
            notification: self.notification.as_ref(),
            active_overlay: self.active_overlay,
            tabs: &self.tabs,
            active_tab: self.active_tab,
            action_flash: self.action_flash,
            neon_color: self.config.neon_color,
        };
        let (rect, _alpha) = ui::two_d::calculate_geometry(&params);
        let btn_size = (rect.width * 0.03).clamp(12.0, 26.0);
        let left_anchor = Point::new(rect.x, rect.y);
        let right_anchor = Point::new(rect.x + rect.width, rect.y);
        
        self.window_controls.hit_test(pos, left_anchor, right_anchor, btn_size)
    }
}

impl Application for App {
    type Executor = cosmic::executor::Default;
    type Flags    = ();
    type Message  = Message;
    const APP_ID: &'static str = "de.diebugger.3dnterminal";

    fn core(&self) -> &Core             { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: ()) -> (Self, Task<Message>) {
        let mut terminal_engine = terminal::TerminalEngine::new(80, 24);
        if let Err(e) = terminal_engine.spawn_shell() {
            eprintln!("Failed to spawn shell: {}", e);
        }
        let config = config::Config::load("~/.config/3dnterminal/config.toml").unwrap_or_default();

        let init_w = 1280.0_f32;
        let init_h = 720.0_f32;
        let active_corner = CornerPosition::BottomRight;
        let corner_rect = active_corner.corner_rect(init_w, init_h);
        let center_rect = Rectangle::new(Point::ORIGIN, Size::new(init_w, init_h));

        let app = App {
            core,
            terminal_engine,
            config,
            last_update: Instant::now(),
            start_time: Instant::now(),
            cursor_pos: Point::ORIGIN,
            cursor_visible: true,
            cache: Cache::new(),
            phase: AnimationPhase::Expanded,
            progress: 1.0,
            corner_rect,
            center_rect,
            window_width: init_w,
            window_height: init_h,
            window_controls: ui::window_controls::WindowControls::new(),
            active_corner: CornerPosition::Free,
            last_corner: CornerPosition::BottomRight,
            is_dragging: false,
            drag_start_pos: Point::ORIGIN,
            hovered_action: None,
            hamburger_menu: ui::hamburger_menu::HamburgerMenu::default(),
            notification: None,
            active_overlay: OverlayMode::None,
            tabs: vec!["Terminal".to_string()],
            active_tab: 0,
            action_flash: 0.0,
        };

        let maximize_task = app.core.maximize(None, true);
        (app, maximize_task)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        if !matches!(message, Message::Tick(_)) {
            tracing::debug!("Update Message: {:?}", message);
        }
        match message {
            Message::Tick(now) => {
                let dt = now.duration_since(self.last_update).as_secs_f32();
                self.last_update = now;
                let mut needs_redraw = false;
                
                // 1. Animation Progress (smooth dt-based)
                if self.phase != AnimationPhase::Expanded && self.phase != AnimationPhase::Collapsed && self.phase != AnimationPhase::Hidden {
                    self.progress = (self.progress + dt / 0.6).min(1.0); // 600ms duration
                    if self.progress >= 1.0 {
                        self.phase = match self.phase {
                            AnimationPhase::Expanding => AnimationPhase::Expanded,
                            AnimationPhase::Collapsing => AnimationPhase::Collapsed,
                            _ => self.phase,
                        };
                    }
                    needs_redraw = true;
                }
                
                // 2. Breathe Animation (nur in kleinen Modi)
                if self.phase == AnimationPhase::Collapsed || self.phase == AnimationPhase::Hidden {
                    needs_redraw = true;
                }

                // 3. Cursor Blinking (500ms Intervall)
                let current_cursor_visible = (now.duration_since(self.start_time).as_millis() / 500) % 2 == 0;
                if self.cursor_visible != current_cursor_visible {
                    self.cursor_visible = current_cursor_visible;
                    needs_redraw = true;
                }

                // 4. Terminal Grid Update
                if let Ok(grid) = self.terminal_engine.grid.lock() {
                    if grid.dirty {
                        needs_redraw = true;
                    }
                }

                // 5. Hamburger Menu Animation (A11Y Check)
                if self.config.physics_mode == crate::config::PhysicsMode::Static {
                    let target_t = if self.hamburger_menu.is_open { 1.0 } else { 0.0 };
                    if (self.hamburger_menu.animation_t - target_t).abs() > 0.001 {
                        self.hamburger_menu.animation_t = target_t;
                        needs_redraw = true;
                    }
                } else {
                    if self.hamburger_menu.is_open && self.hamburger_menu.animation_t < 1.0 {
                        self.hamburger_menu.animation_t = (self.hamburger_menu.animation_t + dt / 0.2).min(1.0); // 200ms
                        needs_redraw = true;
                    } else if !self.hamburger_menu.is_open && self.hamburger_menu.animation_t > 0.0 {
                        self.hamburger_menu.animation_t = (self.hamburger_menu.animation_t - dt / 0.2).max(0.0);
                        needs_redraw = true;
                    }
                }
                
                // 6. Notification expiry (3 seconds)
                if let Some((_, start)) = self.notification {
                    if start.elapsed().as_secs_f32() > 3.0 {
                        self.notification = None;
                        needs_redraw = true;
                    }
                }

                // 7. Action Flash Decay
                if self.action_flash > 0.0 {
                    self.action_flash = (self.action_flash - dt * 2.0).max(0.0);
                    needs_redraw = true;
                }
                
                if needs_redraw {
                    self.cache.clear();
                    // Reset grid dirty flag
                    if let Ok(mut grid) = self.terminal_engine.grid.lock() {
                        grid.dirty = false;
                    }
                }
            }
            Message::ToggleTerminal => {
                let old_phase = self.phase;
                if self.phase == AnimationPhase::Collapsed {
                    self.phase = AnimationPhase::Expanding;
                    self.progress = 0.0;
                } else if self.phase == AnimationPhase::Expanded {
                    self.phase = AnimationPhase::Collapsing;
                    self.progress = 0.0;
                }
                tracing::info!("ToggleTerminal: {:?} -> {:?}", old_phase, self.phase);
                
                self.cache.clear();
            }
            Message::KeyPressed(key, modifiers, _text) => {
                tracing::debug!("KeyPressed: {:?} (mods: {:?})", key, modifiers);
                if key == self.config.flip_key {
                    let old_phase = self.phase;
                    if self.phase == AnimationPhase::Collapsed {
                        self.phase = AnimationPhase::Expanding;
                        self.progress = 0.0;
                    } else if self.phase == AnimationPhase::Expanded {
                        self.phase = AnimationPhase::Collapsing;
                        self.progress = 0.0;
                    }
                    tracing::info!("FlipKey Triggered: {:?} -> {:?}", old_phase, self.phase);
                    self.cache.clear();
                    return Task::none();
                }

                if self.phase == AnimationPhase::Expanded {
                    self.terminal_engine.send_key(&key, modifiers, &self.config);
                }
            }
            Message::Scroll(_) => {}
            Message::WindowResized(w, h) => {
                tracing::info!("WindowResized: {}x{}", w, h);
                self.window_width = w;
                self.window_height = h;
                
                // Rects neu berechnen
                self.center_rect = Rectangle::new(
                    Point::ORIGIN,
                    Size::new(w, h)
                );
                self.corner_rect = self.active_corner.corner_rect(w, h);
                
                self.cache.clear();
                
                // Calculate new grid size based on expanded mode (center_rect)
                let base_font_size = (self.center_rect.height / 30.0).clamp(10.0, 18.0);
                let char_width = base_font_size * 0.6;
                let line_height = base_font_size * 1.5;
                
                let margin_x = (self.center_rect.width * 0.05).clamp(5.0, 20.0);
                let margin_y = (self.center_rect.height * 0.05).clamp(10.0, 30.0);
                
                let usable_width = self.center_rect.width - (margin_x * 2.0);
                let usable_height = self.center_rect.height - (margin_y * 2.0) - (base_font_size * 3.0); // minus header
                
                let cols = (usable_width / char_width).max(20.0) as u16;
                let rows = (usable_height / line_height).max(10.0) as u16;
                
                self.terminal_engine.resize(cols, rows);
                if let Ok(mut grid) = self.terminal_engine.grid.lock() {
                    grid.resize(cols as usize, rows as usize);
                }
                
                self.cache.clear();
            }
            Message::CursorMoved(pos) => {
                self.cursor_pos = pos;
                if self.is_dragging {
                    let delta_x = pos.x - self.drag_start_pos.x;
                    let delta_y = pos.y - self.drag_start_pos.y;
                    self.center_rect.x += delta_x;
                    self.center_rect.y += delta_y;
                    self.drag_start_pos = pos;
                    self.cache.clear();
                } else {
                    let current_action = self.current_hit_test(pos);
                    if current_action != self.hovered_action {
                        self.hovered_action = current_action;
                        self.cache.clear();
                    }
                }
            }
            Message::CanvasButtonPressed(btn, pos) => {
                if btn == mouse::Button::Left {
                    // 0. Check if Overlay is open and handle close
                    if self.active_overlay != OverlayMode::None {
                        let settings_w = 400.0;
                        let settings_h = 320.0;
                        let settings_rect = Rectangle {
                            x: self.center_rect.x + (self.center_rect.width - settings_w) / 2.0,
                            y: self.center_rect.y + (self.center_rect.height - settings_h) / 2.0,
                            width: settings_w,
                            height: settings_h,
                        };
                        
                        // Check for [ X ] button (approximate area)
                        let x_btn_rect = Rectangle {
                            x: settings_rect.x + settings_w - 70.0,
                            y: settings_rect.y + 10.0,
                            width: 60.0,
                            height: 40.0,
                        };
                        
                        if x_btn_rect.contains(pos) || !settings_rect.contains(pos) {
                            self.active_overlay = OverlayMode::None;
                            self.cache.clear();
                            return Task::none();
                        }

                        // Theme Picker Interaction
                        if self.active_overlay == OverlayMode::Themes {
                            let y_start = settings_rect.y + 120.0;
                            for (i, theme) in [
                                crate::config::TerminalTheme::Amber,
                                crate::config::TerminalTheme::Magenta,
                                crate::config::TerminalTheme::Cyan,
                                crate::config::TerminalTheme::Green,
                            ].iter().enumerate() {
                                let chip_x = settings_rect.x + 20.0 + (i as f32 * 90.0);
                                let chip_rect = Rectangle::new(Point::new(chip_x, y_start), Size::new(80.0, 30.0));
                                if chip_rect.contains(pos) {
                                    self.config.theme = *theme;
                                    self.config.neon_color = theme.color();
                                    self.action_flash = 1.0;
                                    self.cache.clear();
                                    return Task::none();
                                }
                            }
                        }
                        return Task::none();
                    }

                    // 0.5 Check Tab Bar
                    let params = ui::two_d::TerminalParams {
                        phase: self.phase,
                        progress: self.progress,
                        start_time: self.start_time,
                        corner_rect: self.corner_rect,
                        center_rect: self.center_rect,
                        cursor_visible: false,
                        window_controls: None,
                        active_corner: self.active_corner,
                        cursor_pos: pos,
                        physics_mode: self.config.physics_mode,
                        hamburger_open: self.hamburger_menu.is_open,
                        notification: self.notification.as_ref(),
                        active_overlay: self.active_overlay,
                        tabs: &self.tabs,
                        active_tab: self.active_tab,
                        action_flash: self.action_flash,
                        neon_color: self.config.neon_color,
                    };
                    let (rect, _) = ui::two_d::calculate_geometry(&params);
                    
                    if pos.y >= rect.y + 8.0 && pos.y <= rect.y + 35.0 {
                        // In Tab Bar height
                        let mut current_x = rect.x + 100.0; // Offset for menu buttons
                        for (i, _) in self.tabs.iter().enumerate() {
                            let tab_w = 100.0;
                            let tab_rect = Rectangle::new(Point::new(current_x, rect.y + 8.0), Size::new(tab_w, 25.0));
                            
                            // Check Close Button [x] on tab
                            let close_rect = Rectangle::new(Point::new(current_x + tab_w - 20.0, rect.y + 10.0), Size::new(15.0, 15.0));
                            if close_rect.contains(pos) && self.tabs.len() > 1 {
                                self.tabs.remove(i);
                                if self.active_tab >= self.tabs.len() {
                                    self.active_tab = self.tabs.len() - 1;
                                }
                                self.cache.clear();
                                return Task::none();
                            }

                            if tab_rect.contains(pos) {
                                self.active_tab = i;
                                self.cache.clear();
                                return Task::none();
                            }
                            current_x += tab_w + 5.0;
                        }
                    }

                    // 1. Check if Menu is open and hit
                    if self.hamburger_menu.is_open {
                        let params = ui::two_d::TerminalParams {
                            phase: self.phase,
                            progress: self.progress,
                            start_time: self.start_time,
                            corner_rect: self.corner_rect,
                            center_rect: self.center_rect,
                            cursor_visible: false,
                            window_controls: None,
                            active_corner: self.active_corner,
                            cursor_pos: pos,
                            physics_mode: self.config.physics_mode,
                            hamburger_open: true,
                            notification: self.notification.as_ref(),
                            active_overlay: self.active_overlay,
                            tabs: &self.tabs,
                            active_tab: self.active_tab,
                            action_flash: self.action_flash,
                            neon_color: self.config.neon_color,
                        };
                        let (rect, _) = ui::two_d::calculate_geometry(&params);
                        let menu_x = rect.x + 5.0;
                        let menu_y = rect.y + 45.0;
                        let menu_w = 280.0;
                        let menu_h = 420.0;
                        
                        if pos.x >= menu_x && pos.x <= menu_x + menu_w {
                            let rel_y = pos.y - menu_y;
                            if rel_y >= 0.0 && rel_y <= menu_h {
                                let index = (rel_y / 60.0) as usize;
                                let items = ui::hamburger_menu::HamburgerMenu::items();
                                if let Some(item) = items.get(index) {
                                    self.hamburger_menu.is_open = false; // Auto-close
                                    let msg = match item.action {
                                        ui::hamburger_menu::MenuAction::OpenSettings => Message::MenuAction(ui::hamburger_menu::MenuAction::OpenSettings),
                                        ui::hamburger_menu::MenuAction::TogglePhysics => Message::MenuAction(ui::hamburger_menu::MenuAction::TogglePhysics),
                                        ui::hamburger_menu::MenuAction::OpenThemePicker => Message::MenuAction(ui::hamburger_menu::MenuAction::OpenThemePicker),
                                        ui::hamburger_menu::MenuAction::NewTab => Message::NewTab,
                                        ui::hamburger_menu::MenuAction::SearchOutput => Message::MenuAction(ui::hamburger_menu::MenuAction::SearchOutput),
                                        ui::hamburger_menu::MenuAction::ShowShortcuts => Message::MenuAction(ui::hamburger_menu::MenuAction::ShowShortcuts),
                                    };
                                    return Task::done(cosmic::Action::App(msg));
                                }
                            }
                        }
                    }

                    // 2. Check Window Controls
                    if let Some(action) = self.current_hit_test(pos) {
                        return self.execute_button_action(action);
                    } else if self.active_corner == CornerPosition::Free && self.phase == AnimationPhase::Expanded {
                        if self.center_rect.contains(pos) {
                            self.is_dragging = true;
                            self.drag_start_pos = pos;
                        }
                    }
                }
                return Task::none();
            }
            Message::ToggleA11Y => {
                self.config.physics_mode = match self.config.physics_mode {
                    crate::config::PhysicsMode::Static => crate::config::PhysicsMode::Breathe,
                    crate::config::PhysicsMode::Breathe => crate::config::PhysicsMode::Hologram3D,
                    crate::config::PhysicsMode::Hologram3D => crate::config::PhysicsMode::Static,
                };
                self.notification = Some((format!("Physics Mode: {:?}", self.config.physics_mode), Instant::now()));
                self.action_flash = 1.0;
                tracing::info!("Physics: Mode -> {:?}", self.config.physics_mode);
                self.cache.clear();
            }
            Message::MenuAction(action) => {
                match action {
                    ui::hamburger_menu::MenuAction::OpenSettings => {
                        self.active_overlay = if self.active_overlay == OverlayMode::Settings { OverlayMode::None } else { OverlayMode::Settings };
                        self.action_flash = 0.8;
                    }
                    ui::hamburger_menu::MenuAction::OpenThemePicker => {
                        self.active_overlay = if self.active_overlay == OverlayMode::Themes { OverlayMode::None } else { OverlayMode::Themes };
                        self.action_flash = 1.0;
                    }
                    ui::hamburger_menu::MenuAction::TogglePhysics => {
                        self.active_overlay = if self.active_overlay == OverlayMode::Physics { OverlayMode::None } else { OverlayMode::Physics };
                        self.action_flash = 0.8;
                    }
                    ui::hamburger_menu::MenuAction::SearchOutput => {
                        self.active_overlay = if self.active_overlay == OverlayMode::Search { OverlayMode::None } else { OverlayMode::Search };
                        self.action_flash = 0.5;
                    }
                    ui::hamburger_menu::MenuAction::ShowShortcuts => {
                        self.active_overlay = if self.active_overlay == OverlayMode::Shortcuts { OverlayMode::None } else { OverlayMode::Shortcuts };
                        self.action_flash = 0.5;
                    }
                    _ => {
                        self.notification = Some((format!("Action: {:?}", action), Instant::now()));
                    }
                }
                tracing::info!("Menu Action: {:?}", action);
                self.cache.clear();
            }
            Message::NewTab => {
                let tab_name = format!("Session {}", self.tabs.len() + 1);
                self.tabs.push(tab_name.clone());
                self.active_tab = self.tabs.len() - 1;
                self.action_flash = 1.0;
                self.notification = Some((format!("New Tab created: {}", tab_name), Instant::now()));
                tracing::info!("Message: NewTab -> {}", tab_name);
                self.cache.clear();
            }
            Message::ToggleHamburger => {
                self.hamburger_menu.is_open = !self.hamburger_menu.is_open;
                tracing::info!("Message: ToggleHamburger -> {}", self.hamburger_menu.is_open);
                self.cache.clear();
            }
            Message::CanvasButtonReleased(btn, _) => {
                if btn == mouse::Button::Left {
                    if self.is_dragging {
                        self.is_dragging = false;
                    }
                }
                return Task::none();
            }
            Message::MinimizeTerminal => {
                tracing::info!("Message: MinimizeTerminal -> Hidden");
                self.phase = AnimationPhase::Hidden;
                self.is_dragging = false; // Drag abbrechen beim Minimieren
                self.cache.clear();
            }
            Message::MaximizeTerminal => {
                tracing::info!("Message: MaximizeTerminal -> Restore/Maximize");
                if self.phase == AnimationPhase::Hidden || self.phase == AnimationPhase::Collapsed || self.phase == AnimationPhase::Collapsing {
                    // Restore from corner or hidden to center (Full)
                    self.phase = AnimationPhase::Expanding;
                    self.active_corner = CornerPosition::Free;
                } else {
                    // Go from center (Full) back to last corner
                    self.phase = AnimationPhase::Collapsing;
                    self.active_corner = self.last_corner;
                    self.corner_rect = self.active_corner.corner_rect(self.window_width, self.window_height);
                }
                self.progress = 0.0;
                self.cache.clear();
            }
            Message::CloseApp => {
                tracing::info!("Message: CloseApp -> Exit");
                std::process::exit(0);
            }
            Message::SetCorner(pos) => {
                tracing::info!("Message: SetCorner -> {:?}", pos);
                self.is_dragging = false;
                let already_at_corner = self.active_corner == pos
                    && (self.phase == AnimationPhase::Collapsed
                        || self.phase == AnimationPhase::Collapsing);
                if already_at_corner {
                    // Nochmal gleiche Ecke → zurück zur Mitte
                    tracing::info!("SetCorner: toggle → Expanding");
                    self.phase = AnimationPhase::Expanding;
                    self.active_corner = CornerPosition::Free;
                    self.progress = 0.0;
                } else {
                    // Neue Ecke → corner_rect updaten + zur Ecke flippen
                    if self.active_corner != CornerPosition::Free {
                        self.last_corner = self.active_corner;
                    }
                    self.active_corner = pos;
                    self.corner_rect = pos.corner_rect(self.window_width, self.window_height);
                    if self.phase != AnimationPhase::Collapsing {
                        self.phase = AnimationPhase::Collapsing;
                        self.progress = 0.0;
                    }
                    // Wenn bereits Collapsing: corner_rect wurde geupdated, Animation
                    // läuft weiter zur neuen Ecke
                }
                self.cache.clear();
            }
            Message::RestoreLast => {
                tracing::info!("Message: RestoreLast -> {:?}", self.last_corner);
                self.active_corner = self.last_corner;
                self.corner_rect = self.active_corner.corner_rect(self.window_width, self.window_height);
                self.phase = AnimationPhase::Collapsing;
                self.progress = 0.0;
                self.is_dragging = false;
                self.cache.clear();
            }
            Message::TerminalClosed => {}
            Message::StartDragging(_) | Message::DragTo(_) | Message::StopDragging => {}
            Message::MouseClicked(_) => {}
            Message::CanvasWheelScrolled(delta) => {
                if self.phase == AnimationPhase::Expanded {
                    let scroll_val = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => y,
                        mouse::ScrollDelta::Pixels { y, .. } => y.signum(),
                    };
                    if scroll_val > 0.0 {
                        self.terminal_engine.scroll_up(3);
                    } else if scroll_val < 0.0 {
                        self.terminal_engine.scroll_down(3);
                    }
                    self.cache.clear();
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let canvas = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = cosmic::iced::time::every(Duration::from_millis(16)).map(Message::Tick);
        
        let events = cosmic::iced::event::listen_with(|event, _status, _window_id| {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, text, .. }) => {
                    return Some(Message::KeyPressed(key, modifiers, text.map(|t| t.to_string())));
                }
                Event::Window(cosmic::iced::window::Event::Resized(size)) => {
                    return Some(Message::WindowResized(size.width, size.height));
                }
                _ => {}
            }
            None
        });

        Subscription::batch(vec![tick, events])
    }
}

impl App {
    fn execute_button_action(&mut self, action: ui::window_controls::ButtonAction) -> Task<Message> {
        match action {
            ui::window_controls::ButtonAction::Minimize => {
                return self.core.minimize(None);
            }
            ui::window_controls::ButtonAction::Maximize => {
                if self.phase == AnimationPhase::Hidden || self.phase == AnimationPhase::Collapsed || self.phase == AnimationPhase::Collapsing {
                    self.phase = AnimationPhase::Expanding;
                    self.active_corner = CornerPosition::Free;
                } else {
                    self.phase = AnimationPhase::Collapsing;
                    self.active_corner = self.last_corner;
                    self.corner_rect = self.active_corner.corner_rect(self.window_width, self.window_height);
                }
                self.progress = 0.0;
            }
            ui::window_controls::ButtonAction::Close => {
                std::process::exit(0);
            }
            ui::window_controls::ButtonAction::RestoreLast => {
                self.active_corner = self.last_corner;
                self.corner_rect = self.active_corner.corner_rect(self.window_width, self.window_height);
                self.phase = AnimationPhase::Collapsing;
                self.progress = 0.0;
            }
            ui::window_controls::ButtonAction::SetCorner(p) => {
                let already_at_corner = self.active_corner == p
                    && (self.phase == AnimationPhase::Collapsed || self.phase == AnimationPhase::Collapsing);
                if already_at_corner {
                    self.phase = AnimationPhase::Expanding;
                    self.active_corner = CornerPosition::Free;
                    self.progress = 0.0;
                } else {
                    if self.active_corner != CornerPosition::Free {
                        self.last_corner = self.active_corner;
                    }
                    self.active_corner = p;
                    self.corner_rect = p.corner_rect(self.window_width, self.window_height);
                    if self.phase != AnimationPhase::Collapsing {
                        self.phase = AnimationPhase::Collapsing;
                        self.progress = 0.0;
                    }
                }
            }
            ui::window_controls::ButtonAction::Hamburger => {
                return Task::done(cosmic::Action::App(Message::ToggleHamburger));
            }
            ui::window_controls::ButtonAction::NewTab => {
                return Task::done(cosmic::Action::App(Message::NewTab));
            }
        }
        self.cache.clear();
        Task::none()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();
    let settings = Settings::default()
        .size(Size::new(1280.0, 720.0))
        .transparent(true)
        .client_decorations(false);
    cosmic::app::run::<App>(settings, ())?;
    Ok(())
}
