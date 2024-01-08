use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use crate::exercises::{Exercise, ExerciseType};
use std::{
    fs,
    io::{self, Write},
    process::exit,
};

// Todo: create a better menu with simpler expandable argument parsing

pub fn start() {
    println!("Welcome, to type trainer, you can start your training by typing 'help'");
    loop {
        show_menu();
    }
}

fn show_menu() {
    println!();
    print!("> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Error reading line");

    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

    println!("> {}", buffer);
    let mut input = buffer.as_str().trim().split(" ");

    let command = input
        .next()
        .expect("There should always be at least a command");

    if let Some(exercise_type) = ExerciseType::get_exercise_type(command) {
        if let Some(path) = input.next() {
            // check if the path is valid and we can read valid UTF-8 from the
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(duration) = input.next() {
                    if let Ok(duration) = duration.parse::<usize>() {
                        Exercise::build_exercise(exercise_type, content, Some(duration)).start();
                    } else {
                        println!("Invalid duration");
                    }
                } else if exercise_type == ExerciseType::Copy {
                    Exercise::build_exercise(exercise_type, content, Option::None).start();
                } else {
                    println!("The quicktype exercise requires a duration");
                }
            } else {
                println!("Invalid file");
            }
        } else {
            println!("Missing arguments");
        }
    } else {
        // match any static arguments
        match command {
            "quit" => exit(0),
            "help" => print_help_message(),
            _ => println!("Invalid option, type 'help' to see the options"),
        }
    }
}

fn print_help_message() {
    println!("There are two exercises:");
    println!("\tquicktype: wait for a word to appear and type it as fast as possible, the test ends when you make a mistake");
    println!("\tcopy: copy a text, when a mistake is made, you have to start the whole word again");
    println!("Type one of the following options:");
    println!("\t'exercise_name' to view the valid file names for the exercise");
    println!("\t'exercise_name file_name.txt' to start an exercise");
    println!("\t'quit' to quit the program");
    println!("\t'help' to view this message again");
}
