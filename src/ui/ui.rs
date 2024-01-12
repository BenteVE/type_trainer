use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::exercise::exercise::Exercise;

pub fn render(exercise: &mut Exercise, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .horizontal_margin(5)
        .vertical_margin(2)
        .split(f.size());

    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc` to stop running.\n\
      "
        ))
        .block(
            Block::default()
                .title("Stats")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        layout[0],
    );
    f.render_widget(
        Paragraph::new(format!("{}", exercise.prompt))
            .block(Block::default().title("Prompt:").borders(Borders::ALL)),
        layout[1],
    );
    f.render_widget(
        Paragraph::new(format!("{}", exercise.typed))
            .block(Block::default().title("Typed:").borders(Borders::ALL)),
        layout[2],
    );
}
