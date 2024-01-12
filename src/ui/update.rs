use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::exercise::exercise::Exercise;

pub fn update(exercise: &mut Exercise, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => exercise.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                exercise.quit()
            }
        }
        _ => {}
    };
}
