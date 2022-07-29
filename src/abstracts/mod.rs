mod application;
mod window;

pub use crate::abstracts::{
    application::{
        Application,
        ApplicationBuilder
    },
    window::{
        Window,
        WindowBuilder
    },
};

#[derive(Clone,Debug)]
pub enum Widget {
    Window(Box<Window>)
}