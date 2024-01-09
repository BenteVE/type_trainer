use clap::{builder::PossibleValue, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
}

impl std::fmt::Display for ExerciseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl ValueEnum for ExerciseType {
    fn value_variants<'a>() -> &'a [Self] {
        &[ExerciseType::Quicktype, ExerciseType::Copy]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            ExerciseType::Quicktype => PossibleValue::new("Quicktype").help("Select a random word from the text"),
            ExerciseType::Copy => PossibleValue::new("Copy").help("Copy the text line by line"),
        })
    }
}

impl std::str::FromStr for ExerciseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
