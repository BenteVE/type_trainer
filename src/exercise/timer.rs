use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::time::{Duration, Instant};

pub struct Timer {
    pub start: Option<Instant>,
    pub end: Option<Instant>,
    pub duration: Option<Duration>,
}

impl Timer {
    pub fn new(duration: Option<Duration>) -> Timer {
        Timer {
            start: Option::None,
            end: Option::None,
            duration,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.end = Some(Instant::now());
    }

    pub fn get_total(&self) -> Duration {
        self.end.unwrap().duration_since(self.start.unwrap())
    }

    pub fn timer_expired(&self) -> bool {
        if let Some(duration) = self.duration {
            self.start
                .expect("The timer should be started at the start of the exercise")
                .elapsed()
                >= duration
        } else {
            false
        }
    }
}

impl Serialize for Timer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Timer", 1)?;
        state.serialize_field("total_duration", &self.get_total())?;
        state.end()
    }
}
