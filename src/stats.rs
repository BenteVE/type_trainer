use std::time::Instant;

use chrono::{DateTime, Local};

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

// Implement serialize for stats
