use crate::current_screen::CurrentScreen;

struct App {
    current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        Self { current_screen: CurrentScreen::Packages }
    }
}
