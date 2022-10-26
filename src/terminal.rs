use std::io::Stdout;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::CrosstermBackend;

pub struct Terminal {
    term: tui::Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> std::io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let term = tui::Terminal::new(backend)?;
        Ok(Self { term })
    }

    pub fn draw<F>(&mut self, f: F) -> std::io::Result<tui::terminal::CompletedFrame>
    where
        F: FnOnce(&mut tui::terminal::Frame<CrosstermBackend<Stdout>>),
    {
        self.term.draw(f)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().ok();
        execute!(
            self.term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .ok();
        self.term.show_cursor().ok();
    }
}
