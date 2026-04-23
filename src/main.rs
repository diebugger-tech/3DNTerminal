mod effects;
mod terminal;

use std::time::Instant;
use cosmic::{
    app::{Core, Settings, Task},
    iced::{Length, Subscription},
    widget::container,
    Application, Element,
};
use effects::crossfade::CrossfadeManager;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

pub struct App {
    core:      Core,
    crossfade: CrossfadeManager,
    last_tick: Instant,
}

impl Application for App {
    type Executor = cosmic::executor::Default;
    type Flags    = ();
    type Message  = Message;

    const APP_ID: &'static str = "de.diebugger.threednterminal";

    fn core(&self) -> &Core             { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: ()) -> (Self, Task<Message>) {
        let app = App {
            core,
            crossfade: CrossfadeManager::new(1280, 720),
            last_tick: Instant::now(),
        };
        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => { self.last_tick = Instant::now(); }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let mode = format!("Modus: {:?}", self.crossfade.current_mode());
        let content = cosmic::widget::column::with_children(vec![
            cosmic::widget::text("3DNTerminal 🦀").size(32).into(),
            cosmic::widget::text(mode).size(16).into(),
        ])
        .spacing(12)
        .padding(24);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(
            cosmic::iced::time::Duration::from_millis(16)
        ).map(|_| Message::Tick)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default()
        .size(cosmic::iced::Size::new(1280.0, 720.0));
    cosmic::app::run::<App>(settings, ())?;
    Ok(())
}
