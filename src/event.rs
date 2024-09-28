use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::mpsc;
use std::time;
use std::thread;
use std::io;

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
        let tick_rate = time::Duration::from_millis(250);
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                if event::poll(tick_rate).expect("no available event") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) => tx.send(Event::Key(e)),
                        CrosstermEvent::Mouse(e) => tx.send(Event::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => tx.send(Event::Resize(w, h)),
                        _ => unimplemented!(),
                    }.expect("failed to send event");
                }
            }
        });

        EventHandler{
            rx
        }
    }

    pub fn next(&self) -> io::Result<Event>{
        Ok(self.rx.recv().expect("error receive event"))
    }
}