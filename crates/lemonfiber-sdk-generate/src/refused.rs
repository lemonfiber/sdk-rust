use std::fmt;

/// Why a command stopped, in a sentence naming what to change. Nothing was written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused(String);

impl Refused {
    /// A refusal saying `why`.
    #[must_use]
    pub fn new(why: impl Into<String>) -> Self {
        Self(why.into())
    }

    /// A refusal listing every problem found under one heading.
    #[must_use]
    pub fn listing(heading: &str, problems: &[String]) -> Self {
        Self(format!("{heading}:\n  {}", problems.join("\n  ")))
    }

    /// The sentence.
    #[must_use]
    pub fn said(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Refused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Refused {}
