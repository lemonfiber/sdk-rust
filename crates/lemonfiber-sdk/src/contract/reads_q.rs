/// `GET /api/queue`, answered with `queue`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Queue {}

impl crate::Answer for super::QueueHealth {
    const TAGGED: bool = false;
}

impl crate::Read for Queue {
    type Answer = super::QueueHealth;
    const PATH: &str = "/api/queue";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["queue"];
}

/// `GET /api/quality`, answered with `quality`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Quality {}

impl crate::Answer for super::QualityReport {
    const TAGGED: bool = false;
}

impl crate::Read for Quality {
    type Answer = super::QualityReport;
    const PATH: &str = "/api/quality";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["quality"];
}
