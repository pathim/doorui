mod app;
mod database;
mod terminal;

use crossterm::event::{self, Event};

fn main() -> std::io::Result<()> {
    let db = database::sqlite::Sqlite::new("testdb.db").expect("Unable to open database file");
    let mut term = terminal::Terminal::new()?;
    let mut app = app::App::new(db);
    loop {
        term.draw(|f| app.ui(f))?;
        if let Event::Key(key) = event::read()? {
            let exit = app.handle_key(key.code);
            if exit {
                return Ok(());
            }
        };
    }
}
