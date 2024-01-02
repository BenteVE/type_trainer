use dialoguer::Select;
use std::{fs, process::exit};

fn main() {
    loop {
        menu();
    }
}

fn menu() {
    let options = vec!["quicktype", "copy", "[x] quit"];

    let selection = Select::new()
        .with_prompt("What do you want to do?")
        .items(&options)
        .interact()
        .unwrap();
    println!("");

    if selection == options.len() - 1 {
        exit(0);
    }    

    // This is the base folder that should contain another folder for each exercise
    let folder_path = String::from("./exercises/");

    let files =
        fs::read_dir(folder_path + options[selection]).expect("Error: exercises folder not found");
    let mut files: Vec<String> = files
        .map(|file| file.unwrap().file_name().to_str().unwrap().to_owned())
        .collect();

    if files.is_empty() {
        println!("No exercises found in the {} folder!", options[selection]);
        println!("You can add exercises by adding text files to the folder");
        println!("");
        // This only works for Linux
        std::process::Command::new("clear").status().unwrap();
        return;
    }

    files.push(String::from("[<-] back"));

    let selection = Select::new()
        .with_prompt("Choose an exercise:")
        .items(&files)
        .interact()
        .unwrap();
    println!("");

    if selection == options.len() - 1 {
        // This only works for Linux
        std::process::Command::new("clear").status().unwrap();
        return;
    }

    println!("You chose: {}", files[selection]);
}
