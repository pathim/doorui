use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame};

pub mod logs;

pub trait Screen<B: Backend> {
    fn ui(&self, f: &mut Frame<B>, area: tui::layout::Rect);
    fn handle_key(&mut self, key: KeyCode) -> bool;
    fn refresh(&mut self, db: &dyn crate::database::Db);
}
