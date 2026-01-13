mod commands;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use ratatui::widgets::{List, ListItem, ListState};

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut terminal = ratatui::init();
    let mut state = ListState::default();
    state.select(Some(0));
    let items = vec!["Help", "IP scanner"];

    loop {
        terminal.draw(|mut f| {
            let area = f.size();
            let text = "Wolcome to pentagon v2.0";
            let paragraph = Paragraph::new(text).alignment(Alignment::Center);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.area());

            let list = List::new(items.iter().map(|s| ListItem::new(*s)).collect::<Vec<ListItem>>())
                .block(Block::default().borders(Borders::ALL).title(" Tools "))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_widget(
                Block::default().style(Style::default().bg(Color::Black)),
                f.area()
            );
            f.render_widget(paragraph, chunks[0]);
            f.render_stateful_widget(list, chunks[1], &mut state)
        })?;

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    if let Some(i) = state.selected() {
                        state.select(Some(i + 1).min(Option::from(items.len() - 1)));
                    }
                }
                KeyCode::Up => {
                    if let Some(i) = state.selected() {
                        state.select(Some(i.saturating_sub(1)));
                    }
                }
                KeyCode::Enter => {
                    if let Some(index) = state.selected() {
                        match items[index] {
                            "Help" => {
                                commands::help::show_help();
                            }
                            "IP scanner" => {
                                commands::port_scanner::launch_port_scanner().expect("TODO: panic message");
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
    ratatui::restore();

    Ok(())

}
