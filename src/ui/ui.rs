use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::exercise::exercise::Exercise;

pub fn render(exercise: &mut Exercise, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc` to stop running.\n\
        Prompt: {}
      ",
            exercise.prompt
        ))
        .block(
            Block::default()
                .title("Type trainer")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        f.size(),
    )
}
