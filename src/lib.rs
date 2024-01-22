/// Contains the definition of the possible arguments for the application
/// and the functions to parse the given arguments into the corresponding exercise.
pub mod parser;

/// Contains the logic of the type trainer application.
pub mod app {
    pub mod content;
    pub mod exercise;
    pub mod prompt;
    pub mod settings;
    pub mod state;
    pub mod timer;
}

/// Contains the logic that handles the UI and the user input events.
pub mod ui {
    pub mod event;
    pub mod tui;
    pub mod render;
}
