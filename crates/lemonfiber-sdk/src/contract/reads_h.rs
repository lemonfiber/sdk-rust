/// `GET /api/held`, answered with `held`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Held {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
    /// The `most` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most: Option<String>,
}

impl crate::Answer for super::HeldReport {
    const TAGGED: bool = false;
}

impl crate::Read for Held {
    type Answer = super::HeldReport;
    const PATH: &str = "/api/held";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["held"];
}

/// `GET /api/held/{id}`, answered with `title`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct HeldId {
    /// The `id` segment of the path.
    pub id: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
}

impl crate::Answer for super::TitleReport {
    const TAGGED: bool = false;
}

impl crate::Read for HeldId {
    type Answer = super::TitleReport;
    const PATH: &str = "/api/held/{id}";
    const SEGMENTS: &[&str] = &["id"];
    const KINDS: &[&str] = &["title"];
}

/// `GET /api/hosting`, answered with `hosting`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Hosting {}

impl crate::Answer for super::HostingReport {
    const TAGGED: bool = false;
}

impl crate::Read for Hosting {
    type Answer = super::HostingReport;
    const PATH: &str = "/api/hosting";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["hosting"];
}

/// `GET /api/history`, answered with `history`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct History {}

impl crate::Answer for super::HistoryReport {
    const TAGGED: bool = false;
}

impl crate::Read for History {
    type Answer = super::HistoryReport;
    const PATH: &str = "/api/history";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["history"];
}

/// `GET /api/held/{id}/poster`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct HeldIdPoster {
    /// The `id` segment of the path.
    pub id: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
}

impl crate::FileRead for HeldIdPoster {
    const PATH: &str = "/api/held/{id}/poster";
    const SEGMENTS: &[&str] = &["id"];
}

/// `GET /api/held/{id}/backdrop`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct HeldIdBackdrop {
    /// The `id` segment of the path.
    pub id: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
}

impl crate::FileRead for HeldIdBackdrop {
    const PATH: &str = "/api/held/{id}/backdrop";
    const SEGMENTS: &[&str] = &["id"];
}
