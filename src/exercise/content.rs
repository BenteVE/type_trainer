use ratatui::{
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, LineGauge},
};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::path::PathBuf;

pub struct Content {
    pub file_path: PathBuf,
    pub words: bool,
    pub random: bool,

    content: Vec<String>,
    prompt_index: usize,
}

impl Content {
    pub fn build(file_path: PathBuf, content: Vec<String>, random: bool, words: bool) -> Content {
        Content {
            file_path,
            content,
            prompt_index: 0,
            words,
            random,
        }
    }

    pub fn next_prompt(&mut self) {
        self.prompt_index += 1;
    }

    pub fn get_prompt(&self) -> Option<Vec<char>> {
        if let Some(s) = self.content.get(self.prompt_index) {
            Some(s.chars().collect())
        } else {
            Option::None
        }
    }

    /// Used how many prompts are left
    pub fn build_widget(&self) -> LineGauge {
        LineGauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
            .ratio(self.ratio())
            .line_set(symbols::line::THICK)
    }

    pub fn ratio(&self) -> f64 {
        match self.random {
            true => 1 as f64,
            false => self.prompt_index as f64 / self.content.len() as f64,
        }
    }

    pub fn into_prompts(content: String, words: bool) -> Vec<String> {
        let prompts: Vec<String> = match words {
            true => content
                .split([' ', '\n'])
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect(),
            false => content
                .split(['\n'])
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect(),
        };

        prompts
    }
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Exercise", 2)?;
        state.serialize_field("file_path", &self.file_path.file_name().unwrap().to_str())?;
        state.serialize_field("words", &self.words)?;
        state.serialize_field("random", &self.random)?;
        state.end()
    }
}
