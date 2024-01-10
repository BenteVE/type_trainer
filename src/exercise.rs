use crate::{settings::Settings, stats::Stats};

use crossterm::cursor::{MoveLeft, MoveTo};
use crossterm::event::{poll, read, Event};
use crossterm::style::Print;
use crossterm::{
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::fs::OpenOptions;
use std::io::{self, Write};
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
        // start the exercise timer
        // self.start = Some(Instant::now());

        execute!(io::stdout(), EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();

        // count the given prompts
        execute!(
            io::stdout(),
            Print("Handle the prompts based on the settings")
        )
        .unwrap();
        // rand::thread_rng().gen_range(0..self.contents.len());

        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();

        println!("Print the serialized stats");
        //Exercise::save_results(&results);
        //Exercise::print_results(&results);
    }

    // return Ok(true) if the exercise should continue and Ok(false) if it should stop
    fn handle_prompt(&mut self, prompt: usize) -> Result<bool, io::Error> {
        let prompt = &self.prompts[prompt];
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
                            // Do handle the character based on the stats
                            execute!(io::stdout(), Print(c))?;
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
            }
            // check if the duration has expired
        }
    }
}