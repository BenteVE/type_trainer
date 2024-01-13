use std::time::Duration;

use serde::ser::{Serialize, SerializeStruct, Serializer};

// The settings are created before the exercise starts and
// cannot be changed during the exercise
pub struct Settings {
    pub duration: Option<Duration>,
    pub blind: bool,     // hide the text when the user is typing
    pub backspace: bool, // allow the use of the backspace key
}
impl Settings {
    pub fn build(duration: Option<Duration>, blind: bool, backspace: bool) -> Settings {
        Settings {
            duration,
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
        let mut state = serializer.serialize_struct("Settings", 5)?;
        state.serialize_field("blind", &self.blind)?;
        state.serialize_field("backspace", &self.backspace)?;
        state.end()
    }
}
