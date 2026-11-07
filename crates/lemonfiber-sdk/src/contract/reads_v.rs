/// `GET /api/version`, answered with `version`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Version {}

impl crate::Answer for super::VersionReport {
    const TAGGED: bool = false;
}

impl crate::Read for Version {
    type Answer = super::VersionReport;
    const PATH: &str = "/api/version";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["version"];
}
