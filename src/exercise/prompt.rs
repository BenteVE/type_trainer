use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, LineGauge, Padding, Paragraph, Wrap},
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

    pub fn build_prompt_area(&self, next_prompts: Vec<String>) -> Paragraph {
        // change the colors of the paragraph
        let mut prompt_styled: Vec<Span> = Vec::new();

        for i in 0..usize::min(self.typed.len(), self.prompt.len()) {
            match self.prompt[i] == self.typed[i] {
                true => prompt_styled.push(
                    Span::from(self.prompt[i].to_string())
                        .bg(Color::Green)
                        .yellow(),
                ),
                false => prompt_styled.push(
                    Span::from(self.prompt[i].to_string())
                        .bg(Color::Red)
                        .yellow(),
                ),
            };
        }
        // if typed is longer than prompt, we add red spaces for each unnecessary letter
        if self.typed.len() > self.prompt.len() {
            prompt_styled.push(
                Span::from(
                    std::iter::repeat(" ")
                        .take(self.typed.len() - self.prompt.len())
                        .collect::<String>(),
                )
                .bg(Color::Red)
                .yellow(),
            )
        }
        // The rest of the line should not be styled
        else if self.typed.len() < self.prompt.len() {
            prompt_styled.push(
                Span::from(self.prompt[self.typed.len()..].iter().collect::<String>()).yellow(),
            );
        }

        let mut text = Text::from(Line::from(prompt_styled));
        text.extend(next_prompts);

        Paragraph::new(text).wrap(Wrap { trim: false }).block(
            Block::default()
                .title("Prompt")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().yellow())
                .padding(Padding::uniform(1)),
        )
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

        /// Calculate a scroll determined by the number of lines it is stretched over
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
