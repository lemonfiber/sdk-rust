/// `GET /api/watching`, answered with `part-way`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Watching {
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

impl crate::Read for Watching {
    type Answer = super::PartWayReport;
    const PATH: &'static str = "/api/watching";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["part-way"];
}

/// `GET /api/wiring`, answered with `wiring`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Wiring {}

impl crate::Answer for super::WiringReport {
    const TAGGED: bool = false;
}

impl crate::Read for Wiring {
    type Answer = super::WiringReport;
    const PATH: &'static str = "/api/wiring";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["wiring"];
}
