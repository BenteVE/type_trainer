use std::{path::PathBuf, time::Duration};

use crate::split::Split;

pub struct Settings {
    pub file_path: PathBuf,
    pub split: Split, // the manner in which the text was split into prompts
    pub duration: Option<Duration>,
    pub blind: bool,     // hide the text when the user is typing
    pub backspace: bool, // allow the use of the backspace key
}
impl Settings {
    pub fn build(
        file_path: PathBuf,
        split: Split,
        duration: Option<Duration>,
        blind: bool,
        backspace: bool,
    ) -> Settings {
        Settings {
            file_path: file_path,
            split,
            duration,
            blind,
            backspace,
        }
    }

    pub fn split_content_into_prompts(&self, content: String) -> Vec<String> {
        let prompts: Vec<String> = match self.split {
            Split::Words => content
                .split([' ', '\n'])
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect(),
            Split::Lines => content
                .split(['\n'])
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect(),
            Split::Text => vec![content],
        };

        if prompts.is_empty() {
            panic!("There are no contents in the file.");
        }

        prompts
    }
}

// Implement serialize for settings
