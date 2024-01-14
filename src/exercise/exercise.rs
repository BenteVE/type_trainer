use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::exercise::settings::Settings;

use super::{content::Content, prompt::Prompt, timer::Timer};

pub struct Exercise {
    pub time: DateTime<Local>,

    pub timer: Timer,
    pub content: Content,
    pub settings: Settings,
    pub prompt: Prompt,

    should_quit: bool,
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
            should_quit: false,
            content,
        }
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

    fn press_enter(&mut self) {
        self.prompt.finish();

        self.content.next_prompt();
        if let Some(p) = self.content.get_prompt() {
            self.prompt.set(p);
        } else {
            self.quit();
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
        state.serialize_field("content", &self.content)?;
        state.serialize_field("settings", &self.settings)?;
        state.serialize_field("stats", &self.prompt)?;
        state.end()
    }
}
