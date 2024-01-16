use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::exercise::settings::Settings;

use super::{content::Content, prompt::Prompt, timer::Timer};

#[derive(PartialEq, Eq)]
pub enum State {
    Waiting,
    Running,
    Pausing,
    Finished,
    Quiting,
}

pub struct Exercise {
    pub time: DateTime<Local>,

    pub timer: Timer,
    pub content: Content,
    pub settings: Settings,
    pub prompt: Prompt,

    pub state: State,
}

impl Exercise {
    pub fn build(timer: Timer, content: Content, settings: Settings) -> Exercise {
        // SET THE FIRST PROMPT!
        let prompt = Prompt::new(
            content
                .get_prompt()
                .expect("The program will not start if there is no content"),
        );
        Exercise {
            time: Local::now(),
            timer,
            prompt: prompt,
            settings,
            content,
            state: State::Waiting,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn update(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            if key_event.modifiers == KeyModifiers::CONTROL {
                match self.state {
                    State::Waiting => match key_event.code {
                        KeyCode::Char('c') | KeyCode::Char('C') => self.quit(),
                        _ => {}
                    },
                    State::Running => match key_event.code {
                        KeyCode::Char('c') | KeyCode::Char('C') => self.stop(),
                        KeyCode::Char('r') | KeyCode::Char('R') => self.restart(),
                        KeyCode::Char('p') | KeyCode::Char('P') => self.pause(),
                        _ => {}
                    },
                    State::Pausing => match key_event.code {
                        KeyCode::Char('c') | KeyCode::Char('C') => self.stop(),
                        KeyCode::Char('r') | KeyCode::Char('R') => self.restart(),
                        _ => {}
                    },
                    State::Finished => match key_event.code {
                        KeyCode::Char('c') | KeyCode::Char('C') => self.quit(),
                        KeyCode::Char('r') | KeyCode::Char('R') => self.restart(),
                        _ => {}
                    },
                    _ => {}
                }
            } else {
                if self.state == State::Waiting || self.state == State::Pausing {
                    self.state = State::Running;
                    self.start();
                }
                if self.state == State::Running {
                    match key_event.code {
                        KeyCode::Enter => self.press_enter(),
                        KeyCode::Char(c) => self.press_char(c),
                        KeyCode::Backspace => self.press_backspace(),
                        _ => {}
                    };
                }
            }
        }
    }

    fn press_enter(&mut self) {
        self.prompt.finish();

        self.content.next_prompt();
        if let Some(p) = self.content.get_prompt() {
            self.prompt.set(p);
        } else {
            self.stop();
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

    pub fn calculate_wpm(&self) -> usize {
        // we only count the correct characters for this calculation
        // we subtract the backspace to avoid having a high wpm by typing and removing the same letters
        let letters = self
            .prompt
            .count_correct
            .saturating_sub(self.prompt.count_backspace);

        // for the wpm calculation, each word is 5 letters long
        let words = letters as f32 / 5 as f32;

        let seconds = self.timer.get_time().as_secs_f32();
        let minutes = seconds / 60 as f32;

        (words / minutes).round() as usize
    }

    fn start(&mut self) {
        self.state = State::Running;
        self.timer.start();
    }

    fn pause(&mut self) {
        self.state = State::Pausing;
        self.timer.pause();
    }

    fn restart(&mut self) {
        self.state = State::Waiting;
        self.time = Local::now();

        self.content.reset();
        self.timer.reset();
        self.prompt = Prompt::new(self.content.get_prompt().unwrap());
    }

    fn stop(&mut self) {
        self.timer.stop();
        self.state = State::Finished;

        // Write the results to a file
        // println!("{}", serde_json::to_string(&exercise).unwrap());
    }

    pub fn quit(&mut self) {
        self.state = State::Quiting;
    }

    pub fn check_timer(&mut self) {
        if self.timer.timer_expired() {
            self.stop();
        }
    }
}

impl Serialize for Exercise {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Exercise", 6)?;
        state.serialize_field("date", &self.time.to_rfc2822())?;
        state.serialize_field("timer", &self.timer)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("settings", &self.settings)?;
        state.serialize_field("stats", &self.prompt)?;
        state.serialize_field("wpm", &self.calculate_wpm())?;
        state.end()
    }
}
