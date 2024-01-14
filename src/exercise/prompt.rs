use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Prompt {
    pub prompt: Vec<char>,
    pub typed: Vec<char>,
    pub count_backspace: usize,
    pub count_correct: usize, // the characters do not need to be submitted for them to count
    pub count_fault: usize,
}

impl Prompt {
    pub fn new(prompt: Vec<char>) -> Prompt {
        Prompt {
            prompt: prompt,
            typed: Vec::new(),
            count_backspace: 0,
            count_correct: 0,
            count_fault: 0,
        }
    }

    pub fn finish(&mut self) {
        self.count_fault += self.count_missing();
        self.prompt = Vec::new();
        self.typed = Vec::new();
    }

    pub fn set(&mut self, prompt: Vec<char>) {
        self.prompt = prompt;
    }

    pub fn count_missing(&self) -> usize {
        if self.prompt.len() > self.typed.len() {
            self.prompt.len() - self.typed.len()
        } else {
            0
        }
    }

    pub fn ratio(&self) -> f64 {
        match self.count_correct + self.count_fault {
            0 => 1 as f64,
            total => self.count_correct as f64 / total as f64,
        }
    }

    /// Return if the character was correct
    pub fn type_char(&mut self, c: char) {
        let index = self.typed.len();
        self.typed.push(c);

        if index < self.prompt.len() && c == self.prompt[index] {
            self.count_correct += 1;
        } else {
            self.count_fault += 1;
        }
    }

    pub fn remove_char(&mut self) {
        if self.typed.len() > 0 {
            self.typed.pop();
            self.count_backspace += 1;
        }
    }
}

impl Serialize for Prompt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 5)?;
        state.serialize_field("count_backspace", &self.count_backspace)?;
        state.serialize_field("count_correct", &self.count_correct)?;
        state.serialize_field("count_fault", &self.count_fault)?;
        state.end()
    }
}
