/// `GET /api/requests`, answered with `household`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Requests {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
}

impl crate::Answer for super::HouseholdReport {
    const TAGGED: bool = false;
}

impl crate::Read for Requests {
    type Answer = super::HouseholdReport;
    const PATH: &str = "/api/requests";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["household"];
}

/// `GET /api/requests/{reference}/poster`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct RequestsReferencePoster {
    /// The `reference` segment of the path.
    pub reference: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
}

impl crate::FileRead for RequestsReferencePoster {
    const PATH: &str = "/api/requests/{reference}/poster";
    const SEGMENTS: &[&str] = &["reference"];
}
