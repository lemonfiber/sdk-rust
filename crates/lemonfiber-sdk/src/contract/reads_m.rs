/// `GET /api/migration`, answered with `migration`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Migration {}

impl crate::Answer for super::MigrationReport {
    const TAGGED: bool = false;
}

impl crate::Read for Migration {
    type Answer = super::MigrationReport;
    const PATH: &str = "/api/migration";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["migration"];
}
