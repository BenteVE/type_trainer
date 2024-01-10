use std::{path::PathBuf, time::Duration};

use crate::split::Split;

pub struct Settings {
    pub file_path: PathBuf,
    pub split: Split, // the manner in which the text was split into prompts
    pub duration: Option<Duration>,
    pub blind: bool,     // hide the text when the user is typing
    pub backspace: bool, // allow the use of the backspace key
    pub random: bool,
}
impl Settings {
    pub fn build(
        file_path: PathBuf,
        split: Split,
        duration: Option<Duration>,
        blind: bool,
        backspace: bool,
        random: bool,
    ) -> Settings {
        Settings {
            file_path: file_path,
            split,
            duration,
            blind,
            backspace,
            random,
        }
    }
}

// Implement serialize for settings
