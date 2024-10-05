use std::io;

use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers, KeyCode};

use crate::app::{App, AppState};

pub fn handle_key_event(key_event: KeyEvent, app: &mut App) -> io::Result<()>{
    if key_event.kind != KeyEventKind::Press {
        return Ok(())
    }

    if app.state == AppState::Search {
        if !app.is_loading {
            match key_event.code {
                KeyCode::Backspace => {
                    app.delete_char_input();
                }
                KeyCode::Char(q) => {
                    app.input(q);
                }
                KeyCode::Enter => {
                    //app.scrape_page();
                    app.set_loading();
                    app.publish_scrape_task();
                }
    
                // exit state
                KeyCode::Esc => {
                    app.back_state();
                    app.input.clear();
                }
               
                _ => {}
            }
        }
    }else{
        match (key_event.code, key_event.modifiers) {
            // search mode
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                if app.state != AppState::Search {
                    app.set_state(AppState::Search);
                } 
            },
    
            // quit app
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => app.quit(),
            (KeyCode::Char('q'),_) => {
                if app.state != AppState::Search {
                    app.quit();
                }
            },
            (KeyCode::Backspace,_) => {
                if app.state == AppState::SearchResult || app.state == AppState::Article {
                    app.set_state(AppState::Init);
                }
            },
            (KeyCode::Esc,_) => {
                if app.state == AppState::SearchResult || app.state == AppState::Article {
                    app.set_state(AppState::Init);
                }
            }        
            _ => {}
        }
    }

   
    Ok(())
}