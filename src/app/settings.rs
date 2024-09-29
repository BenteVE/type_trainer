use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Store the settings of the exercise based on the arguments that were given.
///
/// These settings should not be changed during the execution of the application.
pub struct Settings {
    pub highlight: bool, // Highlight the correct letters in green and the mistakes in red
    pub blind: bool,     // Hide the text when the user is typing
    pub backspace: bool, // Allow the use of the backspace key
    pub auto: bool,      // Automatically progress to the next line without pressing enter
    pub correct: bool,   // Automatically progress to the next line without pressing enter
    pub repeat: bool,    // Repeat the prompt when a mistake was made while typing it
    pub terminate: Option<usize>, // Terminate the exercise after a given amount of mistakes
}
impl Settings {
    pub fn build(
        backspace: bool,
        highlight: bool,
        blind: bool,
        auto: bool,
        correct: bool,
        repeat: bool,
        terminate: Option<usize>,
    ) -> Settings {
        Settings {
            highlight,
            blind,
            backspace,
            auto,
            correct,
            repeat,
            terminate,
        }
    }
}

/// Serialize the settings
impl Serialize for Settings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 3)?;
        state.serialize_field("blind", &self.blind)?;
        state.serialize_field("backspace", &self.backspace)?;
        state.serialize_field("highlight", &self.highlight)?;
        state.serialize_field("auto", &self.auto)?;
        state.serialize_field("correct", &self.correct)?;
        state.serialize_field("repeat", &self.repeat)?;
        state.serialize_field("terminate", &self.terminate)?;
        state.end()
    }
}
