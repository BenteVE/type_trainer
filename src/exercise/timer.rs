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

    pub fn ratio(&self) -> f64 {
        let start = self
            .start
            .expect("The timer should be started at the start of the exercise");
        if let Some(duration) = self.duration {
            return start.elapsed().as_secs_f64() / duration.as_secs_f64();
        } else {
            return start.elapsed().as_secs_f64() % 60 as f64 / 60 as f64;
        }
    }

    /// Get the current time of the timer in String format = XX:XX
    pub fn get_time(&self) -> String {
        let mut seconds = self
            .start
            .expect("The timer should be started at the start of the exercise")
            .elapsed()
            .as_secs();
        let minutes = seconds / 60;
        seconds %= 60;

        format!("{}:{}", Self::pad_label(minutes), Self::pad_label(seconds))
    }

    fn pad_label(num: u64) -> String {
        match num < 10 {
            true => format!("0{}", num),
            false => format!("{}", num),
        }
    }
}

impl Serialize for Timer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Timer", 1)?;
        state.serialize_field("total_duration", &self.get_total().as_secs())?;
        state.end()
    }
}
