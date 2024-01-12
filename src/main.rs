use anyhow::{Ok, Result};
use ratatui::{backend::CrosstermBackend, Terminal};
use type_trainer::{
    exercise::exercise::Exercise,
    parser::Parser,
    ui::{
        event::{Event, EventHandler},
        tui::Tui,
    },
};

fn main() -> Result<()> {
    let matches = Parser::new();
    let settings = Parser::parse(matches)?;

    let mut exercise = Exercise::build(settings);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Do a countdown

    exercise.start();

    // Start the main loop.
    while !exercise.should_quit() {
        // Render the user interface.
        tui.draw(&mut exercise)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => exercise.update(key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Show the stats on an end screen

    // Exit the user interface.
    tui.exit()?;

    println!("Print the serialized stats");
    println!("{}", serde_json::to_string(&exercise.settings).unwrap());
    println!("{}", serde_json::to_string(&exercise.stats).unwrap());

    Ok(())
}
