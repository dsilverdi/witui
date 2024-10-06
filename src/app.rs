use crate::scrape::{self, http_get, scrape, ScrapeResult};
use tokio::sync::mpsc;
use tokio::runtime::Handle;

const BASE_URL: &str = "https://en.wikipedia.org/wiki/";

#[derive(Debug, PartialEq)]
pub enum AppState {
    None,
    Init,
    SearchResult, // will render list of wikipedia article
    Article, // render article
    ArticleReference, // render article references
}

#[derive(Debug, PartialEq)]
pub enum PopupState {
    None,
    Search,
    Reference
}

pub struct App {
    pub running: bool,
    pub state: AppState,
    pub popup_state: PopupState,
    pub prev_state: Option<AppState>,
    pub input: String, 
    pub is_loading: bool,
    pub content: Option<ScrapeResult>,
    pub rx: mpsc::Receiver<Option<ScrapeResult>>,
    tx: mpsc::Sender<Option<ScrapeResult>>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            running: true,
            is_loading: false,
            state: AppState::Init,
            popup_state: PopupState::None,
            prev_state: None,
            input: "".to_string(),
            content: None,
            rx,
            tx,
        }
    }

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

    pub fn set_popup(&mut self, state: PopupState) {
        self.popup_state = state
    }

    pub fn close_popup(&mut self) {
        self.popup_state = PopupState::None
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
        let url = BASE_URL.to_string() + &self.input;
        let tx = self.tx.clone();
        tracing::info!("spawn search task");
        tokio::spawn(async move {
            match http_get(&url).await {
                Ok(html) => {
                    tracing::info!("success get http result");
                    let scrape_result = scrape(&html);
                    tracing::info!("success scraping");
                    if let Err(e) = tx.send(scrape_result).await {
                        tracing::error!("Failed to send scrape result: {}", e);
                    }
                },
                Err(e) => {
                    tracing::error!("HTTP get error: {:?}", e);
                    if let Err(e) = tx.send(None).await {
                        tracing::error!("Failed to send error result: {}", e);
                    }
                }
            }
        });
    }


    /// on receive scrape signal and result do scrape page
    pub fn listen_scrape_task(&mut self) {
        if let Ok(content) = self.rx.try_recv() {
            tracing::info!("receive scrape signal");
            self.close_loading();
            self.save_app_content(content);
        }
    }

    // save app content
    pub fn save_app_content(&mut self, content: Option<ScrapeResult>) {
        self.content = content;
        if let Some(content_result)  = &self.content {
            match content_result {
                scrape::ScrapeResult::LinksResult(_) => {
                    self.set_state(AppState::SearchResult);
                    self.close_popup();
                },
                scrape::ScrapeResult::Basic(res) => tracing::info!("{:?}", res),
            }
        }

    }

}