use crate::exercise::{settings::Settings, stats::Stats};

use clap::builder::Str;
use crossterm::cursor::{MoveLeft, MoveRight, MoveTo};
use crossterm::event::{poll, read, Event, KeyEventKind};
use crossterm::style::Print;
use crossterm::{
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use rand::Rng;
use std::io;
use std::time::{Duration, Instant};

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

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    // start the exercise (loop giving are reading prompts and answers while the timer had not reached the end)
    pub fn start(&mut self) {
        execute!(io::stdout(), EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();

        // build the alternate screen with ratatui

        // start the exercise timer
        self.stats.start = Some(Instant::now());

        loop {
            match self.handle_prompt() {
                Ok(true) => {}
                Ok(false) => break,
                Err(e) => panic!("{}", e.to_string()),
            }
        }

        self.stats.end = Some(Instant::now());

        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();

        println!("Print the serialized stats");
        println!("{}", serde_json::to_string(&self.settings).unwrap());
        println!("{}", serde_json::to_string(&self.stats).unwrap());

        // write serializing to file
    }

    // return Ok(true) if the exercise should continue and Ok(false) if it should stop
    fn handle_prompt(&mut self) -> Result<bool, io::Error> {

        loop {
            // use non-blocking read to be able to stop while the timer runs
            if poll(Duration::from_millis(50))? {
                if let Event::Key(key) = read()? {
                    // compare the key with the character that is supposed to be typed
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            crossterm::event::KeyCode::Char(c) => {
                                match self.settings.blind {
                                    true => execute!(io::stdout(), MoveRight(1))?,
                                    false => execute!(io::stdout(), Print(c))?,
                                }
                                // handle the counting of the correct or wrong characters

                                //typed.push(c);
                            }
                            crossterm::event::KeyCode::Backspace if self.settings.backspace => {
                                //typed.pop();
                                execute!(
                                    io::stdout(),
                                    MoveLeft(1),
                                    Clear(ClearType::FromCursorDown)
                                )?
                            }
                            crossterm::event::KeyCode::Enter => {
                                // Any missing are also mistakes (extra chars are already counted when the characters were typed)

                                return Ok(true);
                            }
                            crossterm::event::KeyCode::Esc => {
                                return Ok(false);
                            }
                            _ => {}
                        }
                    }
                }
            } else if self.timer_expired() {
                return Ok(false);
            }
        }
    }

    // return true if false if the timer has expired
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
