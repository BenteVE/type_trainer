use console::Term;
use rand::{seq::SliceRandom, thread_rng};
use std::{fmt, fs, io, time::SystemTime};

pub enum Exercise {
    Quicktype,
    Copy,
}

impl Exercise {
    pub fn start(&self, argument: Option<&str>) {
        let files = self.get_files();

        if let Some(file_name) = argument {
            if files.contains(&file_name.to_string()) {
                // read in the correct file
                let contents = fs::read_to_string(self.get_path() + "/" + file_name)
                    .expect("Should have been able to read the file");
                if contents.is_empty() {
                    println!("The file {} is empty", file_name);
                    return;
                }

                // start a timer
                let start = SystemTime::now();

                // read the input from the user
                let mut buffer = String::new();
                let stdin = io::stdin();

                match self {
                    Exercise::Quicktype => {
                        // quicktype: choose a random word to let the user type
                        let words: Vec<&str> = contents.split([' ', '\n']).collect();
                        let mut rng = thread_rng();

                        let mut correct_answers = 0;
                        // the exercise will last for 3 minutes
                        while start.elapsed().unwrap().as_secs() < 20 {
                            // show the user a word
                            let word = words.choose(&mut rng).unwrap();

                            let term = Term::stdout();
                            term.clear_screen().unwrap();
                            term.write_line(word).unwrap();

                            // read the line from the user
                            buffer.clear();
                            stdin.read_line(&mut buffer).expect("Error reading line");

                            if word == &buffer.trim_end() {
                                correct_answers += 1;
                            }
                        }
                        println!(
                            "You typed {} correct answers in 20 seconds",
                            correct_answers
                        );
                    }
                    Exercise::Copy => {
                        // copy: put all lines in an iterator and let the user type them in order
                        for line in contents.lines() {
                            println!("{}", line);
                        }
                    }
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

    fn get_path(&self) -> String {
        String::from("./exercises/") + &self.to_string()
    }

    fn get_files(&self) -> Vec<String> {
        let dir = fs::read_dir(self.get_path()).expect("Error: exercises folder not found");

        // filter the files from the directory
        let files = dir.filter(|entry| {
            entry
                .as_ref()
                .is_ok_and(|file| file.file_type().unwrap().is_file())
        });

        // filter the txt files from the directory
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

        // collect the file names
        let file_names: Vec<String> = txt_files
            .map(|file| file.unwrap().file_name().to_str().unwrap().to_owned())
            .collect();

        file_names
    }
}

// makes the to_string() method available for the variants
impl fmt::Display for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exercise::Quicktype => write!(f, "quicktype"),
            Exercise::Copy => write!(f, "copy"),
        }
    }
}
