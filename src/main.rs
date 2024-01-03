use std::{fs, io, process::exit};
use type_trainer::exercises::copy;
use type_trainer::exercises::quicktype;

enum Exercise {
    Quicktype,
    Copy,
}

fn main() {
    println!("Welcome, to type trainer, you can start your training by typing 'help'");
    loop {
        menu();
    }
}

fn menu() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Error reading line");
    let mut input = buffer.as_str().trim().split(" ");

    match input.next().unwrap() {
        "quit" => exit(0),
        "help" => print_help_message(),
        "quicktype" => exercise_option(Exercise::Quicktype, input.next()),
        "copy" => exercise_option(Exercise::Copy, input.next()),
        _ => println!("Invalid option, type 'help' to see the options"),
    }
}

fn exercise_option(exercise: Exercise, argument: Option<&str>) {
    let files = get_files(&exercise);

    if let Some(file_name) = argument {
        if files.contains(&file_name.to_string()) {
            // start exercise
            match exercise {
                Exercise::Quicktype => quicktype::start(),
                Exercise::Copy => copy::start(),
            }
        } else {
            println!("File not found or not a valid exercise file");
        }
    } else {
        println!("Exercise options:");
        for file_name in files {
            println!("{}", file_name);
        }
        println!("You can add other exercises by adding ")
    }
}

fn get_files(exercise: &Exercise) -> Vec<String> {
    let dir = match exercise {
        Exercise::Copy => fs::read_dir("./exercises/copy"),
        Exercise::Quicktype => fs::read_dir("./exercises/quicktype"),
    };
    let dir = dir.expect("Error: exercises folder not found");

    let files = dir.filter(|entry| {
        entry
            .as_ref()
            .is_ok_and(|file| file.file_type().unwrap().is_file())
    });

    let txt_files = files.filter(|file| {
        file.as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .split_terminator(".")
            .last()
            .unwrap()
            == "txt"
    });

    let file_names: Vec<String> = txt_files
        .map(|file| file.unwrap().file_name().to_str().unwrap().to_owned())
        .collect();

    file_names
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
