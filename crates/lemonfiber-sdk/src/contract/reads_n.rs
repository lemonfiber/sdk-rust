/// `GET /api/notify`, answered with `notify`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Notify {}

impl crate::Answer for super::NotifyReport {
    const TAGGED: bool = false;
}

impl crate::Read for Notify {
    type Answer = super::NotifyReport;
    const PATH: &str = "/api/notify";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["notify"];
}

/// `GET /api/notify/mine`, answered with `notify-mine`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct NotifyMine {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
}

impl crate::Answer for super::NotifyMineReport {
    const TAGGED: bool = false;
}

impl crate::Read for NotifyMine {
    type Answer = super::NotifyMineReport;
    const PATH: &str = "/api/notify/mine";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["notify-mine"];
}

/// `GET /api/news`, answered with `news-items`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct News {}

impl crate::Answer for super::News {
    const TAGGED: bool = false;
}

impl crate::Read for News {
    type Answer = super::News;
    const PATH: &str = "/api/news";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["news-items"];
}
