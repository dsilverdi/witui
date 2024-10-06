use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use tokio::sync::mpsc;
use tokio::time::interval;
use std::time::Duration;

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

pub struct EventHandler {
    rx: mpsc::Receiver<Event>
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(10);
        tokio::spawn(async move {
            let tick_rate = Duration::from_millis(250);
            loop {
                if event::poll(tick_rate).unwrap() {
                    if let Ok(event) = event::read() {
                        match event {
                            CrosstermEvent::Key(e) => tx.send(Event::Key(e)).await,
                            CrosstermEvent::Mouse(e) => tx.send(Event::Mouse(e)).await,
                            CrosstermEvent::Resize(w, h) => tx.send(Event::Resize(w, h)).await,
                            _ => Ok(()),
                        }.unwrap();
                    }
                }
            }
        });

        EventHandler{
            rx
        }
    }

    pub async fn next(&mut self) -> Option<Event>{
        self.rx.recv().await
    }
}