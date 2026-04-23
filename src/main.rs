mod effects;
mod terminal;

use std::time::Instant;
use cosmic::{
    app::{Core, Settings, Task},
    iced::{
        Length, Subscription,
        mouse,
        widget::canvas::{self, Cache, Canvas, Frame, Geometry},
        Rectangle,
    },
    Application, Element, Theme,
    widget::container,
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
    cache:     Cache,
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
            frame.fill_rectangle(
                cosmic::iced::Point::ORIGIN,
                bounds.size(),
                cosmic::iced::Color::from_rgb(7.0/255.0, 7.0/255.0, 18.0/255.0),
            );
            let mode_text = format!("Modus: {:?}", self.crossfade.current_mode());
            frame.fill_text(canvas::Text {
                content: mode_text,
                position: cosmic::iced::Point::new(20.0, 20.0),
                color: cosmic::iced::Color::from_rgb(6.0/255.0, 182.0/255.0, 212.0/255.0),
                size: cosmic::iced::Pixels(16.0),
                ..canvas::Text::default()
            });
        });
        vec![geometry]
    }
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
            cache:     Cache::new(),
        };
        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.last_tick = Instant::now();
                self.cache.clear();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        Element::from(canvas)
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
