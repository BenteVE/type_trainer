use std::{fs, path::PathBuf, time::Duration};

use crate::exercise::{content::Content, exercise::Exercise, settings::Settings, timer::Timer};
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
            // random and start should not exist together
            // remove split option argument text (long texts can't be show anyway)
            // instead: show 0..=10 lines in the prompt => (for random, we should select multiple lines at the same time?)
            // whenever enter is pressed, we should remove 1 line
            // change random to just shuffle the vector instead of keep selecting the lines?
            // Pro: progress bar would work
            // Con: could take a long time for long texts (need extra crate)
            .arg(
                Arg::new("start")
                    .long("start")
                    .short('s')
                    .help("Determines the line to start at")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u32).range(0..)),
            )
            .arg(
                Arg::new("lines")
                    .long("lines")
                    .short('l')
                    .help("Limit the amount of lines that are prompted.")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u32).range(1..)),
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
                Arg::new("terminate")
                    .long("terminate")
                    .short('t')
                    .help("Terminate the exercise after the given amount of mistakes are made")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u16).range(1..)),
            )
            .arg(
                Arg::new("words")
                    .long("words")
                    .short('w')
                    .help("Split every word of the text.")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("random")
                    .long("random")
                    .short('r')
                    .help("Shuffle the prompts in a random order.")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("backspace")
                    .long("fixed")
                    .short('f')
                    .help("Disable the backspace")
                    .required(false)
                    .action(ArgAction::SetFalse),
            )
            .arg(
                Arg::new("blind")
                    .long("blind")
                    .short('b')
                    .help("Hide the letters you type from the screen")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("highlight")
                    .long("unmark")
                    .short('u')
                    .help("Disable the highlighting of the correct letters in green and the mistakes in red")
                    .required(false)
                    .action(ArgAction::SetFalse),
            )
            .arg(
                Arg::new("auto")
                    .long("auto")
                    .short('a')
                    .help("Automatically progress to the next line without pressing enter.")
                    .required(false)
                    .action(ArgAction::SetTrue),
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
        let words = matches.get_flag("words");

        let mut prompts = Content::into_prompts(content, words);

        if prompts.is_empty() {
            return Err(anyhow!(
                "Could't create any prompts from the file at {}",
                path.to_str()
                    .expect("An invalid path should be caught when we read the file to a string")
            ));
        }

        // Start at the given line
        if let Some(&start) = matches.get_one::<u32>("start") {
            let start = start as usize;
            if start >= prompts.len() {
                return Err(anyhow!("Starting value {} results in 0 prompts", start));
            } else {
                prompts = prompts[start..].to_vec();
            }
        };

        // Shorten the amount of lines if necessary
        if let Some(&lines) = matches.get_one::<u32>("lines") {
            let lines = lines as usize;
            if lines < prompts.len() {
                prompts = prompts[..lines].to_vec();
            }
        }

        let random = matches.get_flag("random");

        Ok(Content::build(path.to_owned(), prompts, random, words))
    }

    // parse the command line arguments to create the settings
    pub fn get_settings(matches: &ArgMatches) -> Result<Settings> {
        let backspace = matches.get_flag("backspace");
        let highlight = matches.get_flag("highlight");
        let blind = matches.get_flag("blind");
        let auto = matches.get_flag("auto");
        let terminate = match matches.get_one::<u16>("terminate") {
            Some(&t) => Some(t as usize),
            None => Option::None,
        };

        Ok(Settings::build(
            backspace, highlight, blind, auto, terminate,
        ))
    }

    pub fn get_timer(matches: &ArgMatches) -> Result<Timer> {
        let duration = match matches.get_one::<u16>("duration") {
            Some(d) => Some(Duration::from_secs(*d as u64)),
            None => Option::None,
        };
        Ok(Timer::new(duration))
    }
}
