use crossterm::event::KeyCode;
use tui::backend::Backend;

use crate::database::{Db, Error, Log, Result};

pub struct Logs {
    logs: Vec<Log>,
    selected: u32,
    err: Option<Error>,
    logs_fn: Box<dyn Fn(&dyn Db) -> Result<Vec<Log>>>,
}

impl Logs {
    pub fn new(logs_fn: Box<dyn Fn(&dyn Db) -> Result<Vec<Log>>>) -> Self {
        Self {
            selected: 0,
            logs: Vec::default(),
            err: None,
            logs_fn,
        }
    }
}

impl<B: Backend> super::Screen<B> for Logs {
    fn ui(&self, f: &mut tui::Frame<B>, area: tui::layout::Rect) {
        let s = self
            .err
            .as_ref()
            .map(|e| format!("{:?}", e))
            .unwrap_or_else(|| {
                format!(
                    "Number of logs:{}. selected {}",
                    self.logs.len(),
                    self.selected
                )
            });
        f.render_widget(tui::widgets::Paragraph::new(s), area)
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Down => {
                self.selected += 1;
            }
            KeyCode::Up => {
                self.selected = self.selected.saturating_sub(1);
            }
            _ => {}
        }
        false
    }

    fn refresh(&mut self, db: &dyn Db) {
        match (self.logs_fn)(db) {
            Ok(logs) => {
                self.logs = logs;
            }
            Err(e) => {
                self.err = Some(e);
            }
        }
    }
}
