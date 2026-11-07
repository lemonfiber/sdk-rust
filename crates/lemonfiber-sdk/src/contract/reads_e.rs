/// `GET /api/explain`, answered with `glossary` or `word`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Explain {
    /// The `word` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word: Option<String>,
}

/// What `GET /api/explain` answers with, by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
#[non_exhaustive]
pub enum ExplainAnswer {
    /// The `glossary` kind.
    #[serde(rename = "glossary")]
    Glossary(Box<super::Vocabulary>),
    /// The `word` kind.
    #[serde(rename = "word")]
    Word(Box<super::Term>),
}

impl crate::Answer for ExplainAnswer {
    const TAGGED: bool = true;
}

impl crate::Read for Explain {
    type Answer = ExplainAnswer;
    const PATH: &str = "/api/explain";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["glossary", "word"];
}
