use crate::abstracts::Widget;

#[derive(Clone, Debug)]
pub struct Window {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub content: Option<Widget>,
    pub title_default: bool
}

impl Window {
    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }
}

pub struct WindowBuilder {
    title: String,
    width: usize,
    height: usize,
    content: Option<Widget>,
    title_default: bool,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            title: String::from("My Window"),
            width: 250,
            height: 250,
            content: None,
            title_default: true,
        }
    }

    pub fn title(mut self, title: &str) -> WindowBuilder {
        self.title = String::from(title);
        self.title_default = false;
        self
    }


    pub fn width(mut self, width: usize) -> WindowBuilder {
        self.width = width;
        self
    }

    pub fn height(mut self, height: usize) -> WindowBuilder {
        self.height = height;
        self
    }

    pub fn dimensions(mut self, width: usize, height: usize) -> WindowBuilder {
        self.width = width;
        self.height = height;
        self
    }

    pub fn content(mut self, widget: Widget) -> WindowBuilder {
        self.content = Some(widget);
        self
    }

    pub fn build(self) -> Window {
        Window {
            title: self.title,
            width: self.width,
            height: self.height,
            content: self.content,
            title_default: self.title_default,
        }
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
