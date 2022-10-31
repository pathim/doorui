use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Constraint,
    widgets::{Block, Borders, Row, Table},
};

use crate::database::Db;

pub struct Permissions {
    pub allow_unlock: bool,
    pub allow_entry: bool,
    pub allow_sneaky: bool,
    pub always_sneaky: bool,
}
pub struct EditOverlay {
    uid: String,
    member_id: String,
    pin: String,
    owner: String,
    permissions: Permissions,
    pin_hidden: bool,
}

impl From<crate::database::Card> for EditOverlay {
    fn from(c: crate::database::Card) -> Self {
        let permissions = Permissions {
            allow_unlock: c.allow_unlock,
            allow_entry: c.allow_entry,
            allow_sneaky: c.allow_sneaky,
            always_sneaky: c.always_sneaky,
        };
        Self {
            uid: c.uid,
            member_id: c.member_id,
            pin: c.pin,
            owner: c.owner,
            permissions,
            pin_hidden: true,
        }
    }
}

fn checkbox(b: bool) -> &'static str {
    if b {
        "[x]"
    } else {
        "[ ]"
    }
}

impl<B: Backend> super::Screen<B> for EditOverlay {
    fn ui(&mut self, f: &mut tui::Frame<B>, area: tui::layout::Rect) {
        let table = Table::new(vec![
            Row::new(vec!["UID", &self.uid]),
            Row::new(vec!["Member ID", &self.member_id]),
            Row::new(vec!["Owner", &self.owner]),
            Row::new(vec![
                "PIN",
                if self.pin_hidden { "****" } else { &self.pin },
            ]),
            Row::new(vec![
                "Allow Unlock",
                checkbox(self.permissions.allow_unlock),
            ]),
            Row::new(vec!["Allow Entry", checkbox(self.permissions.allow_entry)]),
            Row::new(vec![
                "Allow Sneaky",
                checkbox(self.permissions.allow_sneaky),
            ]),
            Row::new(vec![
                "Always Sneaky",
                checkbox(self.permissions.always_sneaky),
            ]),
            Row::new(vec!["Save"]),
        ])
        .widths(&[Constraint::Length(19), Constraint::Length(10)]);
        f.render_widget(
            table.block(Block::default().title("Error").borders(Borders::all())),
            area,
        )
    }

    fn handle_key(&mut self, key: KeyCode, db: &dyn Db) -> bool {
        match key {
            KeyCode::Esc => true,
            _ => false,
        }
    }

    fn refresh(&mut self, db: &dyn crate::database::Db) {
        todo!()
    }
}
