/// `GET /api/forms`, answered with `forms` or `preview`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Forms {
    /// The `form` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form: Vec<String>,
}

/// What `GET /api/forms` answers with, by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum FormsAnswer {
    /// The `forms` kind.
    #[serde(rename = "forms")]
    Forms(Box<super::FormsReport>),
    /// The `preview` kind.
    #[serde(rename = "preview")]
    Preview(Box<super::Plan>),
}

impl crate::Answer for FormsAnswer {
    const TAGGED: bool = true;
}

impl crate::Read for Forms {
    type Answer = FormsAnswer;
    const PATH: &'static str = "/api/forms";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["forms", "preview"];
}

/// `GET /api/front-door`, answered with `front-door`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct FrontDoor {}

impl crate::Answer for super::FrontDoorReport {
    const TAGGED: bool = false;
}

impl crate::Read for FrontDoor {
    type Answer = super::FrontDoorReport;
    const PATH: &'static str = "/api/front-door";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["front-door"];
}
