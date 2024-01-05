use console::Term;
use edit_distance::edit_distance;
use rand::{seq::SliceRandom, thread_rng};
use std::{fmt, io, time::SystemTime};

use crate::file_handler;

//  change this to the type of exercise
#[derive(PartialEq)]
pub enum ExerciseType {
    Quicktype,
    Copy,
}

impl ExerciseType {
    // read in the file, and create the contents based on the exercise type
    fn build_contents_from_file(&self, file_name: String) -> Vec<String> {
        // call the file handler to read in all the lines
        let contents = file_handler::get_file_content(file_name);

        match self {
            ExerciseType::Quicktype => contents
                .split([' ', '\n'])
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
            ExerciseType::Copy => contents
                .split('\n')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        }
    }
}

// makes the to_string() method available for the variants
impl fmt::Display for ExerciseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExerciseType::Quicktype => write!(f, "quicktype"),
            ExerciseType::Copy => write!(f, "copy"),
        }
    }
}

pub struct Exercise {
    exercise_type: ExerciseType,
    start: Option<SystemTime>,
    duration: Option<usize>, // in seconds
    contents: Vec<String>,
    prompts: usize,
    correct: usize,
    mistakes: usize,
    lines_with_mistakes: Vec<(String, String)>,
}

impl Exercise {
    pub fn build_exercise(
        exercise_type: ExerciseType,
        duration: Option<usize>,
        file_name: String,
    ) -> Exercise {
        Exercise {
            contents: exercise_type.build_contents_from_file(file_name),
            exercise_type: exercise_type,
            start: Option::None,
            duration: duration,
            prompts: 0, // count the total prompts given (used for giving the correct next line in the copy exercise)
            correct: 0, // count the lines without mistakes
            mistakes: 0, // calculated with edit distance
            lines_with_mistakes: Vec::new(), // store a list of tuples with original to typed with mistake
        }
    }

    // start the exercise (loop giving are reading prompts and answers while the timer had not reached the end)
    pub fn start(&mut self) {
        if self.contents.is_empty() {
            println!("There are no contents in the file.");
        }
        // start the exercise timer
        self.start = Some(SystemTime::now());
        loop {
            if self.duration.is_some_and(|d| self.elapsed_time() >= d) {
                break;
            }
            if self.exercise_type == ExerciseType::Copy && self.prompts >= self.contents.len() {
                break;
            }
            // Note: when the timer runs out, it will not immediately stop the exercise
            // but allow you to finish the current prompt
            self.handle_prompt()
        }
        self.save_results();
        self.format_results();
    }

    fn handle_prompt(&mut self) {
        // clear the screen
        let term = Term::stdout();
        term.clear_screen().unwrap();

        // print prompt
        let prompt = match self.exercise_type {
            ExerciseType::Quicktype => self.contents.choose(&mut thread_rng()).unwrap(),
            ExerciseType::Copy => &self.contents[self.prompts],
        };
        term.write_line(prompt).unwrap();
        self.prompts += 1;

        // read the input from the user
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Error reading line");

        // compare prompt and result and count mistakes
        match edit_distance(prompt.trim_end(), &buffer.trim_end()) {
            0 => self.correct += 1,
            edit_distance => {
                self.mistakes += edit_distance;
                self.lines_with_mistakes
                    .push((prompt.clone(), buffer.clone()));
            }
        }
    }

    // save the results to a file
    fn save_results(&self) {}

    fn format_results(&self) {
        Term::stdout().clear_screen().unwrap();
        println!("Time: {}", self.elapsed_time());
        println!("Prompts: {}", self.prompts);
        println!("Correct: {}", self.correct);
        println!("Mistakes: {}", self.mistakes);

        for (orig, fault) in &self.lines_with_mistakes{
            println!("{}", orig.trim_end());
            println!("{}", fault.trim_end());
        }
    }

    fn elapsed_time(&self) -> usize {
        self.start.unwrap().elapsed().unwrap().as_secs() as usize
    }
}
