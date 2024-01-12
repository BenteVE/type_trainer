use clap::{builder::PossibleValue, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Split {
    Words,
    Lines,
    Text,
}

impl Split {
    pub fn into_prompts(&self, content: String) -> Vec<String> {
        let prompts: Vec<String> = match self {
            Split::Words => content
                .split([' ', '\n'])
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect(),
            Split::Lines => content
                .split(['\n'])
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect(),
            Split::Text => vec![content],
        };

        prompts
    }
}

// Can also be derived with feature flag `derive`
impl ValueEnum for Split {
    fn value_variants<'a>() -> &'a [Self] {
        &[Split::Words, Split::Lines, Split::Text]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Split::Words => PossibleValue::new("words").help("Split the text at each space"),
            Split::Lines => PossibleValue::new("lines").help("Split the text at each line break"),
            Split::Text => PossibleValue::new("text").help("Don't split the text"),
        })
    }
}

impl std::fmt::Display for Split {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Split {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
