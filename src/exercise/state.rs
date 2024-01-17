use std::fmt;

#[derive(PartialEq, Eq)]
pub enum State {
    Waiting,
    Running,
    Pausing,
    Finished,
    Quitting,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Waiting => write!(f, "Waiting"),
            State::Running => write!(f, "Running"),
            State::Pausing => write!(f, "Pausing"),
            State::Finished => write!(f, "Finished"),
            State::Quitting => write!(f, "Quitting"),
        }
    }
}

impl State {
    pub fn button(&self) -> &str {
        match self {
            State::Waiting => "Restart: 'Ctrl+R'",
            State::Running => "Start:   Type",
            State::Pausing => "Pause:   'Ctrl+P'",
            State::Finished => "Stop:    'Ctrl+C'",
            State::Quitting => "Quit:    'Ctrl+C'",
        }
    }
}
