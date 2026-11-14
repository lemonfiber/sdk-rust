/// `GET /api/version`, answered with `version`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct VersionRead {}

impl crate::Answer for super::VersionReport {
    const TAGGED: bool = false;
}

impl crate::Read for VersionRead {
    type Answer = super::VersionReport;
    const PATH: &str = "/api/version";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["version"];
}
