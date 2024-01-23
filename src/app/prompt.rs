use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Maintains the current prompt the user is typing and counts the button presses.
pub struct Prompt {
    pub prompt: Vec<char>,
    pub typed: Vec<char>,
    pub count_correct: usize, // the characters do not need to be submitted for them to count
    pub count_correct_remove: usize,
    pub count_fault: usize,
    pub count_fault_remove: usize,
}

impl Prompt {
    /// Create a new prompt
    pub fn new(prompt: Vec<char>) -> Prompt {
        Prompt {
            prompt: prompt,
            typed: Vec::new(),
            count_correct: 0,
            count_correct_remove: 0,
            count_fault: 0,
            count_fault_remove: 0,
        }
    }

    /// Finish a prompt.
    ///
    /// Any typed characters that are missing compared to the prompt are counted as mistakes.
    pub fn finish(&mut self) {
        self.count_fault += self.count_missing();
        self.prompt = Vec::new();
        self.typed = Vec::new();
    }

    /// Change the current prompt.
    ///
    /// This function should only be used after [Prompt::finish()] is called.
    pub fn set(&mut self, prompt: Vec<char>) {
        self.prompt = prompt;
    }

    /// Count the missing characters of a prompt
    pub fn count_missing(&self) -> usize {
        if self.prompt.len() > self.typed.len() {
            self.prompt.len() - self.typed.len()
        } else {
            0
        }
    }

    /// Returns the ratio of the correctly typed characters compared to the total amount of typed characters.
    pub fn ratio(&self) -> f64 {
        match self.count_correct + self.count_fault {
            0 => 1 as f64,
            total => self.count_correct as f64 / total as f64,
        }
    }

    /// Push a typed character, check if the character was correct, and update the counters accordingly.
    pub fn type_char(&mut self, c: char) {
        let index = self.typed.len();
        self.typed.push(c);

        if index < self.prompt.len() && c == self.prompt[index] {
            self.count_correct += 1;
        } else {
            self.count_fault += 1;
        }
    }

    /// Remove a character from the prompt and update the counter
    pub fn remove_char(&mut self) {
        if let Some(c) = self.typed.pop() {
            if self.typed.len() >= self.prompt.len() || c != self.prompt[self.typed.len()] {
                self.count_fault_remove += 1;
            } else {
                self.count_correct_remove += 1;
            }
        }
    }
}

/// Serialize a prompt, only the counters are relevant.
impl Serialize for Prompt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Prompt", 4)?;
        state.serialize_field("count_correct", &self.count_correct)?;
        state.serialize_field("count_correct_remove", &self.count_correct_remove)?;
        state.serialize_field("count_fault", &self.count_fault)?;
        state.serialize_field("count_fault_remove", &self.count_fault_remove)?;
        state.end()
    }
}
