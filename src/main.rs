use std::io;

use witui::app::App;
use witui::event::{EventHandler, Event};
use witui::handler::handle_key_event;
use witui::tui::Tui;

fn main() -> io::Result<()> {
    let mut app: App = App::default();

    let terminal = ratatui::init();
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);

    tui.init()?;
    while app.running {
        tui.draw()?;

        match tui.events.next()? {
            Event::Key(key_event) => handle_key_event(key_event, &mut app)?,
            Event::Mouse(_) | Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
