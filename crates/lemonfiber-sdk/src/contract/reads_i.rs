/// `GET /api/in-flight`, answered with `in-flight`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct InFlightRead {
    /// The `forms` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forms: Vec<String>,
    /// The `services` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<String>,
}

impl crate::Answer for super::InFlightReport {
    const TAGGED: bool = false;
}

impl crate::Read for InFlightRead {
    type Answer = super::InFlightReport;
    const PATH: &str = "/api/in-flight";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["in-flight"];
}
