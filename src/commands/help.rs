use ratatui::layout::Alignment;
use std::slice::Chunks;
use ratatui::{Frame, Terminal};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn show_help() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|mut f| {
            let pragraph = Paragraph::new("Pentagon help")
                .alignment(Alignment::Center);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.area());

            f.render_widget(pragraph, chunks[0]);
        }).unwrap();
    }
}