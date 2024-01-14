use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::{thread_rng, Rng};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::exercise::settings::Settings;

use std::path::PathBuf;

use super::{prompt::Prompt, split::Split, timer::Timer};

pub struct Exercise {
    pub time: DateTime<Local>,
    pub file_path: PathBuf,
    pub contents: Vec<String>,
    pub contents_index: usize,

    pub split: Split,

    pub timer: Timer,
    pub settings: Settings,
    pub prompt: Prompt,

    should_quit: bool,
}

impl Exercise {
    pub fn build(
        timer: Timer,
        settings: Settings,
        file_path: PathBuf,
        split: Split,
        contents: Vec<String>,
    ) -> Exercise {
        let mut exercise = Exercise {
            time: Local::now(),
            timer,
            prompt: Prompt::new(),
            settings,

            file_path,
            split,
            contents,
            contents_index: 0,

            should_quit: false,
        };

        exercise.select_first_prompt();
        let prompt_chars = exercise.create_prompt();
        exercise.prompt.next(prompt_chars);

        exercise
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

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

    fn select_first_prompt(&mut self) {
        match self.settings.random {
            true => self.contents_index = thread_rng().gen_range(0..self.contents.len()),
            false => self.contents_index = 0,
        };
    }

    fn select_next_prompt(&mut self) {
        match self.settings.random {
            true => self.contents_index = thread_rng().gen_range(0..self.contents.len()),
            false => self.contents_index += 1,
        };
    }

    fn create_prompt(&mut self) -> Vec<char> {
        if let Some(string) = self.contents.get(self.contents_index) {
            string.chars().collect()
        } else {
            self.quit();
            Vec::new()
        }
    }

    fn press_enter(&mut self) {
        if self.split == Split::Text {
        } else {
            self.prompt.finish();
            self.select_next_prompt();
            let prompt_chars = self.create_prompt();
            self.prompt.next(prompt_chars);
        }
    }

    fn press_backspace(&mut self) {
        if self.settings.backspace {
            self.prompt.remove_char();
        }
    }

    fn press_char(&mut self, c: char) {
        self.prompt.type_char(c);
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.timer.stop();
        self.should_quit = true;
    }

    pub fn should_quit(&mut self) -> bool {
        if self.timer.timer_expired() {
            self.quit();
        }
        self.should_quit
    }
}

impl Serialize for Exercise {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Exercise", 3)?;
        state.serialize_field("date", &self.time.to_rfc2822())?;
        state.serialize_field("timer", &self.timer)?;
        state.serialize_field("stats", &self.prompt)?;
        state.serialize_field("settings", &self.settings)?;
        state.end()
    }
}
