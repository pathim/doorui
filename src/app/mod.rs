mod screens;
mod tab;

use self::screens::Screen;
use tab::Tab;

use super::database::Db;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders},
    Frame,
};

pub struct App<T: Db, B: Backend> {
    db: T,
    tabs: Vec<Tab<B>>,
    current_tab: usize,
}

impl<T: Db, B: Backend> App<T, B> {
    pub fn new(db: T) -> Self {
        let mut tabs = vec![
            Tab::new(
                String::from("Logs Failure"),
                Box::new(screens::logs::Logs::new(Box::new(|db: &dyn Db| {
                    db.get_logs_failure()
                }))),
            ),
            Tab::new(
                String::from("Logs Success"),
                Box::new(screens::logs::Logs::new(Box::new(|db: &dyn Db| {
                    db.get_logs_success()
                }))),
            ),
        ];
        tabs[0].screen.refresh(&db);
        Self {
            db,
            tabs,
            current_tab: 0,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Esc => return true,
            KeyCode::F(x) => {
                if x > 0 && (x as usize) <= self.tabs.len() {
                    self.current_tab = (x - 1) as usize;
                    self.tabs[self.current_tab].screen.refresh(&self.db);
                }
            }
            _ => {
                self.tabs[self.current_tab].screen.handle_key(key);
            }
        }
        false
    }

    pub fn ui(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let titles = self
            .tabs
            .iter()
            .map(|x| x.title.clone())
            .map(Spans::from)
            .collect();
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);
        let tabs = tui::widgets::Tabs::new(titles)
            .block(Block::default().title("Door UI").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(tui::symbols::DOT)
            .select(self.current_tab as usize);
        f.render_widget(tabs, chunks[0]);
        self.tabs[self.current_tab].screen.ui(f, chunks[1]);
    }
}
