/// `GET /api/queue`, answered with `queue`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct QueueRead {}

impl crate::Answer for super::QueueHealth {
    const TAGGED: bool = false;
}

impl crate::Read for QueueRead {
    type Answer = super::QueueHealth;
    const PATH: &str = "/api/queue";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["queue"];
}

/// `GET /api/quality`, answered with `quality`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct QualityRead {}

impl crate::Answer for super::QualityReport {
    const TAGGED: bool = false;
}

impl crate::Read for QualityRead {
    type Answer = super::QualityReport;
    const PATH: &str = "/api/quality";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["quality"];
}
