use rand::{seq::SliceRandom, thread_rng};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::path::PathBuf;

/// Contains the collection of prompts that were read from the specified file, together with the relevant settings.
pub struct Content {
    pub file_path: PathBuf,
    pub words: bool,
    pub random: bool,

    content: Vec<String>,
    prompt_index: usize,
}

impl Content {
    /// Create the content struct
    pub fn build(file_path: PathBuf, content: Vec<String>, random: bool, words: bool) -> Content {
        let mut content = Content {
            file_path,
            content,
            prompt_index: 0,
            words,
            random,
        };

        if content.random {
            content.shuffle_prompts();
        }

        content
    }

    /// Shuffle the prompts. This function should only be called when the random-flag is set.
    fn shuffle_prompts(&mut self) {
        self.content.shuffle(&mut thread_rng())
    }

    /// Prepare the struct for restarting the exercise.
    pub fn reset(&mut self) {
        self.prompt_index = 0;
        if self.random {
            self.shuffle_prompts();
        }
    }

    /// Advance the pointer to the next prompt
    pub fn next_prompt(&mut self) {
        self.prompt_index += 1;
    }

    /// Returns the current prompt as a vector of characters.
    ///
    /// This format makes it easier for the [Prompt] to compare the individual typed characters with the corresponding characters from the prompt.
    ///
    /// [Prompt]: crate::app::prompt::Prompt
    pub fn get_prompt(&self) -> Option<Vec<char>> {
        if let Some(s) = self.content.get(self.prompt_index) {
            Some(s.chars().collect())
        } else {
            Option::None
        }
    }

    /// Returns all the following prompts in the form of a vector of Strings
    ///
    /// This format is chosen because the characters in these Strings do not need to be individually styled or compared.
    pub fn get_next_prompts(&self) -> Vec<String> {
        match self.content.len() >= self.prompt_index + 1 {
            true => self.content[self.prompt_index + 1..].to_vec(),
            false => Vec::new(),
        }
    }

    /// Returns the ratio of the finished prompts compared to the total amount of prompts.
    pub fn ratio(&self) -> f64 {
        self.prompt_index as f64 / self.content.len() as f64
    }

    /// Used to split the original text from the specified file into prompts.
    ///
    /// The text is split after every line, or every word, if the flag is set to true.
    pub fn into_prompts(content: String, words: bool) -> Vec<String> {
        match words {
            false => content.lines().map(|s| s.to_string()).collect(),
            true => content
                .split_whitespace()
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        }
    }
}

/// Serialize the content, he complete list of prompts is not relevant to save
impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Content", 3)?;
        state.serialize_field("file_path", &self.file_path.file_name().unwrap().to_str())?;
        state.serialize_field("words", &self.words)?;
        state.serialize_field("random", &self.random)?;
        state.end()
    }
}
