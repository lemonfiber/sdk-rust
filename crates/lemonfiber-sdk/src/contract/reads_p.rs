/// `GET /api/playing`, answered with `playing`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct PlayingRead {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
}

impl crate::Answer for super::PlayingReport {
    const TAGGED: bool = false;
}

impl crate::Read for PlayingRead {
    type Answer = super::PlayingReport;
    const PATH: &str = "/api/playing";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["playing"];
}

/// `GET /api/provenance`, answered with `provenance`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct ProvenanceRead {}

impl crate::Answer for super::ProvenanceReport {
    const TAGGED: bool = false;
}

impl crate::Read for ProvenanceRead {
    type Answer = super::ProvenanceReport;
    const PATH: &str = "/api/provenance";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["provenance"];
}

/// `GET /api/plugins`, answered with `plugins`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct PluginsRead {}

impl crate::Answer for super::PluginInstalls {
    const TAGGED: bool = false;
}

impl crate::Read for PluginsRead {
    type Answer = super::PluginInstalls;
    const PATH: &str = "/api/plugins";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["plugins"];
}
