/// `GET /api/status`, answered with `status`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct StatusRead {}

impl crate::Answer for super::StatusReport {
    const TAGGED: bool = false;
}

impl crate::Read for StatusRead {
    type Answer = super::StatusReport;
    const PATH: &str = "/api/status";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["status"];
}

/// `GET /api/services`, answered with `status`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct ServicesRead {
    /// The `form` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form: Vec<String>,
}

impl crate::Read for ServicesRead {
    type Answer = super::StatusReport;
    const PATH: &str = "/api/services";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["status"];
}

/// `GET /api/storage`, answered with `doctor`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct StorageRead {}

impl crate::Read for StorageRead {
    type Answer = super::DoctorReport;
    const PATH: &str = "/api/storage";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["doctor"];
}

/// `GET /api/stack`, answered with `stack`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct StackRead {}

impl crate::Answer for super::StackReport {
    const TAGGED: bool = false;
}

impl crate::Read for StackRead {
    type Answer = super::StackReport;
    const PATH: &str = "/api/stack";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["stack"];
}

/// `GET /api/stuck`, answered with `stuck`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct StuckRead {}

impl crate::Answer for super::StuckReport {
    const TAGGED: bool = false;
}

impl crate::Read for StuckRead {
    type Answer = super::StuckReport;
    const PATH: &str = "/api/stuck";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["stuck"];
}

/// `GET /api/stored`, answered with `stored`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct StoredRead {}

impl crate::Answer for super::Stored {
    const TAGGED: bool = false;
}

impl crate::Read for StoredRead {
    type Answer = super::Stored;
    const PATH: &str = "/api/stored";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["stored"];
}

/// `GET /api/space`, answered with `space`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct SpaceRead {}

impl crate::Answer for super::Reckoning {
    const TAGGED: bool = false;
}

impl crate::Read for SpaceRead {
    type Answer = super::Reckoning;
    const PATH: &str = "/api/space";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["space"];
}

/// `GET /api/settings`, answered with `settings`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct SettingsRead {}

impl crate::Answer for super::SettingsReport {
    const TAGGED: bool = false;
}

impl crate::Read for SettingsRead {
    type Answer = super::SettingsReport;
    const PATH: &str = "/api/settings";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["settings"];
}
