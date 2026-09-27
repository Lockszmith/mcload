//! Minimal hello-tick Model shared by Croft (TTY) and Loft (Web).

use ftui::core::event::Event;
use ftui::core::geometry::Rect;
use ftui::render::frame::Frame;
use ftui::runtime::{App, Cmd, Model, ScreenMode};
use ftui::widgets::paragraph::Paragraph;
use ftui::widgets::Widget;

/// Tiny demo app: tick counter + quit on `q`.
pub struct HelloTick {
    ticks: u64,
    title: &'static str,
}

impl HelloTick {
    pub fn croft() -> Self {
        Self {
            ticks: 0,
            title: "McLoad Croft",
        }
    }

    pub fn loft() -> Self {
        Self {
            ticks: 0,
            title: "McLoad Loft",
        }
    }

    /// Build an App ready for interactive TTY `run()` (does not enter the loop).
    pub fn croft_app() -> ftui::runtime::AppBuilder<Self> {
        App::new(Self::croft()).screen_mode(ScreenMode::Inline { ui_height: 3 })
    }
}

#[derive(Debug)]
pub enum Msg {
    Tick,
    Quit,
}

impl From<Event> for Msg {
    fn from(e: Event) -> Self {
        match e {
            Event::Key(k) if k.is_char('q') => Msg::Quit,
            _ => Msg::Tick,
        }
    }
}

impl Model for HelloTick {
    type Message = Msg;

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Tick => {
                self.ticks = self.ticks.saturating_add(1);
                Cmd::none()
            }
            Msg::Quit => Cmd::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let text = format!(
            "{} — FrankenTUI hello-tick={}  (press 'q' to quit)",
            self.title, self.ticks
        );
        let area = Rect::new(0, 0, frame.width(), frame.height().min(3));
        Paragraph::new(text).render(area, frame);
    }
}
