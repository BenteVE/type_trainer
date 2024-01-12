use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::seq::SliceRandom;

use crate::exercise::{settings::Settings, stats::Stats};

use std::time::Instant;

pub struct Exercise {
    pub settings: Settings,
    pub stats: Stats,
    pub prompt: String, // store the current prompt
    pub typed: String,  // store whatever is typed for the current prompt
    // implement functions to compare a character to the current prompts
    pub should_quit: bool,
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
                KeyCode::Enter => {
                    // Any missing are also mistakes (extra chars are already counted when the characters were typed)
                    if self.prompt.chars().count() > self.typed.chars().count() {
                        self.stats.count_fault +=
                            self.prompt.chars().count() - self.typed.chars().count();
                    }
                    self.next_prompt();
                    self.typed = String::new();
                }
                KeyCode::Char(c) => {
                    if self.typed.len() + 1 > self.prompt.len()
                        || self.prompt.chars().nth(self.typed.chars().count()).unwrap() != c
                    {
                        self.stats.count_fault += 1;
                    } else {
                        self.stats.count_correct += 1;
                    }
                    self.typed.push(c);
                }
                KeyCode::Backspace => {
                    if self.settings.backspace {
                        self.typed.pop();
                    }
                }

                _ => {}
            };
        }
    }

    pub fn next_prompt(&mut self) {
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
                if self.stats.count_prompts >= self.settings.prompts.len() {
                    self.quit();
                    return;
                } else {
                    self.prompt = self.settings.prompts[self.stats.count_prompts].clone();
                }
            }
        }
        self.stats.count_prompts += 1;
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
