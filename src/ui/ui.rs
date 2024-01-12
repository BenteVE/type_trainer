use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame},
    style::{Color, Style, Stylize},
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders, Padding, Paragraph, Wrap},
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
        BarChart::default()
            .block(Block::default().title("Stats").borders(Borders::ALL))
            .direction(Direction::Horizontal)
            .label_style(Style::new().white())
            .data(
                BarGroup::default().bars(&[
                    Bar::default()
                        .value(exercise.stats.count_correct as u64)
                        .label("Correct: ".into())
                        .style(Style::default().fg(Color::Green)),
                    Bar::default()
                        .value(exercise.stats.count_fault as u64)
                        .label("Faults:  ".into())
                        .style(Style::default().fg(Color::Red)),
                ]),
            ),
        inner[0],
    );

    f.render_widget(
        Paragraph::new(format!("{}", exercise.prompt))
            .block(
                Block::default()
                    .title("Prompt:")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow))
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
