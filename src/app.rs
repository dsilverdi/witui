pub struct App {
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
        }
    }
}

impl App {
    pub fn quit(&mut self) {
        println!("app exit");
        self.running = false;
    }
}