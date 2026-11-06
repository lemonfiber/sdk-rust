/// `GET /api/checks`, answered with `doctor`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Checks {
    /// The `only` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<String>,
}

impl crate::Answer for super::DoctorReport {
    const TAGGED: bool = false;
}

impl crate::Read for Checks {
    type Answer = super::DoctorReport;
    const PATH: &'static str = "/api/checks";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["doctor"];
}

/// `GET /api/config`, answered with `config`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Config {
    /// The `key` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl crate::Answer for super::ConfigReport {
    const TAGGED: bool = false;
}

impl crate::Read for Config {
    type Answer = super::ConfigReport;
    const PATH: &'static str = "/api/config";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["config"];
}

/// `GET /api/catalogue`, answered with `catalogue`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Catalogue {}

impl crate::Answer for super::CatalogueReport {
    const TAGGED: bool = false;
}

impl crate::Read for Catalogue {
    type Answer = super::CatalogueReport;
    const PATH: &'static str = "/api/catalogue";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["catalogue"];
}

/// `GET /api/clients`, answered with `clients`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Clients {}

impl crate::Answer for super::Guidance {
    const TAGGED: bool = false;
}

impl crate::Read for Clients {
    type Answer = super::Guidance;
    const PATH: &'static str = "/api/clients";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["clients"];
}

/// `GET /api/credentials`, answered with `credentials`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Credentials {}

impl crate::Answer for super::Inventory {
    const TAGGED: bool = false;
}

impl crate::Read for Credentials {
    type Answer = super::Inventory;
    const PATH: &'static str = "/api/credentials";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["credentials"];
}
