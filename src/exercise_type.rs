use std::fmt;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, EnumIter)]
pub enum ExerciseType {
    Quicktype,
    Copy,
}

impl ExerciseType {
    // split the content of a file based on the type of exercise
    pub fn split_content(&self, content: String) -> Vec<String> {
        match self {
            ExerciseType::Quicktype => content.split([' ', '\n']),
            ExerciseType::Copy => content.split(['\n', '\n']),
        }
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
    }

    // return an exerciseType based on the given string
    pub fn get_exercise_type(arg: &str) -> Option<ExerciseType> {
        for exercise_type in ExerciseType::iter() {
            if arg == exercise_type.to_string() {
                return Some(exercise_type);
            }
        }
        return Option::None;
    }
}

// makes the to_string() method available for the variants
impl fmt::Display for ExerciseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExerciseType::Quicktype => write!(f, "quicktype"),
            ExerciseType::Copy => write!(f, "copy"),
        }
    }
}
