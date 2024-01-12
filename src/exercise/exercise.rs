use crate::exercise::{settings::Settings, stats::Stats};
use crate::ui::update::next_prompt;

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
        next_prompt(self);
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.stats.end = Some(Instant::now());
        self.should_quit = true;
    }

    pub fn should_quit(&mut self) -> bool {
        // return true if false if the timer has expired
        self.should_quit || self.timer_expired()
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
