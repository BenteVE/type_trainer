use std::time::Instant;

use chrono::{DateTime, Local};

use serde::ser::{Serialize, SerializeStruct, Serializer};

// Stats are created at the beginning of the exercise
// and are changed during the exercise
pub struct Stats {
    pub time: DateTime<Local>,
    pub start: Option<Instant>,
    pub end: Option<Instant>,
    pub count_enter: usize,
    pub count_backspace: usize,
    pub count_correct: usize, // the characters do not need to be submitted for them to count
    pub count_fault: usize,
    // record backspace presses
    // record a hashmap with each prompt char to a set of chars it was mistaken for
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            time: Local::now(),
            start: Option::None,
            end: Option::None,
            count_enter: 0,
            count_backspace: 0,
            count_correct: 0,
            count_fault: 0,
        }
    }

    // Compare the typed character with the given prompt
    pub fn compare_char(&self) {}

    pub fn duration(&self) -> u64 {
        let duration = self
            .end
            .unwrap()
            .checked_duration_since(self.start.unwrap());
        match duration {
            Some(d) => d.as_secs(),
            None => 0,
        }
    }

    pub fn ratio(&self) -> f64 {
        match self.count_correct + self.count_fault {
            0 => 1 as f64,
            total => self.count_correct as f64 / total as f64,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 4)?;
        state.serialize_field("date", &self.time.to_rfc2822())?;
        state.serialize_field("duration", &self.duration())?;
        state.serialize_field("count_enter", &self.count_enter)?;
        state.serialize_field("count_backspace", &self.count_backspace)?;
        state.serialize_field("count_correct", &self.count_correct)?;
        state.serialize_field("count_fault", &self.count_fault)?;
        state.end()
    }
}
