use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::seq::SliceRandom;

use crate::exercise::exercise::Exercise;

pub fn update(exercise: &mut Exercise, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Esc => exercise.quit(),
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                exercise.quit()
            }
            KeyCode::Enter => {
                next_prompt(exercise);
                exercise.typed = String::new();
            }
            KeyCode::Char(c) => {
                exercise.typed.push(c);
            }
            KeyCode::Backspace => {
                exercise.typed.pop();
            }

            _ => {}
        };
    }
}

pub fn next_prompt(exercise: &mut Exercise) {
    match exercise.settings.random {
        true => {
            exercise.prompt = exercise
                .settings
                .prompts
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone()
        }
        false => {
            if exercise.stats.count_prompts >= exercise.settings.prompts.len() {
                exercise.quit();
                return;
            } else {
                exercise.prompt = exercise.settings.prompts[exercise.stats.count_prompts].clone();
            }
        }
    }
    exercise.stats.count_prompts += 1;
}
