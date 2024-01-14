use ratatui::{
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, LineGauge},
};
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

    // if there is some duration: give the ratio to that duration
    // otherwise reset every minute
    pub fn build_widget(&self) -> LineGauge {
        LineGauge::default()
            .block(Block::default().borders(Borders::ALL).title("Timer"))
            .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
            .ratio(self.ratio())
            .line_set(Self::build_timer_set())
            .label(self.label())
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

    pub fn label(&self) -> String {
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

    fn build_timer_set() -> symbols::line::Set {
        symbols::line::Set {
            horizontal: symbols::DOT,
            vertical: symbols::DOT,
            top_right: symbols::DOT,
            top_left: symbols::DOT,
            bottom_right: symbols::DOT,
            bottom_left: symbols::DOT,
            vertical_left: symbols::DOT,
            vertical_right: symbols::DOT,
            horizontal_down: symbols::DOT,
            horizontal_up: symbols::DOT,
            cross: symbols::DOT,
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
