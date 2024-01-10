use std::time::Instant;

pub struct Stats {
    start: Option<Instant>,
    count_correct: usize, // the characters do not need to be submitted for them to count
    count_fault: usize,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            start: Option::None,
            count_correct: 0,
            count_fault: 0,
        }
    }

    fn elapsed_time(&self) -> usize {
        self.start.unwrap().elapsed().as_secs() as usize
    }

}

// Implement serialize for stats
