use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::exercise::{settings::Settings, stats::Stats};

use std::time::Instant;

use super::prompt::Prompt;

pub struct Exercise {
    pub settings: Settings,
    pub prompt: Prompt,
    pub stats: Stats,
    should_quit: bool,
}

impl Exercise {
    pub fn build(settings: Settings, prompt: Prompt) -> Exercise {
        Exercise {
            settings,
            prompt: prompt,
            stats: Stats::new(),
            should_quit: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Start the exercise timer, select the first prompt
    pub fn start(&mut self) {
        // start the exercise timer
        self.stats.start = Some(Instant::now());
    }

    pub fn update(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Esc => self.quit(),
                KeyCode::Char('c') | KeyCode::Char('C')
                    if key_event.modifiers == KeyModifiers::CONTROL =>
                {
                    self.quit()
                }
                KeyCode::Enter => self.press_enter(),
                KeyCode::Char(c) => self.press_char(c),
                KeyCode::Backspace => self.press_backspace(),
                _ => {}
            };
        }
    }

    fn press_enter(&mut self) {
        // THIS SHOULD BE CHANGED BECAUSE FOR A TEXT, THE MISSING COUNTS AREN'T NECESSARILY FAULTS
        self.stats.count_fault += self.prompt.count_missing();
        self.prompt.handle_enter();
        self.stats.count_enter += 1;
    }

    fn press_backspace(&mut self) {
        match self.settings.backspace {
            true => {
                self.prompt.remove_char();
                self.stats.count_backspace += 1;
            }
            false => {}
        }
    }

    fn press_char(&mut self, c: char) {
        match self.prompt.type_char(c) {
            true => self.stats.count_correct += 1,
            false => self.stats.count_fault += 1,
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.stats.end = Some(Instant::now());
        self.should_quit = true;
    }

    pub fn should_quit(&mut self) -> bool {
        if self.timer_expired() {
            self.quit();
        }
        self.should_quit
    }

    fn timer_expired(&self) -> bool {
        if let Some(duration) = self.settings.duration {
            self.stats
                .start
                .expect("The timer should be started at the start of the exercise")
                .elapsed()
                >= duration
        } else {
            false
        }
    }
}
