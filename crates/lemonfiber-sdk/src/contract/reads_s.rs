/// `GET /api/status`, answered with `status`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Status {}

impl crate::Answer for super::StatusReport {
    const TAGGED: bool = false;
}

impl crate::Read for Status {
    type Answer = super::StatusReport;
    const PATH: &'static str = "/api/status";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["status"];
}

/// `GET /api/services`, answered with `status`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Services {
    /// The `form` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form: Vec<String>,
}

impl crate::Read for Services {
    type Answer = super::StatusReport;
    const PATH: &'static str = "/api/services";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["status"];
}

/// `GET /api/storage`, answered with `doctor`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Storage {}

impl crate::Read for Storage {
    type Answer = super::DoctorReport;
    const PATH: &'static str = "/api/storage";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["doctor"];
}

/// `GET /api/stuck`, answered with `stuck`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Stuck {}

impl crate::Answer for super::StuckReport {
    const TAGGED: bool = false;
}

impl crate::Read for Stuck {
    type Answer = super::StuckReport;
    const PATH: &'static str = "/api/stuck";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["stuck"];
}

/// `GET /api/stored`, answered with `stored`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Stored {}

impl crate::Answer for super::Stored {
    const TAGGED: bool = false;
}

impl crate::Read for Stored {
    type Answer = super::Stored;
    const PATH: &'static str = "/api/stored";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["stored"];
}

/// `GET /api/space`, answered with `space`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Space {}

impl crate::Answer for super::Reckoning {
    const TAGGED: bool = false;
}

impl crate::Read for Space {
    type Answer = super::Reckoning;
    const PATH: &'static str = "/api/space";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["space"];
}

/// `GET /api/settings`, answered with `settings`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Settings {}

impl crate::Answer for super::SettingsReport {
    const TAGGED: bool = false;
}

impl crate::Read for Settings {
    type Answer = super::SettingsReport;
    const PATH: &'static str = "/api/settings";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["settings"];
}
