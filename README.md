# Rust Type Trainer

A command-line interface (CLI) type trainer written in the Rust programming language.
This project is designed to let users enhance their typing skills by turning any personal UTF-8 encoded files into a typing exercise.

![options](doc/vhs/type-trainer.gif?raw=true)

## Features

- **Terminal-based Interface:** By running the type trainer from your own familiar terminal, you get a straightforward and realistic typing experience.

- **Performance Metrics:** During the exercise, the Text-Based User Interface (TUI) displays your words per minute (WPM) and accuracy ratio. Other statistics are stored in a .json file when the exercise is complete.

- **Customizable Exercises:**
In addition to creating your own exercises from your local files, you can also use simple command-line arguments to tailor the behaviour of the type trainer.

The complete list of options is available with the '--help' command:
![Help Command](doc/help.png?raw=true)

## Getting Started

To get started with this the type trainer, you can follow the simple steps below to clone and build the repository:

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/BenteVE/type_trainer.git
   cd type_trainer
   ```

2. **Build the Project:**

   ```bash
   cargo build --release
   ```

3. **Run the Type Trainer:**

   ```bash
   ./target/release/type_trainer <Path to training file>
   ```
