//! Minimal shared FrankenTUI Model/App for Croft (TTY) and Loft (Web).
//!
//! Only the display name and render backend differ between modes.

use ftui::core::event::Event;
use ftui::core::geometry::Rect;
use ftui::render::frame::Frame;
use ftui::runtime::{App, Cmd, Model, ScreenMode};
use ftui::widgets::block::Block;
use ftui::widgets::paragraph::Paragraph;
use ftui::widgets::Widget;

/// Shared min UI: bordered viewport + press-`q`-to-quit.
pub struct HelloTick {
    ticks: u64,
    title: &'static str,
}

impl HelloTick {
    pub fn croft() -> Self {
        Self {
            ticks: 0,
            title: "Croft",
        }
    }

    pub fn loft() -> Self {
        Self {
            ticks: 0,
            title: "Loft",
        }
    }

    /// Display name shown in the bordered title (`"Croft"` / `"Loft"`).
    pub fn title(&self) -> &'static str {
        self.title
    }

    /// Build an App ready for interactive TTY `run()` (does not enter the loop).
    pub fn croft_app() -> ftui::runtime::AppBuilder<Self> {
        App::new(Self::croft()).screen_mode(ScreenMode::Inline { ui_height: 8 })
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
        let area = Rect::new(0, 0, frame.width(), frame.height());
        let block = Block::bordered().title(self.title);
        let inner = block.inner(area);
        block.render(area, frame);
        let text = format!(
            "McLoad {title} — press 'q' to quit  (ticks={ticks})",
            title = self.title,
            ticks = self.ticks
        );
        Paragraph::new(text).render(inner, frame);
    }
}
