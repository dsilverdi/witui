use std::io;

use tokio::select;
use witui::app::App;
use witui::event::{EventHandler, Event};
use witui::handler::handle_key_event;
use witui::tracing::initialize_logging;
use witui::tui::Tui;

use ratatui::backend::Backend;

#[tokio::main]
async fn main() -> io::Result<()> {
    initialize_logging().expect("error initialize log");
    
    let mut app: App = App::new();

    let terminal = ratatui::init();
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);

    tui.init()?;
    tracing::info!("application started");
    let runner = run_app(&mut app, &mut tui).await;
    
    runner.and(tui.exit())
}

async fn run_app(app: &mut App, tui: &mut Tui<impl Backend>) -> io::Result<()> {
    loop {
        tui.draw(app)?;
        select! {
            Some(key) = tui.events.next() => {
                match key {
                    Event::Key(key_event) => handle_key_event(key_event, app)?,
                    Event::Mouse(_) | Event::Resize(_, _) => {}
                }
            }
            Some(content) = app.rx.recv() => {
                app.close_loading();
                app.save_app_content(content);
            }
             // graceful shutdown
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)), if !app.running => {
                return Ok(())
            }
        }
    }
}