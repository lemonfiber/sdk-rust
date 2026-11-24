/// A proposed change, read against what is in force, and where it stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Review {
    /// The difference, as it would be applied.
    pub change: super::ConfigChange,
    /// What the change comes to on this machine, beyond the value it changes.
    ///
    /// Filled by whoever went and asked — the services where they file, the clients
    /// what they are fetching — and empty on every change that comes to nothing
    /// beyond its value. Carried on a staged proposal as well as an applied one: a
    /// review that withheld this until after the yes would be asking for a yes to
    /// something unstated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub findings: Option<super::Findings>,
    /// What proving the replacement credential came to, where one was proven.
    ///
    /// A replacement is proven against its live service before the credential it
    /// replaces is discarded, so this is present for exactly those settings and
    /// absent everywhere else.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<super::Validation>,
    /// Why nothing was written, where nothing was and the reason is not simply that
    /// somebody has yet to say yes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Where the proposal stands.
    pub stance: super::Stance,
}

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

/// How a run-and-exit service's last run exited.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum RunHealth {
    /// It has not run yet.
    #[serde(rename = "not-run")]
    NotRun,
    /// It exited zero.
    #[serde(rename = "succeeded")]
    Succeeded,
    /// It exited non-zero, or ended before it exited.
    #[serde(rename = "failed")]
    Failed,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What `lemonfiber run` came to for one service.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RunReport {
    /// Whether a time fell due, a run was owed or one was asked for at once, so it ran or
    /// would have.
    pub due: bool,
    /// Why nothing was looked at, where something held it back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub held: Option<String>,
    /// The run, where one went.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ran: Option<super::ScheduledRun>,
    /// Whether nothing was run because this was a rehearsal.
    pub rehearsed: bool,
    /// The service.
    pub service: String,
    /// How many times fell due while that run was still going, each skipped.
    pub skipped: u32,
}
