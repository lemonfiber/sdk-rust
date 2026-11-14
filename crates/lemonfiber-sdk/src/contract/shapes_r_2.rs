/// How far a removal got, across the two services a household member exists on.
///
/// The media server is removed first and the request service second, because the
/// request service authenticates *through* the media server — so once the first is gone
/// they can do nothing either way, and a failure at the second leaves an account that
/// cannot sign in rather than somebody who can still watch.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Revoked {
    /// Both, which is what removal means.
    #[serde(rename = "everywhere")]
    Everywhere,
    /// The media server only, and the request service still holds an account.
    ///
    /// They can neither watch nor ask — the account left behind signs in through the one
    /// that is gone — but something is there that should not be, and the next run of this
    /// command takes it.
    #[serde(rename = "media-server-only")]
    MediaServerOnly,
    /// Nothing was removed, because nothing was confirmed.
    #[serde(rename = "nothing")]
    Nothing,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// The hours the household is awake, declared once for every download client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rhythm {
    /// When the household's day starts.
    pub from: String,
    /// When it ends, which may be the next morning.
    pub to: String,
}

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
