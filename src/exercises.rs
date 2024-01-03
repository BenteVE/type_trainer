use std::{fmt, fs};

pub enum Exercise {
    Quicktype,
    Copy,
}

impl Exercise {
    pub fn start(&self, argument: Option<&str>) {
        let files = self.get_files();

        if let Some(file_name) = argument {
            if files.contains(&file_name.to_string()) {
                match self {
                    Exercise::Quicktype => println!("starting the quicktype exercise"),
                    Exercise::Copy => println!("starting the copy exercise"),
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
