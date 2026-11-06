/// `GET /api/backups`, answered with `archives`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Backups {}

impl crate::Answer for super::Listing {
    const TAGGED: bool = false;
}

impl crate::Read for Backups {
    type Answer = super::Listing;
    const PATH: &'static str = "/api/backups";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["archives"];
}

/// `GET /api/bandwidth`, answered with `bandwidth`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Bandwidth {}

impl crate::Answer for super::Sharing {
    const TAGGED: bool = false;
}

impl crate::Read for Bandwidth {
    type Answer = super::Sharing;
    const PATH: &'static str = "/api/bandwidth";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["bandwidth"];
}

/// `GET /api/bundle/{name}`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct BundleName {
    /// The `name` segment of the path.
    pub name: String,
}

impl crate::FileRead for BundleName {
    const PATH: &'static str = "/api/bundle/{name}";
    const SEGMENTS: &'static [&'static str] = &["name"];
}
