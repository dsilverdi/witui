use std::io;

use witui::app::App;
use witui::event::{EventHandler, Event};
use witui::handler::handle_key_event;
use witui::tracing::initialize_logging;
use witui::tui::Tui;
use tokio::runtime::Handle;

#[tokio::main]
async fn main() -> io::Result<()> {
    initialize_logging().expect("error initialize log");
    
    let runtime_handle = Handle::current();
    let mut app: App = App::new(runtime_handle);

    let terminal = ratatui::init();
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);

    tui.init()?;
    tracing::info!("application started");
    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Key(key_event) => handle_key_event(key_event, &mut app)?,
            Event::Mouse(_) | Event::Resize(_, _) => {}
        }

        app.listen_scrape_task();
    }

    tui.exit()?;
    Ok(())
}
