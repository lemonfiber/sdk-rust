/// `GET /api/boot`, answered with `boot`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct BootRead {}

impl crate::Answer for super::BootReport {
    const TAGGED: bool = false;
}

impl crate::Read for BootRead {
    type Answer = super::BootReport;
    const PATH: &str = "/api/boot";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["boot"];
}

/// `GET /api/backups`, answered with `archives`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct BackupsRead {}

impl crate::Answer for super::Listing {
    const TAGGED: bool = false;
}

impl crate::Read for BackupsRead {
    type Answer = super::Listing;
    const PATH: &str = "/api/backups";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["archives"];
}

/// `GET /api/bandwidth`, answered with `bandwidth`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct BandwidthRead {}

impl crate::Answer for super::Sharing {
    const TAGGED: bool = false;
}

impl crate::Read for BandwidthRead {
    type Answer = super::Sharing;
    const PATH: &str = "/api/bandwidth";
    const SEGMENTS: &[&str] = &[];
    const KINDS: &[&str] = &["bandwidth"];
}

/// `GET /api/bundle/{name}`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct BundleNameRead {
    /// The `name` segment of the path.
    pub name: String,
}

impl crate::FileRead for BundleNameRead {
    const PATH: &str = "/api/bundle/{name}";
    const SEGMENTS: &[&str] = &["name"];
}
