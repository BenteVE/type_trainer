use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, LineGauge, Padding, Paragraph, Wrap},
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
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(screen.inner(screen_area));

    f.render_widget(screen, screen_area);

    f.render_widget(exercise.timer.build_widget(), inner[0]);

    f.render_widget(
        LineGauge::default()
            .block(Block::default().borders(Borders::ALL).title("Ratio"))
            .gauge_style(Style::default().fg(Color::LightGreen).bg(Color::LightRed))
            .ratio(exercise.prompt.ratio())
            .line_set(symbols::line::THICK),
        inner[1],
    );

    // change the colors of the paragraph
    let mut prompt_styled: Vec<Span> = Vec::new();

    for i in 0..exercise.prompt.prompt.len() {
        if i < exercise.prompt.typed.len() {
            match exercise.prompt.prompt[i] == exercise.prompt.typed[i] {
                true => prompt_styled
                    .push(Span::from(exercise.prompt.prompt[i].to_string()).bg(Color::Green)),
                false => prompt_styled
                    .push(Span::from(exercise.prompt.prompt[i].to_string()).bg(Color::Red)),
            };
        } else {
            prompt_styled.push(Span::from(exercise.prompt.prompt[i].to_string()));
        }
    }

    // ADD EXTRA RED SPACES FOR EACH CHAR THAT PROMPT IS LONGER THAN TYPED

    // change the colors of the typed text
    let mut typed_styled: Vec<Span> = Vec::new();
    for i in 0..exercise.prompt.typed.len() {
        typed_styled.push(Span::from(exercise.prompt.typed[i].to_string()));
    }

    // Add a cursor
    typed_styled.push(Span::styled(
        symbols::block::FULL,
        Style::default().add_modifier(Modifier::SLOW_BLINK),
    ));

    f.render_widget(
        Paragraph::new(Line::from(prompt_styled))
            .block(
                Block::default()
                    .title("Prompt:")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::LightYellow))
                    .padding(Padding::uniform(1)),
            )
            .wrap(Wrap { trim: false }),
        inner[2],
    );

    f.render_widget(
        Paragraph::new(Line::from(typed_styled))
            .block(
                Block::default()
                    .title("Typed:")
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(1)),
            )
            .wrap(Wrap { trim: false }),
        inner[3],
    );
}

