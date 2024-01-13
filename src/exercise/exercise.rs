use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::seq::SliceRandom;

use crate::exercise::{settings::Settings, stats::Stats};

use std::time::Instant;

pub struct Exercise {
    pub settings: Settings,
    pub stats: Stats,
    pub prompt: String, // store the current prompt
    pub typed: String,  // store whatever is typed for the current prompt
    should_quit: bool,
}

impl Exercise {
    pub fn build(settings: Settings) -> Exercise {
        Exercise {
            settings,
            stats: Stats::new(),
            prompt: String::new(),
            typed: String::new(),
            should_quit: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Start the exercise timer, select the first prompt
    pub fn start(&mut self) {
        // start the exercise timer
        self.stats.start = Some(Instant::now());

        // select the first prompt
        self.next_prompt();
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

    // This function should work differently when the split is text
    fn press_enter(&mut self) {
        self.stats.count_enter += 1;
        // Any missing are also mistakes (extra chars are already counted when the characters were typed)
        if self.prompt.chars().count() > self.typed.chars().count() {
            self.stats.count_fault += self.prompt.chars().count() - self.typed.chars().count();
        }
        self.next_prompt();
        self.typed = String::new();
    }

    fn press_backspace(&mut self) {
        match self.settings.backspace {
            true => {
                self.typed.pop();
                self.stats.count_backspace += 1;
            }
            false => {}
        }
    }

    fn press_char(&mut self, c: char) {
        match c == self.correct_char() {
            true => self.stats.count_correct += 1,
            false => self.stats.count_fault += 1,
        }
        self.typed.push(c);
    }

    /// Returns the character that is supposed to be pressed at any given time
    fn correct_char(&self) -> char {
        if self.typed.len() + 1 > self.prompt.len() {
            return '\n';
        } else {
            return self.prompt.chars().nth(self.typed.chars().count()).unwrap();
        }
    }

    fn next_prompt(&mut self) {
        match self.settings.random {
            true => {
                self.prompt = self
                    .settings
                    .prompts
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone()
            }
            false => {
                if self.stats.count_enter >= self.settings.prompts.len() {
                    self.quit();
                    return;
                } else {
                    self.prompt = self.settings.prompts[self.stats.count_enter].clone();
                }
            }
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
