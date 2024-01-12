use std::{path::PathBuf, time::Duration};

use rand::Rng;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::exercise::split::Split;

// The settings are created before the exercise starts and
// cannot be changed during the exercise
pub struct Settings {
    pub file_path: PathBuf,
    pub split: Split, // the manner in which the text was split into prompts
    pub prompts: Vec<String>,
    pub duration: Option<Duration>,
    pub blind: bool,     // hide the text when the user is typing
    pub backspace: bool, // allow the use of the backspace key
    pub random: bool,
}
impl Settings {
    pub fn build(
        file_path: PathBuf,
        split: Split,
        prompts: Vec<String>,
        duration: Option<Duration>,
        blind: bool,
        backspace: bool,
        random: bool,
    ) -> Settings {
        Settings {
            file_path: file_path,

            split,
            prompts,
            duration,
            blind,
            backspace,
            random,
        }
    }

    pub fn next_prompt(&self, prompt: usize) -> Option<usize> {
        match self.random {
            true => Some(rand::thread_rng().gen_range(0..self.prompts.len())),
            false => {
                if prompt >= self.prompts.len() {
                    return Option::None;
                } else {
                    Some(prompt + 1)
                }
            }
        }
    }
}

impl Serialize for Settings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 5)?;
        state.serialize_field("file", &self.file_path.file_name().unwrap().to_str())?;
        state.serialize_field("split", &self.split.to_string())?;
        state.serialize_field("blind", &self.blind)?;
        state.serialize_field("backspace", &self.backspace)?;
        state.serialize_field("random", &self.random)?;
        state.end()
    }
}
