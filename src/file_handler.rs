use std::fs;

pub fn get_file_content(file_name: String) -> String {
    // todo: handle invalid file
    fs::read_to_string(get_path() + &file_name).unwrap()
}

pub fn get_file_names() -> Vec<String> {
    let dir = fs::read_dir(get_path()).expect("Error: exercises folder not found");

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

// the path to the folder with exercises
fn get_path() -> String {
    String::from("./exercises/")
}
