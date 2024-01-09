use crossterm::cursor::{MoveLeft, MoveRight, MoveTo};
use crossterm::event::{poll, read, Event};
use crossterm::style::Print;
use crossterm::{
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::io;
use std::time::Duration;
use std::time::SystemTime;

use crate::exercise_type::ExerciseType;

pub struct Exercise {
    exercise_type: ExerciseType,
    start: Option<SystemTime>,
    duration: Option<u16>, // in seconds
    blind: bool,           // hide the text when the user is typing
    backspace: bool,       // allow the use of the backspace key
    contents: Vec<String>,
    count_correct: usize, // the characters do not need to be submitted for them to count
    count_fault: usize,
    // count the amount of times the backspace was used

    // Possibly add option to hide the cursor

    // Possibly store all typed lines with with the originals in a tuple to so we can show the user the text 
    // after the exercises is completed
}

impl Exercise {
    pub fn build_exercise(
        exercise_type: ExerciseType,
        content: String,
        duration: Option<u16>,
        blind: bool,
        backspace: bool,
    ) -> Exercise {
        Exercise {
            contents: exercise_type.split_content(content),
            exercise_type: exercise_type,
            start: Option::None,
            duration,
            blind,
            backspace,
            count_correct: 0,
            count_fault: 0,
        }
    }

    // start the exercise (loop giving are reading prompts and answers while the timer had not reached the end)
    pub fn start(&mut self) {
        if self.contents.is_empty() {
            println!("There are no contents in the file.");
        }

        // todo: do a countdown

        // start the exercise timer
        self.start = Some(SystemTime::now());
        // try to add a timer permanently on top of the screen with counters for correct and mistakes

        execute!(io::stdout(), EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();

        match self.exercise_type {
            ExerciseType::Quicktype => loop {
                let prompt = rand::thread_rng().gen_range(0..self.contents.len());
                if let Ok(false) = self.handle_prompt(prompt) {
                    break;
                }
            },
            ExerciseType::Copy => {
                for prompt in 0..self.contents.len() {
                    if let Ok(false) = self.handle_prompt(prompt) {
                        break;
                    }
                }
            }
        }

        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();

        self.save_results();
        self.format_results();
    }

    // return Ok(true) if the exercise should continue and Ok(false) if it should stop
    fn handle_prompt(&mut self, prompt: usize) -> Result<bool, io::Error> {
        let prompt = &self.contents[prompt];
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
                            match self.blind {
                                true => execute!(io::stdout(), MoveRight(1))?,
                                // todo: print in a red color when a mistake is made
                                false => execute!(io::stdout(), Print(c))?,
                            }

                            if typed.len() + 1 > prompt.len()
                                || prompt.chars().nth(typed.chars().count()).unwrap() != c
                            {
                                self.count_fault += 1;
                            } else {
                                self.count_correct += 1;
                            }
                            typed.push(c);
                        }
                        crossterm::event::KeyCode::Backspace if self.backspace => {
                            typed.pop();
                            execute!(io::stdout(), MoveLeft(1), Clear(ClearType::FromCursorDown))?
                        }
                        crossterm::event::KeyCode::Enter => {
                            // Any missing are also mistakes (extra chars are already counted when the characters were typed)

                            if prompt.chars().count() > typed.chars().count() {
                                self.count_fault += prompt.chars().count() - typed.chars().count();
                            }

                            return Ok(true);
                        }
                        crossterm::event::KeyCode::Esc => {
                            return Ok(false);
                        }
                        _ => {}
                    }
                }
            } else if let Some(duration) = self.duration {
                if self.elapsed_time() > duration as usize {
                    return Ok(false);
                }
            }
        }
    }

    // save the results to a file
    fn save_results(&self) {}

    fn format_results(&self) {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        println!("Time: {}", self.elapsed_time());

        println!("Correct characters: {}", self.count_correct);
        println!("Mistakes: {}", self.count_fault);
    }

    fn elapsed_time(&self) -> usize {
        self.start.unwrap().elapsed().unwrap().as_secs() as usize
    }
}
