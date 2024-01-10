use std::time::Instant;

use chrono::{DateTime, Local};

use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Stats {
    pub time: DateTime<Local>,
    pub start: Option<Instant>,
    pub end: Option<Instant>,
    pub count_prompts: usize,
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
            count_prompts: 0,
            count_correct: 0,
            count_fault: 0,
        }
    }

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
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 4)?;
        state.serialize_field("date", &self.time.to_rfc2822())?;
        state.serialize_field("duration", &self.duration())?;
        state.serialize_field("count_prompts", &self.count_prompts)?;
        state.serialize_field("count_correct", &self.count_correct)?;
        state.serialize_field("count_fault", &self.count_fault)?;
        state.end()
    }
}
