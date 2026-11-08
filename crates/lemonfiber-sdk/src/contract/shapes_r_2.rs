/// Which of the two volumes a reading is about.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Role {
    /// Where the media and the downloads live.
    #[serde(rename = "data")]
    Data,
    /// Where the services keep their own configuration and databases.
    #[serde(rename = "services")]
    Services,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A directory everything lemonfiber keeps sits under.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Root {
    /// The directory itself.
    pub at: String,
    /// What lives under it, and what losing it would cost.
    pub what: String,
}

/// What one rotation came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rotation {
    /// Every consumer, and how far the rotation reached it.
    pub consumers: Vec<super::Propagation>,
    /// Which credential was to be replaced.
    pub credential: String,
    /// The offer a replacement that takes one answers, where this reading named one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// What became of the replacement.
    pub settled: super::Settled,
}
