use crate::scrape::{scrape, http_get};
use std::sync::mpsc;
use std::thread;
use tokio::runtime::Runtime;

const BASE_URL: &str = "https://en.wikipedia.org/wiki";

#[derive(Debug, PartialEq)]
pub enum AppState {
    None,
    Init,
    Search,
    SearchResult, // will render list of wikipedia article
    Article, // render article
    ArticleReference, // render article references
}

pub struct App {
    pub running: bool,
    pub state: AppState,
    pub prev_state: Option<AppState>,
    pub input: String, 
    pub is_loading: bool,
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            running: true,
            is_loading: false,
            state: AppState::Init,
            prev_state: None,
            input: "".to_string(),
            tx,
            rx
        }
    }
}

impl App {
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn set_loading(&mut self) {
        self.is_loading = true;
    }

    pub fn close_loading(&mut self) {
        self.is_loading = false;
    }

    pub fn set_state(&mut self, state: AppState) {
        tracing::info!("[set_state]: {:?}", state);
        let curr_sate = std::mem::replace(&mut self.state, state);
        self.prev_state = Some(curr_sate);
    }

    pub fn back_state(&mut self) {
        if let Some(s) = self.prev_state.take() {
            self.state = s;
            self.prev_state = None;
        }
    }

    pub fn input(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn delete_char_input(&mut self) {
        self.input.pop();
    }

    /// Send and create scraping thread
    pub fn publish_scrape_task(&mut self) {
        let url = BASE_URL.to_string() + self.input.as_str();
        let tx = self.tx.clone();

        tracing::info!("spawn search thread");
        thread::spawn(move || {
            let rt = Runtime::new().expect("failed create runtime");
            let html = rt.block_on(http_get(&url));
            match html {
                Ok(res) =>  tx.send(res).expect("failed to send html result"),
                Err(e) => tracing::error!("{:}?",e)
            }
        });
    }


    /// on receive scrape signal and result do scrape page
    pub fn listen_scrape_task(&mut self) {
        if let Ok(content) = self.rx.try_recv() {
            tracing::info!(content);
            self.close_loading();
        }
    }

}