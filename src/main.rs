use std::{fs, path::PathBuf};

use clap::{command, value_parser, Arg, ArgAction};
use type_trainer::{exercise_type::ExerciseType, exercises::Exercise};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("ExerciseType")
                .index(1)
                .help("The type of exercise")
                .required(true)
                .value_parser(value_parser!(ExerciseType)),
        )
        .arg(
            Arg::new("path")
                .index(2)
                .help("The path to the exercise file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        // optional
        .arg(
            Arg::new("duration")
                .long("duration")
                .short('d')
                .help("Limit of the duration of the exercise in seconds")
                .required(false)
                .action(ArgAction::Set)
                .value_parser(value_parser!(u16).range(1..)),
        )
        // flags
        .arg(
            Arg::new("blind")
                .long("blind")
                .short('b')
                .help("Hide the letters while you are typing")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("backspace")
                .short('c')
                .help("Disable the backspace")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches(); // builds the instance of ArgMatches

    if let Some(path) = matches.get_one::<PathBuf>("path") {
        match fs::read_to_string(path) {
            Ok(content) => {
                let exercise_type = *matches
                    .get_one::<ExerciseType>("ExerciseType")
                    .expect("'ExerciseType' is required and parsing will fail if its missing");
                let duration = matches.get_one::<u16>("duration").copied();
                let _blind = matches.get_flag("blind");
                let _backspace = matches.get_flag("backspace");

                Exercise::build_exercise(exercise_type, content, duration).start();
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
