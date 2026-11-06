/// `GET /api/outbound`, answered with `outbound`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Outbound {}

impl crate::Answer for super::Leaving {
    const TAGGED: bool = false;
}

impl crate::Read for Outbound {
    type Answer = super::Leaving;
    const PATH: &'static str = "/api/outbound";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["outbound"];
}
