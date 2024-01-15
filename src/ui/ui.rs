use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Frame},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders,
    },
};

use crate::exercise::exercise::Exercise;

pub fn render(exercise: &mut Exercise, f: &mut Frame) {
    let screen = Block::default()
        .title(" Type Trainer ")
        .title_alignment(Alignment::Center)
        .title(Title::from(" Press 'Esc' to quit ").position(Position::Bottom))
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let screen_area = f.size();

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3), // timer
            Constraint::Length(3), // progress bar
            Constraint::Length(3), // correct ratio
            Constraint::Min(7),    // prompt area
            Constraint::Length(7), // type area
        ])
        .margin(2)
        .split(screen.inner(screen_area));

    f.render_widget(screen, screen_area);

    f.render_widget(exercise.timer.build_widget(), inner[0]);
    f.render_widget(exercise.content.build_progress_bar(), inner[1]);
    f.render_widget(exercise.prompt.build_ratio_bar(), inner[2]);
    f.render_widget(
        exercise
            .prompt
            .build_prompt_area(exercise.content.get_next_prompts()),
        inner[3],
    );
    f.render_widget(exercise.prompt.build_type_area(), inner[4]);
}
