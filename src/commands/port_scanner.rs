use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Paragraph},
};
use tui_textarea::{TextArea, Input};

pub fn launch_port_scanner() -> Result<()> {
    let mut terminal = ratatui::init();

    // Créer les deux textareas
    let mut ip_area = TextArea::default();
    ip_area.set_block(
        ratatui::widgets::Block::bordered().title("IP Address")
    );

    let mut port_area = TextArea::default();
    port_area.set_block(
        ratatui::widgets::Block::bordered().title("Port")
    );

    // Lequel est actif (0 = IP, 1 = Port)
    let mut active = 0;

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Titre
                    Constraint::Length(3), // Champ IP
                    Constraint::Length(3), // Champ Port
                    Constraint::Min(1),    // Espace restant
                ])
                .split(frame.area());

            // Titre
            let title = Paragraph::new("Pentagon IP Scanner")
                .alignment(Alignment::Center)
                .block(ratatui::widgets::Block::bordered());
            frame.render_widget(title, chunks[0]);

            // Champs de saisie
            frame.render_widget(ip_area.widget(), chunks[1]);
            frame.render_widget(port_area.widget(), chunks[2]);
        })?;

        // Gérer les événements
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Tab => {
                    // Changer de champ avec Tab
                    active = if active == 0 { 1 } else { 0 };
                }
                KeyCode::Enter => {
                    // Lancer le scan (à implémenter)
                    break;
                }
                _ => {
                    // Envoyer la touche au textarea actif
                    if active == 0 {
                        ip_area.input(Input::from(key));
                    } else {
                        port_area.input(Input::from(key));
                    }
                }
            }
        }
    }

    ratatui::restore();

    // Afficher ce qui a été saisi
    println!("IP: {}", ip_area.lines().join(""));
    println!("Port: {}", port_area.lines().join(""));

    Ok(())
}