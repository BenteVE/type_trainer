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
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 4)?;
        state.serialize_field("time", &self.time.to_rfc2822())?;
        state.serialize_field("count_prompts", &self.count_prompts)?;
        state.serialize_field("count_correct", &self.count_correct)?;
        state.serialize_field("count_fault", &self.count_fault)?;
        state.end()
    }
}
