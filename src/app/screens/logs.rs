use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::Row,
};

use crate::database::{Db, Error, Log, Result};

pub struct Logs {
    logs: Vec<Log>,
    table_state: tui::widgets::TableState,
    err: Option<Error>,
    logs_fn: Box<dyn Fn(&dyn Db) -> Result<Vec<Log>>>,
    edit_overlay: Option<super::edit_overlay::EditOverlay>,
}

impl Logs {
    pub fn new(logs_fn: Box<dyn Fn(&dyn Db) -> Result<Vec<Log>>>) -> Self {
        let mut table_state = tui::widgets::TableState::default();
        table_state.select(Some(0));
        Self {
            table_state,
            logs: Vec::default(),
            err: None,
            logs_fn,
            edit_overlay: None,
        }
    }
}

impl From<&Log> for tui::widgets::Row<'_> {
    fn from(l: &Log) -> Self {
        Self::new(vec![l.date.to_string(), l.card_uid.clone()])
    }
}

impl<B: Backend> super::Screen<B> for Logs {
    fn ui(&mut self, f: &mut tui::Frame<B>, area: tui::layout::Rect) {
        match &self.err {
            Some(e) => super::render_error(f, area, format!("{}", e)),
            None => {
                let rows: Vec<_> = self.logs.iter().map(|l| l.into()).collect();
                let s = format!("{:?}", rows);
                let table = tui::widgets::Table::new(rows)
                    .header(
                        Row::new(vec!["Datetime", "uid"])
                            .style(Style::default().fg(Color::Yellow))
                            .bottom_margin(1),
                    )
                    .widths(&[Constraint::Length(19), Constraint::Length(10)])
                    .column_spacing(3)
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol(">>");
                f.render_stateful_widget(table, area, &mut self.table_state);
                if let Some(edit) = self.edit_overlay.as_mut() {
                    edit.ui(
                        f,
                        area.inner(&tui::layout::Margin {
                            vertical: 5,
                            horizontal: 5,
                        }),
                    )
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyCode, db: &dyn Db) -> bool {
        if let Some(edit) = self.edit_overlay.as_mut() {
            let close_overlay =
                <super::edit_overlay::EditOverlay as super::Screen<B>>::handle_key(edit, key, db);
            if close_overlay {
                self.edit_overlay = None;
            }
            return false;
        }
        let selected = self.table_state.selected().unwrap_or(0);
        match key {
            KeyCode::Esc => return true,
            KeyCode::Down => {
                if selected + 1 < self.logs.len() {
                    self.table_state.select(Some(selected + 1));
                }
            }
            KeyCode::Up => {
                if selected >= 1 {
                    self.table_state.select(Some(selected - 1));
                }
            }
            KeyCode::Enter => {
                let uid = self.logs[selected].card_uid.clone();
                let new_card = crate::database::Card {
                    uid,
                    ..Default::default()
                };
                self.edit_overlay = Some(new_card.into());
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
