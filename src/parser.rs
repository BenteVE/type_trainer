use crate::app::{content::Content, exercise::Exercise, settings::Settings, timer::Timer};
use anyhow::{anyhow, Ok, Result};
use clap::{command, value_parser, Arg, ArgAction, ArgMatches};
use std::{fs, path::PathBuf, time::Duration};

/// Specifies all possible command line arguments for the application
pub fn create_commands() -> ArgMatches {
    command!()
            .arg(
                Arg::new("path")
                    .index(1)
                    .help("The path to the file you want to use for training")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                Arg::new("start")
                    .long("start")
                    .short('s')
                    .help("Select the starting line of the exercise")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u32).range(0..)),
            )
            .arg(
                Arg::new("prompts")
                    .long("prompts")
                    .short('p')
                    .help("Limit the amount of prompts")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u32).range(1..)),
            )
            .arg(
                Arg::new("duration")
                    .long("duration")
                    .short('d')
                    .value_name("seconds")
                    .help("Limit of the duration of the exercise in seconds")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u16).range(1..)),
            )
            .arg(
                Arg::new("terminate")
                    .long("terminate")
                    .short('t')
                    .value_name("mistakes")
                    .help("Terminate the exercise after the given amount of mistakes are made")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(u16).range(1..)),
            )
            .arg(
                Arg::new("words")
                    .long("words")
                    .short('w')
                    .help("Split every word of the text into a separate prompt")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("random")
                    .long("random")
                    .short('r')
                    .help("Shuffle the prompts in a random order")
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
                    .help("Automatically progress to the next line without pressing enter")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("correct")
                    .long("correct")
                    .short('c')
                    .help("Only progress when the prompt is completely correct")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("repeat")
                    .long("repeat")
                    .short('q')
                    .help("Repeat the prompt when a mistake was made while typing it")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .get_matches()
}

/// Parse the command line arguments to create the [Exercise]
pub fn get_exercise(matches: &ArgMatches) -> Result<Exercise> {
    let timer = get_timer(matches)?;
    let content = get_content(matches)?;
    let settings = get_settings(matches)?;

    Ok(Exercise::build(timer, content, settings))
}

/// Parse the command line arguments to create the [Content]
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
    if let Some(&p) = matches.get_one::<u32>("prompts") {
        let p = p as usize;
        if p < prompts.len() {
            prompts = prompts[..p].to_vec();
        }
    }

    let random = matches.get_flag("random");

    Ok(Content::build(path.to_owned(), prompts, random, words))
}

/// Parse the command line arguments to create the [Settings]
pub fn get_settings(matches: &ArgMatches) -> Result<Settings> {
    let backspace = matches.get_flag("backspace");
    let highlight = matches.get_flag("highlight");
    let blind = matches.get_flag("blind");
    let auto = matches.get_flag("auto");
    let correct = matches.get_flag("correct");
    let repeat = matches.get_flag("repeat");
    let terminate = match matches.get_one::<u16>("terminate") {
        Some(&t) => Some(t as usize),
        None => Option::None,
    };

    Ok(Settings::build(
        backspace, highlight, blind, auto, correct, repeat, terminate,
    ))
}

/// Parse the command line arguments to create the [Timer]
pub fn get_timer(matches: &ArgMatches) -> Result<Timer> {
    let duration = match matches.get_one::<u16>("duration") {
        Some(d) => Some(Duration::from_secs(*d as u64)),
        None => Option::None,
    };
    Ok(Timer::new(duration))
}
