/// `GET /api/quality`, answered with `quality`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Quality {}

impl crate::Answer for super::QualityReport {
    const TAGGED: bool = false;
}

impl crate::Read for Quality {
    type Answer = super::QualityReport;
    const PATH: &'static str = "/api/quality";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["quality"];
}
