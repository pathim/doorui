mod state;

use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct App {
    s: state::State,
}

impl App {
    pub fn new() -> Self {
        Self {
            s: state::State::default(),
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Esc => return true,
            KeyCode::F(1) => self.s.tab = state::Tabs::Edit,
            KeyCode::F(2) => self.s.tab = state::Tabs::Logs,
            _ => {}
        }
        false
    }

    pub fn ui<B: Backend>(&self, f: &mut Frame<B>) {
        let size = f.size();
        let titles = state::Tabs::names()
            .iter()
            .cloned()
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
            .select(self.s.tab as usize);
        f.render_widget(tabs, chunks[0]);

        match self.s.tab {
            state::Tabs::Edit => {
                f.render_widget(Paragraph::new("Hier muss das Bearbeiten hin"), chunks[1])
            }
            state::Tabs::Logs => {
                f.render_widget(Paragraph::new("Hier stehen dann die logs"), chunks[1])
            }
        };
    }
}
