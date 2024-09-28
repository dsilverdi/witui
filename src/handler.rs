use std::io;

use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers, KeyCode};

use crate::app::App;

pub fn handle_key_event(key_event: KeyEvent, app: &mut App) -> io::Result<()>{
    if key_event.kind != KeyEventKind::Press {
        return Ok(())
    }

    match (key_event.code, key_event.modifiers) {
        // search mode
        (KeyCode::Char(':'), KeyModifiers::CONTROL) => {},

        // quit app
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => app.quit(),
        (KeyCode::Char('q'),_) => app.quit(),
        _ => unimplemented!()
    }
    Ok(())
}