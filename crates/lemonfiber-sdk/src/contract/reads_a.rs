/// `GET /api/askable`, answered with `askable`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Askable {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `term` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// The `after` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The `kind` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The `unit` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl crate::Answer for super::AskableReport {
    const TAGGED: bool = false;
}

impl crate::Read for Askable {
    type Answer = super::AskableReport;
    const PATH: &str = "/api/askable";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["askable"];
}

/// `GET /api/askable/kinds`, answered with `askable-kinds`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct AskableKinds {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
}

impl crate::Answer for super::AskableKindsReport {
    const TAGGED: bool = false;
}

impl crate::Read for AskableKinds {
    type Answer = super::AskableKindsReport;
    const PATH: &str = "/api/askable/kinds";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["askable-kinds"];
}

/// `GET /api/alerts`, answered with `alerts`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Alerts {}

impl crate::Answer for super::AlertReport {
    const TAGGED: bool = false;
}

impl crate::Read for Alerts {
    type Answer = super::AlertReport;
    const PATH: &str = "/api/alerts";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["alerts"];
}

/// `GET /api/askable/{id}/poster`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct AskableIdPoster {
    /// The `id` segment of the path.
    pub id: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `kind` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl crate::FileRead for AskableIdPoster {
    const PATH: &str = "/api/askable/{id}/poster";
    const SEGMENTS: &[&str] = &["id"];
}
