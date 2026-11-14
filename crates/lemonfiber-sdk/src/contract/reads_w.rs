/// `GET /api/watching`, answered with `part-way`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct WatchingRead {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `most` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most: Option<String>,
}

impl crate::Answer for super::PartWayReport {
    const TAGGED: bool = false;
}

impl crate::Read for WatchingRead {
    type Answer = super::PartWayReport;
    const PATH: &str = "/api/watching";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["part-way"];
}

/// `GET /api/watcher`, answered with `watcher`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct WatcherRead {}

impl crate::Answer for super::WatcherReport {
    const TAGGED: bool = false;
}

impl crate::Read for WatcherRead {
    type Answer = super::WatcherReport;
    const PATH: &str = "/api/watcher";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["watcher"];
}

/// `GET /api/wiring`, answered with `wiring`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct WiringRead {}

impl crate::Answer for super::WiringReport {
    const TAGGED: bool = false;
}

impl crate::Read for WiringRead {
    type Answer = super::WiringReport;
    const PATH: &str = "/api/wiring";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["wiring"];
}
