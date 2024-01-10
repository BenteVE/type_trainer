use crate::{settings::Settings, stats::Stats};

use crossterm::cursor::{MoveLeft, MoveRight, MoveTo};
use crossterm::event::{poll, read, Event};
use crossterm::style::Print;
use crossterm::{
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::io;
use std::time::{Duration, Instant};

pub struct Exercise {
    prompts: Vec<String>,
    settings: Settings,
    stats: Stats,
}

impl Exercise {
    pub fn build(prompts: Vec<String>, settings: Settings) -> Exercise {
        Exercise {
            prompts,
            settings,
            stats: Stats::new(),
        }
    }

    // start the exercise (loop giving are reading prompts and answers while the timer had not reached the end)
    pub fn start(&mut self) {
        execute!(io::stdout(), EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();

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
    }

    // return Ok(true) if the exercise should continue and Ok(false) if it should stop
    fn handle_prompt(&mut self) -> Result<bool, io::Error> {
        let index = match self.settings.random {
            true => rand::thread_rng().gen_range(0..self.prompts.len()),
            false => {
                if self.stats.count_prompts >= self.prompts.len() {
                    return Ok(false);
                } else {
                    self.stats.count_prompts
                }
            }
        };

        self.stats.count_prompts += 1;

        let prompt = self.prompts[index].clone();
        let mut typed = String::new();

        execute!(
            io::stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(prompt),
            Print("\r\n")
        )?;

        loop {
            // use non-blocking read to be able to stop while the timer runs
            if poll(Duration::from_millis(50))? {
                if let Event::Key(key) = read()? {
                    // compare the key with the character that is supposed to be typed
                    match key.code {
                        crossterm::event::KeyCode::Char(c) => {
                            match self.settings.blind {
                                true => execute!(io::stdout(), MoveRight(1))?,
                                false => execute!(io::stdout(), Print(c))?,
                            }
                            // handle the counting of the correct or wrong characters

                            typed.push(c);
                        }
                        crossterm::event::KeyCode::Backspace if self.settings.backspace => {
                            typed.pop();
                            execute!(io::stdout(), MoveLeft(1), Clear(ClearType::FromCursorDown))?
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
