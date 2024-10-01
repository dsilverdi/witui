#[derive(Debug, PartialEq)]
pub enum AppState {
    None,
    Init,
    Search,
    SearchResult, // will render list of wikipedia article
    Article, // render article
    ArticleReference, // render article references
}

// pub enum InputMode {
//     Normal,
//     Editing
// }

pub struct App {
    pub running: bool,
    pub state: AppState,
    pub prev_state: Option<AppState>,
    pub input: String, 
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: AppState::Init,
            prev_state: None,
            input: "".to_string(),
        }
    }
}

impl App {
    pub fn quit(&mut self) {
        self.running = false;
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

}