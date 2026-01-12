use ratatui::layout::Alignment;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn show_help() {
    let mut terminal = ratatui::init();
    let items = vec!["Ip scanner: scan ip for show who port is open"];

    loop {
        terminal.draw(|mut f| {
            let pragraph = Paragraph::new("Pentagon help")
                .alignment(Alignment::Center);

            let list = List::new(items.iter().map(|s| ListItem::new(*s)).collect::<Vec<ListItem>>())
                .block(Block::default().borders(Borders::ALL));

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.area());

            f.render_widget(pragraph, chunks[0]);
            f.render_widget(list, chunks[1]);
        }).unwrap();
    }
}