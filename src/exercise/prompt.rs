use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, LineGauge, Padding, Paragraph, Wrap},
};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Prompt {
    pub prompt: Vec<char>,
    pub typed: Vec<char>,
    pub count_backspace: usize,
    pub count_correct: usize, // the characters do not need to be submitted for them to count
    pub count_fault: usize,
}

impl Prompt {
    pub fn new(prompt: Vec<char>) -> Prompt {
        Prompt {
            prompt: prompt,
            typed: Vec::new(),
            count_backspace: 0,
            count_correct: 0,
            count_fault: 0,
        }
    }

    pub fn finish(&mut self) {
        self.count_fault += self.count_missing();
        self.prompt = Vec::new();
        self.typed = Vec::new();
    }

    pub fn set(&mut self, prompt: Vec<char>) {
        self.prompt = prompt;
    }

    pub fn count_missing(&self) -> usize {
        if self.prompt.len() > self.typed.len() {
            self.prompt.len() - self.typed.len()
        } else {
            0
        }
    }

    pub fn ratio(&self) -> f64 {
        match self.count_correct + self.count_fault {
            0 => 1 as f64,
            total => self.count_correct as f64 / total as f64,
        }
    }

    /// Return if the character was correct
    pub fn type_char(&mut self, c: char) {
        let index = self.typed.len();
        self.typed.push(c);

        if index < self.prompt.len() && c == self.prompt[index] {
            self.count_correct += 1;
        } else {
            self.count_fault += 1;
        }
    }

    pub fn remove_char(&mut self) {
        if self.typed.len() > 0 {
            self.typed.pop();
            self.count_backspace += 1;
        }
    }

    /// Used how many prompts are left
    pub fn build_ratio_bar(&self) -> LineGauge {
        LineGauge::default()
            .block(Block::default().borders(Borders::ALL).title("Ratio"))
            .gauge_style(Style::default().fg(Color::LightGreen).bg(Color::LightRed))
            .ratio(self.ratio())
            .line_set(symbols::line::THICK)
    }

    /// TODO ADD EXTRA RED SPACES FOR EACH CHAR THAT PROMPT IS LONGER THAN TYPED
    pub fn build_prompt_area(&self) -> Paragraph {
        // change the colors of the paragraph
        let mut prompt_styled: Vec<Span> = Vec::new();

        for i in 0..self.prompt.len() {
            if i < self.typed.len() {
                match self.prompt[i] == self.typed[i] {
                    true => {
                        prompt_styled.push(Span::from(self.prompt[i].to_string()).bg(Color::Green))
                    }
                    false => {
                        prompt_styled.push(Span::from(self.prompt[i].to_string()).bg(Color::Red))
                    }
                };
            } else {
                prompt_styled.push(Span::from(self.prompt[i].to_string()));
            }
        }
        Paragraph::new(Line::from(prompt_styled))
            .wrap(Wrap { trim: false })
            .yellow()
    }

    /// The typing area
    pub fn build_type_area(&self) -> Paragraph {
        let mut typed_styled: Vec<Span> = Vec::new();
        typed_styled.push(Span::from(self.typed.iter().collect::<String>()));

        // Add a cursor to the typed text
        typed_styled.push(Span::styled(
            symbols::block::FULL,
            Style::default().add_modifier(Modifier::SLOW_BLINK),
        ));

        Paragraph::new(Line::from(typed_styled))
            .block(
                Block::default()
                    .title("Typed:")
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(1)),
            )
            .wrap(Wrap { trim: false })
    }
}

impl Serialize for Prompt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 5)?;
        state.serialize_field("count_backspace", &self.count_backspace)?;
        state.serialize_field("count_correct", &self.count_correct)?;
        state.serialize_field("count_fault", &self.count_fault)?;
        state.end()
    }
}
