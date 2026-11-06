/// `GET /api/logs`, answered with `job` or `log`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Logs {
    /// The `form` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form: Vec<String>,
    /// The `service` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<String>,
    /// The `tail` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<String>,
    /// The `follow` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow: Option<String>,
}

/// What `GET /api/logs` answers with, by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum LogsAnswer {
    /// The `job` kind.
    #[serde(rename = "job")]
    Job(Box<super::Started>),
    /// The `log` kind.
    #[serde(rename = "log")]
    Log(Box<super::LogLine>),
}

impl crate::Answer for LogsAnswer {
    const TAGGED: bool = true;
}

impl crate::Read for Logs {
    type Answer = LogsAnswer;
    const PATH: &'static str = "/api/logs";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["job", "log"];
}
