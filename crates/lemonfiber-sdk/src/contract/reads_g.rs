/// `GET /api/guide`, answered with `guide`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct GuideRead {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
}

impl crate::Answer for super::GuideReport {
    const TAGGED: bool = false;
}

impl crate::Read for GuideRead {
    type Answer = super::GuideReport;
    const PATH: &str = "/api/guide";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["guide"];
}
