use crossterm::event::KeyCode;
use tui::style::{Color, Style};
use tui::text::{Spans, Text};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::{backend::Backend, Frame};

pub mod logs;

pub trait Screen<B: Backend> {
    fn ui(&mut self, f: &mut Frame<B>, area: tui::layout::Rect);
    fn handle_key(&mut self, key: KeyCode) -> bool;
    fn refresh(&mut self, db: &dyn crate::database::Db);
}

fn render_error<'a, B: Backend, TT: Into<Text<'a>>>(
    f: &mut tui::Frame<B>,
    area: tui::layout::Rect,
    msg: TT,
) {
    f.render_widget(
        Paragraph::new(msg)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Error")
                    .borders(Borders::all())
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().bg(Color::Red).fg(Color::White)),
        area.inner(&tui::layout::Margin {
            vertical: 5,
            horizontal: 5,
        }),
    )
}
