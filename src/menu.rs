use crate::exercises::Exercise;
use std::{io, process::exit};

pub fn start() {
    println!("Welcome, to type trainer, you can start your training by typing 'help'");
    loop {
        show_menu();
    }
}

fn show_menu() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Error reading line");
    let mut input = buffer.as_str().trim().split(" ");

    match input.next().unwrap() {
        "quit" => exit(0),
        "help" => print_help_message(),
        e1 if Exercise::Copy.to_string() == e1 => Exercise::Copy.start(input.next()),
        e2 if Exercise::Quicktype.to_string() == e2 => Exercise::Quicktype.start(input.next()),
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
