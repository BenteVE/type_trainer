use std::{fs, path::PathBuf, time::Duration};

use clap::{command, value_parser, Arg, ArgAction};
use type_trainer::{exercise::Exercise, settings::Settings, split::Split};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("path")
                .index(1)
                .help("The path to the exercise file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        // Determine how to split the content of the file into prompts
        .arg(
            Arg::new("split")
                .index(2)
                .help("Determines how to split the contents of the file into prompts")
                .required(true)
                .value_parser(value_parser!(Split)),
        )
        // Stats
        .arg(
            Arg::new("duration")
                .long("duration")
                .short('d')
                .help("Limit of the duration of the exercise in seconds")
                .required(false)
                .action(ArgAction::Set)
                .value_parser(value_parser!(u16).range(1..)),
        )
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
                .action(ArgAction::SetFalse),
        )
        .get_matches(); // builds the instance of ArgMatches

    if let Some(path) = matches.get_one::<PathBuf>("path") {
        match fs::read_to_string(path) {
            Ok(content) => {
                let split = matches
                    .get_one::<Split>("split")
                    .expect("'split' is required and parsing will fail if its missing")
                    .to_owned();

                let duration = match matches.get_one::<u16>("duration") {
                    Some(d) => Some(Duration::from_secs(*d as u64)),
                    None => Option::None,
                };
                let blind = matches.get_flag("blind");
                let backspace = matches.get_flag("backspace");

                let settings = Settings::build(path.to_owned(), split, duration, blind, backspace);

                let prompts = settings.split_content_into_prompts(content);

                Exercise::build(prompts, settings).start();
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
