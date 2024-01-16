use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::time::{Duration, Instant};

pub struct Timer {
    start: Option<Instant>,
    saved: Duration,
    duration: Option<Duration>,
}

impl Timer {
    pub fn new(duration: Option<Duration>) -> Timer {
        Timer {
            saved: Duration::from_secs(0),
            start: Option::None,
            duration,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }
    
    pub fn stop(&mut self) {
        self.saved = self.get_time();
        self.start = Option::None;
    }

    pub fn reset(&mut self) {
        self.start = Option::None;
        self.saved = Duration::from_secs(0);
    }

    pub fn get_time(&self) -> Duration {
        let elapsed = if let Some(start) = self.start {
            Instant::now().duration_since(start)
        } else {
            Duration::new(0, 0)
        };
        self.saved + elapsed
    }

    pub fn timer_expired(&self) -> bool {
        if let Some(duration) = self.duration {
            self.get_time() >= duration
        } else {
            false
        }
    }

    pub fn ratio(&self) -> f64 {
        if let Some(duration) = self.duration {
            return self.get_time().as_secs_f64() / duration.as_secs_f64();
        } else {
            return self.get_time().as_secs_f64() % 60 as f64 / 60 as f64;
        }
    }

    /// Get the current time of the timer in String format = XX:XX
    pub fn get_time_label(&self) -> String {
        let mut seconds = self.get_time().as_secs();
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
        state.serialize_field("total_duration", &self.get_time().as_secs())?;
        state.end()
    }
}
