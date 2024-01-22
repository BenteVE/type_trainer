use super::{content::Content, prompt::Prompt, state::State, timer::Timer};
use crate::app::settings::Settings;
use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{fs::OpenOptions, io::Write};

/// The main structure of the application that combines all the parts of the type trainer.
pub struct Exercise {
    pub time: DateTime<Local>,
    pub timer: Timer,
    pub content: Content,
    pub settings: Settings,
    pub prompt: Prompt,
    pub state: State,
}

impl Exercise {
    /// Builds a new exercise
    pub fn build(timer: Timer, content: Content, settings: Settings) -> Exercise {
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
    ///
    /// This function is unused in the current implementation of the program.
    pub fn tick(&self) {}

    /// Handles key events and changes the state of the application accordingly.
    ///
    /// Any [KeyEvent] with the [KeyModifiers::CONTROL] modifier will be used to change the state of the application.
    /// Events without this modifier will be interpreted as input for the exercise prompt.
    ///
    /// The application will stay in [State::Waiting] until the user starts typing.
    /// The application will then progress to [State::Running].
    /// The user can also press the quit button to immediately progress to [State::Quitting] that will shut down the program.
    ///
    /// When the exercise is finished, the application will automatically progress from [State::Running] to [State::Finished].
    /// At any time during [State::Running], the user can also enter the following states by pressing the related buttons:
    /// - [State::Finished] by pressing the stop button
    /// - [State::Waiting] by pressing the restart button
    /// - [State::Pausing] by pressing the pause button, the application will wait in this state until the user starts typing again
    ///
    /// From [State::Finished] the user can quit the application or restart the exercise.
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
                if self.settings.auto && self.prompt.prompt.len() == self.prompt.typed.len() {
                    self.press_enter();
                }
                if let Some(t) = self.settings.terminate {
                    if self.prompt.count_fault >= t {
                        self.stop();
                    }
                }
            }
        }
    }

    /// Handles the press of [KeyCode::Enter]
    fn press_enter(&mut self) {
        self.prompt.finish();

        self.content.next_prompt();
        if let Some(p) = self.content.get_prompt() {
            self.prompt.set(p);
        } else {
            self.stop();
        }
    }

    /// Handles the press of [KeyCode::Backspace]
    fn press_backspace(&mut self) {
        if self.settings.backspace {
            self.prompt.remove_char();
        }
    }

    /// Handles the press of any [KeyCode::Char]
    fn press_char(&mut self, c: char) {
        self.prompt.type_char(c);
    }

    /// Calculate the current WPM score.
    ///
    /// For the standard calculation of a word per minute score,
    /// each word is set to equal any 5 characters.
    ///
    /// In the current implementation for this calculation, the correct characters are subtracted by the amount of times the backspace was used.
    /// This implementation should change so removing faulty characters does not subtract from the WPM score.
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

    /// Handles the transition to [State::Running]
    fn start(&mut self) {
        self.state = State::Running;
        self.timer.start();
    }

    /// Handles the transition to [State::Pausing]
    fn pause(&mut self) {
        self.state = State::Pausing;
        self.timer.stop();
    }

    /// Handles the transition to [State::Waiting]
    fn restart(&mut self) {
        self.state = State::Waiting;
        self.time = Local::now();

        self.content.reset();
        self.timer.reset();
        self.prompt = Prompt::new(self.content.get_prompt().unwrap());
    }

    /// Handles the transition to [State::Finished]
    fn stop(&mut self) {
        self.timer.stop();
        self.state = State::Finished;

        self.save();
    }

    /// Handles the transition to [State::Quitting]
    pub fn quit(&mut self) {
        self.state = State::Quitting;
    }

    pub fn check_timer(&mut self) {
        if self.timer.timer_expired() {
            self.stop();
        }
    }

    /// Saves the stats and the settings of the [Exercise] in a .json file.
    fn save(&self) {
        if let Ok(mut s) = serde_json::to_string(self) {
            s.push('\n');

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .write(true)
                .open("type_trainer.json");

            if let Ok(mut file) = file {
                if let Ok(_) = file.write(s.as_bytes()) {}
            }
        }
    }
}

/// Serializes the [Exercise] to a .json string
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
