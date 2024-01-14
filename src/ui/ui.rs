use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders},
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
            Constraint::Length(3),  // timer
            Constraint::Length(3),  // progress bar
            Constraint::Length(3),  // correct ratio
            Constraint::Min(10),    // prompt area
            Constraint::Length(10), // type area
        ])
        .margin(2)
        .split(screen.inner(screen_area));

    let prompt_area = Block::default()
        .title("Prompt")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().yellow());

    let prompts = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(10), Constraint::Min(0)])
        .margin(1)
        .split(prompt_area.inner(inner[3]));

    f.render_widget(screen, screen_area);
    f.render_widget(prompt_area, inner[3]);

    f.render_widget(exercise.timer.build_widget(), inner[0]);
    f.render_widget(exercise.content.build_progress_bar(), inner[1]);
    f.render_widget(exercise.prompt.build_ratio_bar(), inner[2]);
    f.render_widget(exercise.prompt.build_prompt_area(), prompts[0]);
    f.render_widget(exercise.content.build_next_prompts(), prompts[1]);
    f.render_widget(exercise.prompt.build_type_area(), inner[4]);
}
