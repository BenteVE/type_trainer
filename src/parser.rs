use std::{fs, path::PathBuf, time::Duration};

use crate::exercise::{
    content::Content, exercise::Exercise, settings::Settings, split::Split, timer::Timer,
};
use clap::{command, value_parser, Arg, ArgAction, ArgMatches};

use anyhow::{anyhow, Ok, Result};

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
            // random and start should not exist together
            // remove split option argument text (long texts can't be show anyway)
            // instead: show 0..=10 lines in the prompt => (for random, we should select multiple lines at the same time?)
            // whenever enter is pressed, we should remove 1 line
            // change random to just shuffle the vector instead of keep selecting the lines?
            .arg(
                Arg::new("start")
                    .long("start")
                    .short('s')
                    .help("Allows you to select the line to start at")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u32).range(0..)),
            )
            .arg(
                Arg::new("random")
                    .short('r')
                    .help("Give the prompts in random order instead of consecutive order")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
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
        let timer = Self::get_timer(matches)?;
        let content = Self::get_content(matches)?;
        let settings = Self::get_settings(matches)?;

        Ok(Exercise::build(timer, content, settings))
    }

    pub fn get_content(matches: &ArgMatches) -> Result<Content> {
        let path = matches
            .get_one::<PathBuf>("path")
            .expect("Path is required");

        let content = fs::read_to_string(path)?;

        let split = matches
            .get_one::<Split>("split")
            .expect("'split' is required and parsing will fail if its missing")
            .to_owned();

        let content = split.into_prompts(content);

        if content.is_empty() {
            return Err(anyhow!(
                "Could't create any prompts from the file at {}",
                path.to_str()
                    .expect("An invalid path should be caught when we read the file to a string")
            ));
        }

        let start = match matches.get_one::<u32>("start") {
            Some(start) => *start as usize,
            None => 0,
        };

        if start >= content.len() {
            return Err(anyhow!("Starting value {} results in 0 prompts", start));
        }

        let random = matches.get_flag("random");

        Ok(Content::build(
            path.to_owned(),
            split,
            content[start..].to_vec(),
            random,
        ))
    }

    // parse the command line arguments to create the settings
    pub fn get_settings(matches: &ArgMatches) -> Result<Settings> {
        let backspace = matches.get_flag("backspace");
        let marker = matches.get_flag("marker");
        let blind = matches.get_flag("blind");

        Ok(Settings::build(backspace, marker, blind))
    }

    pub fn get_timer(matches: &ArgMatches) -> Result<Timer> {
        let duration = match matches.get_one::<u16>("duration") {
            Some(d) => Some(Duration::from_secs(*d as u64)),
            None => Option::None,
        };
        Ok(Timer::new(duration))
    }
}
