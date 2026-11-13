/// `GET /api/trace`, answered with `trace`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct TraceRead {
    /// The `term` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// The `season` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
}

impl crate::Answer for super::TraceReport {
    const TAGGED: bool = false;
}

impl crate::Read for TraceRead {
    type Answer = super::TraceReport;
    const PATH: &str = "/api/trace";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["trace"];
}
