use tui::backend::Backend;

pub struct Tab<B: Backend> {
    pub title: String,
    pub screen: Box<dyn super::Screen<B>>,
}

impl<B: Backend> Tab<B> {
    pub fn new(title: String, screen: Box<dyn super::Screen<B>>) -> Self {
        Self { title, screen }
    }
}
