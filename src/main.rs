use std::{fs, io, process::exit};

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
        "quicktype" => {
            let dir =
                fs::read_dir("./exercises/quicktype").expect("Error: exercises folder not found");
            let filter = dir.filter(|entry| {
                entry.as_ref().is_ok_and(|file| {
                    file.file_type().unwrap().is_file()
                        && file
                            .file_name()
                            .to_str()
                            .unwrap()
                            .split_terminator(".")
                            .last()
                            .unwrap()
                            == "txt"
                })
            });
            let file_names: Vec<_> = filter
                .map(|file| file.unwrap().file_name().to_str().unwrap().to_owned())
                .collect();

            if let Some(file_name) = input.next() {
                if file_names.contains(&file_name.to_string()) {
                    // start exercise
                    println!("Starting the quicktype exercise for {}", file_name);
                } else {
                    println!("File not found");
                }
            } else {
                println!("Quicktype exercise options:");
                for file_name in file_names {
                    println!("{}", file_name);
                }
                println!("You can add other exercises by adding ")
            }
        }

        "copy" => {}
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
