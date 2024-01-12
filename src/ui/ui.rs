use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Gauge, Padding, Paragraph, Wrap},
};

use crate::exercise::exercise::Exercise;

pub fn render(exercise: &mut Exercise, f: &mut Frame) {
    let screen = Block::default()
        .title("Type Trainer")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let screen_area = f.size();

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .margin(2)
        .split(screen.inner(screen_area));

    f.render_widget(screen, screen_area);

    f.render_widget(
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Ratio"))
            .gauge_style(Style::default().fg(Color::LightGreen).bg(Color::LightRed))
            .percent(exercise.stats.ratio()),
        inner[0],
    );

    f.render_widget(
        Paragraph::new(format!("{}", exercise.prompt))
            .block(
                Block::default()
                    .title("Prompt:")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::LightYellow))
                    .padding(Padding::uniform(1)),
            )
            .wrap(Wrap { trim: false }),
        inner[1],
    );

    f.render_widget(
        Paragraph::new(format!("{}", exercise.typed))
            .block(
                Block::default()
                    .title("Typed:")
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(1)),
            )
            .not_rapid_blink()
            .wrap(Wrap { trim: false }),
        inner[2],
    );
}
