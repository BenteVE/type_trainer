use console::Term;

use crate::{
    exercises::{Exercise, ExerciseType},
    file_handler,
};
use std::{
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

    match input.next().unwrap() {
        "quit" => exit(0),
        "help" => print_help_message(),
        // todo: rework this with a new menu
        exercise if exercise == ExerciseType::Copy.to_string() => {
            if let Some(file_name) = input.next() {
                Exercise::build_exercise(ExerciseType::Copy, Option::None, file_name.to_owned())
                    .start();
            } else {
                println!("file name options");
                for file_name in file_handler::get_file_names() {
                    println!("{}", file_name);
                }
            }
        }
        exercise if exercise == ExerciseType::Quicktype.to_string() => {
            if let Some(file_name) = input.next() {
                Exercise::build_exercise(ExerciseType::Quicktype, Some(30), file_name.to_owned())
                    .start();
            } else {
                println!("file name options");
                for file_name in file_handler::get_file_names() {
                    println!("{}", file_name);
                }
            }
        }
        _ => println!("Invalid option, type 'help' to see the options"),
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
