use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, LineGauge, Padding, Paragraph, Wrap},
};

use crate::exercise::{content::Content, exercise::Exercise, prompt::Prompt, timer::Timer};

pub fn render(exercise: &Exercise, f: &mut Frame) {
    let border = Block::default()
        .title(" Type Trainer ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(11), // information block
            Constraint::Min(7),     // prompt area
            Constraint::Length(7),  // type area
        ])
        .vertical_margin(2)
        .horizontal_margin(1)
        .split(border.inner(f.size()));

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Min(22), Constraint::Length(22)])
        .split(inner[0]);

    let top_left = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Ratio(1, 3), // timer
            Constraint::Ratio(1, 3), // progress
            Constraint::Ratio(1, 3), // ratio
        ])
        .split(border.inner(top[0]));

    let top_right = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Ratio(2, 3), // options
            Constraint::Ratio(1, 3), // words per minute
        ])
        .split(border.inner(top[1]));

    f.render_widget(border, f.size());
    f.render_widget(timer(&exercise.timer), top_left[0]);
    f.render_widget(progress_bar(&exercise.content), top_left[1]);
    f.render_widget(ratio_bar(&exercise.prompt), top_left[2]);
    f.render_widget(info(), top_right[0]);
    f.render_widget(wpm(&exercise), top_right[1]);
    f.render_widget(prompt(exercise), inner[1]);
    f.render_widget(typed(&exercise.prompt), inner[2]);
}

fn info() -> Paragraph<'static> {
    let options = vec![
        "Start:    Type",
        "Stop:    'Esc'",
        "Restart: 'Ctrl+R'",
        "Quit:    'Ctrl+C'",
    ];
    let text = Text::from(
        options
            .iter()
            .map(|&option| Line::from(option))
            .collect::<Vec<Line>>(),
    );
    Paragraph::new(text).block(
        Block::default()
            .title("Options")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL),
    )
}

fn wpm(exercise: &Exercise) -> Paragraph {
    Paragraph::new(format!("{} WPM", exercise.calculate_wpm()))
        .block(
            Block::new()
                .title("Words per minute")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false })
}

// if there is some duration: give the ratio to that duration
// otherwise reset every minute
fn timer(timer: &Timer) -> LineGauge {
    LineGauge::default()
        .block(Block::default().borders(Borders::ALL).title("Timer"))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .ratio(timer.ratio())
        .line_set(linegauge_set_dots())
        .label(timer.get_time_label())
}

/// Used to show how many prompts are remaining
fn progress_bar(content: &Content) -> LineGauge {
    LineGauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .ratio(content.ratio())
        .line_set(symbols::line::THICK)
}

/// Used how many prompts are left
fn ratio_bar(prompt: &Prompt) -> LineGauge {
    LineGauge::default()
        .block(Block::default().borders(Borders::ALL).title("Ratio"))
        .gauge_style(Style::default().fg(Color::LightGreen).bg(Color::LightRed))
        .ratio(prompt.ratio())
        .line_set(symbols::line::THICK)
}

fn prompt(exercise: &Exercise) -> Paragraph {
    let prompt = &exercise.prompt.prompt;
    let typed = &exercise.prompt.typed;

    // change the colors of the paragraph
    let mut prompt_styled: Vec<Span> = Vec::new();

    for i in 0..usize::min(typed.len(), prompt.len()) {
        match prompt[i] == typed[i] {
            true => prompt_styled.push(Span::from(prompt[i].to_string()).bg(Color::Green).yellow()),
            false => prompt_styled.push(Span::from(prompt[i].to_string()).bg(Color::Red).yellow()),
        };
    }
    // if typed is longer than prompt, we add red spaces for each unnecessary letter
    if typed.len() > prompt.len() {
        prompt_styled.push(
            Span::from(
                std::iter::repeat(" ")
                    .take(typed.len() - prompt.len())
                    .collect::<String>(),
            )
            .bg(Color::Red)
            .yellow(),
        )
    }
    // The rest of the line should not be styled
    else if typed.len() < prompt.len() {
        prompt_styled.push(Span::from(prompt[typed.len()..].iter().collect::<String>()).yellow());
    }

    let mut text = Text::from(Line::from(prompt_styled));
    text.extend(exercise.content.get_next_prompts());

    Paragraph::new(text).wrap(Wrap { trim: false }).block(
        Block::default()
            .title("Prompt")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().yellow())
            .padding(Padding::uniform(1)),
    )
}

/// The typing area
fn typed(prompt: &Prompt) -> Paragraph {
    let mut typed_styled = vec![Span::from(prompt.typed.iter().collect::<String>())];

    // Add a cursor to the typed text
    typed_styled.push(Span::from(symbols::block::FULL).add_modifier(Modifier::SLOW_BLINK));

    Paragraph::new(Line::from(typed_styled))
        .block(
            Block::default()
                .title("Typed:")
                .borders(Borders::ALL)
                .padding(Padding::uniform(1)),
        )
        .wrap(Wrap { trim: false })
}

fn linegauge_set_dots() -> symbols::line::Set {
    symbols::line::Set {
        horizontal: symbols::DOT,
        vertical: symbols::DOT,
        top_right: symbols::DOT,
        top_left: symbols::DOT,
        bottom_right: symbols::DOT,
        bottom_left: symbols::DOT,
        vertical_left: symbols::DOT,
        vertical_right: symbols::DOT,
        horizontal_down: symbols::DOT,
        horizontal_up: symbols::DOT,
        cross: symbols::DOT,
    }
}
