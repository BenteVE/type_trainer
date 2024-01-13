use std::path::PathBuf;

use rand::Rng;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use super::split::Split;

pub struct Prompt {
    pub file_path: PathBuf,
    pub split: Split,
    pub random: bool,

    pub prompts: Vec<String>,
    pub prompts_index: usize, // the index of the current prompt

    pub orig: Vec<char>,
    pub copy: Vec<char>,
}

impl Prompt {
    pub fn build(file_path: PathBuf, split: Split, random: bool, prompts: Vec<String>) -> Prompt {
        let first_prompt = match random {
            true => rand::thread_rng().gen_range(0..prompts.len()),
            false => 0,
        };
        let mut prompt = Prompt {
            file_path,
            split,
            random,
            prompts,
            prompts_index: first_prompt,
            orig: Vec::new(),
            copy: Vec::new(),
        };
        prompt.set_prompt();
        prompt
    }

    /// Return if the character was correct
    pub fn type_char(&mut self, c: char) -> bool {
        let index = self.copy.len();
        self.copy.push(c);

        match index >= self.orig.len() {
            true => false,
            false => c == self.orig[index],
        }
    }

    pub fn remove_char(&mut self) {
        if self.copy.len() > 0 {
            self.copy.pop();
        }
    }

    pub fn handle_enter(&mut self) {
        match self.split {
            Split::Text => todo!(),
            _ => {
                self.next_prompt();
            }
        }
    }

    fn next_prompt(&mut self) {
        self.select_next_prompt();
        self.set_prompt();
    }

    fn select_next_prompt(&mut self) {
        match self.random {
            true => self.prompts_index = rand::thread_rng().gen_range(0..self.prompts.len()),
            false => self.prompts_index += 1,
        }
    }

    fn set_prompt(&mut self) {
        self.orig = self.prompts[self.prompts_index].chars().collect();
        self.copy = Vec::new();
    }

    pub fn count_missing(&self) -> usize {
        if self.orig.len() > self.copy.len() {
            self.orig.len() - self.copy.len()
        } else {
            0
        }
    }
}

impl Serialize for Prompt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 5)?;
        state.serialize_field("file", &self.file_path.file_name().unwrap().to_str())?;
        state.serialize_field("split", &self.split.to_string())?;
        state.serialize_field("random", &self.random)?;
        state.end()
    }
}
