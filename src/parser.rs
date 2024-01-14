use std::{fs, path::PathBuf, time::Duration};

use crate::exercise::{exercise::Exercise, settings::Settings, split::Split, timer::Timer};
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
            .arg(
                Arg::new("marker")
                    .short('m')
                    .help("Mark the correct chars in green and the mistakes in red")
                    .required(false)
                    .action(ArgAction::SetFalse),
            )
            .get_matches()
    }

    // parse the command line arguments to create the settings
    pub fn get_exercise(matches: &ArgMatches) -> Result<Exercise> {
        let path = matches
            .get_one::<PathBuf>("path")
            .expect("Path is required");

        let content = fs::read_to_string(path)?;

        let split = matches
            .get_one::<Split>("split")
            .expect("'split' is required and parsing will fail if its missing")
            .to_owned();

        let contents = split.into_prompts(content);

        if contents.is_empty() {
            return Err(anyhow!(
                "Could't create any prompts from the file at {}",
                path.to_str()
                    .expect("An invalid path should be caught when we read the file to a string")
            ));
        }

        let timer = Self::get_timer(matches)?;
        let settings = Self::get_settings(matches)?;

        Ok(Exercise::build(
            timer,
            settings,
            path.to_owned(),
            split,
            contents,
        ))
    }

    // parse the command line arguments to create the settings
    pub fn get_settings(matches: &ArgMatches) -> Result<Settings> {
        let backspace = matches.get_flag("backspace");
        let random = matches.get_flag("random");
        let marker = matches.get_flag("marker");
        let blind = matches.get_flag("blind");

        Ok(Settings::build(backspace, random, marker, blind))
    }

    pub fn get_timer(matches: &ArgMatches) -> Result<Timer> {
        let duration = match matches.get_one::<u16>("duration") {
            Some(d) => Some(Duration::from_secs(*d as u64)),
            None => Option::None,
        };
        Ok(Timer::new(duration))
    }
}
