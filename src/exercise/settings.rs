use serde::ser::{Serialize, SerializeStruct, Serializer};

// The settings are created before the exercise starts and
// cannot be changed during the exercise
pub struct Settings {
    pub highlight: bool, // Highlight the correct letters in green and the mistakes in red
    pub blind: bool,     // Hide the text when the user is typing
    pub backspace: bool, // Allow the use of the backspace key
}
impl Settings {
    pub fn build(backspace: bool, highlight: bool, blind: bool) -> Settings {
        Settings {
            highlight,
            blind,
            backspace,
        }
    }
}

impl Serialize for Settings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 3)?;
        state.serialize_field("blind", &self.blind)?;
        state.serialize_field("backspace", &self.backspace)?;
        state.serialize_field("highlight", &self.highlight)?;
        state.end()
    }
}
