use std::{fs, path::PathBuf, time::Duration};

use crate::exercise::{settings::Settings, split::Split};
use clap::{command, value_parser, Arg, ArgAction, ArgMatches};

use anyhow::{anyhow, Result};

pub struct Parser;

impl Parser {
    pub fn new() -> ArgMatches {
        command!()
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
            .arg(
                Arg::new("random")
                    .short('r')
                    .help("Give the prompts in random order instead of consecutive order")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .get_matches()
    }

    // parse the command line arguments to create the settings
    pub fn parse(matches: ArgMatches) -> Result<Settings> {
        let path = matches
            .get_one::<PathBuf>("path")
            .expect("Path is required");

        let content = fs::read_to_string(path)?;

        let split = matches
            .get_one::<Split>("split")
            .expect("'split' is required and parsing will fail if its missing")
            .to_owned();

        let prompts = split.into_prompts(content);

        if prompts.is_empty() {
            return Err(anyhow!(
                "Could't create any prompts from the file at {}",
                path.to_str()
                    .expect("An invalid path should be caught when we read the file to a string")
            ));
        }

        let duration = match matches.get_one::<u16>("duration") {
            Some(d) => Some(Duration::from_secs(*d as u64)),
            None => Option::None,
        };
        let blind = matches.get_flag("blind");
        let backspace = matches.get_flag("backspace");
        let random = matches.get_flag("random");

        Ok(Settings::build(
            path.to_owned(),
            split,
            prompts,
            duration,
            blind,
            backspace,
            random,
        ))
    }
}
