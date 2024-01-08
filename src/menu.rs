use console::Term;

use crate::exercises::{Exercise, ExerciseType};
use std::{
    fs,
    io::{self, Write},
    process::exit,
};

// Use cargo menu instead?

pub fn start() {
    println!("Welcome, to type trainer, you can start your training by typing 'help'");
    let term = Term::stdout();
    loop {
        show_menu(&term);
    }
}

fn show_menu(term: &Term) {
    println!();
    print!("> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Error reading line");

    term.clear_screen().unwrap();
    println!("> {}", buffer);
    let mut input = buffer.as_str().trim().split(" ");

    let command = input
        .next()
        .expect("There should always be at least a command");

    if let Some(exercise_type) = ExerciseType::get_exercise_type(command) {
        if let Some(path) = input.next() {
            // check if the path is valid and we can read valid UTF-8 from the
            if let Ok(content) = fs::read_to_string(path) {
                // todo: read the possible duration option, without it, only the copy exercise will work
                Exercise::build_exercise(exercise_type, content, Option::None).start();
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
