use crate::abstracts::Window;

#[derive(Clone)]
pub struct Application {
    pub title: String,
    pub main_windows: Vec<Window>,
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }
}

pub struct ApplicationBuilder {
    title: String,
    main_windows: Vec<Window>,
}

impl ApplicationBuilder {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {
            title: String::from("My App"),
            main_windows: Vec::new(),
        }
    }

    pub fn title(mut self, title: &str) -> ApplicationBuilder {
        self.title = String::from(title);
        self
    }

    pub fn add_window(mut self, window: Window) -> ApplicationBuilder {
        self.main_windows.push(window);
        self
    }

    pub fn build(mut self) -> Application {
        for window in &mut self.main_windows {
            if window.title_default {
                window.title = self.title.clone();
            }
        }
        Application {
            title: self.title,
            main_windows: self.main_windows
        }
    }
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
