/// The record written inside an archive, and read back to decide a restore.
///
/// Everything a restore needs to know before it overwrites anything: what made
/// the archive, when, what data root it was taken against, what it covers, whether
/// it is sensitive, and the contents to list. Round-trips through JSON so the same
/// value the capture wrote is the value the restore reads.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BackupManifest {
    /// When it was taken. Opaque here; the surface stamps it from the clock.
    pub created_at: String,
    /// The data root it was taken against, to notice a restore to a different one.
    pub data_root: String,
    /// What is inside, for a listing shown before anything is overwritten.
    pub members: Vec<super::Member>,
    /// The lemonfiber version that wrote it, checked against the one restoring.
    pub product_version: String,
    /// The archive format, checked before anything inside is trusted.
    pub schema: u32,
    /// What it covers.
    pub scope: super::Scope,
    /// Whether it carries credentials, and so must be handled as sensitive.
    pub sensitive: bool,
}

/// What a capture produced.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BackupReport {
    /// What the capture moved, against what a capture is meant to stay inside.
    ///
    /// Read off the room check that already ran, so saying it costs nothing: the trees
    /// were walked to decide whether the archive would fit, and this is the same number
    /// put to a second use.
    pub pace: super::Pace,
    /// Where the archive was written, or — on a run that only said what it would
    /// capture — where it would have gone.
    pub path: String,
    /// The older backups retention pruned, oldest first — or would prune.
    pub pruned: Vec<String>,
    /// Whether this run only said what it would capture.
    ///
    /// A flag rather than a second shape, because every other field means the same
    /// thing either way: a capture is settled before it is written — the room is
    /// measured, the manifest described, the name and the path derived, and retention
    /// worked out — so what a rehearsal reports is what a real run would report, with
    /// the one write left out. What changes is the tense a surface says it in.
    pub rehearsed: bool,
    /// What the backup covers.
    pub scope: super::Scope,
    /// Whether it carries credentials, and so must be handled as sensitive.
    pub sensitive: bool,
}

/// One direction on one client: what it was asked for, took, and is doing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BandwidthHeld {
    /// What it reports as in force.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<u64>,
    /// What it was asked to hold to, in bytes a second, where anything was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asked: Option<u64>,
    /// What it is moving right now, where it reported a figure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moving: Option<u64>,
    /// What that adds up to.
    pub verdict: super::BandwidthVerdict,
}

/// One direction's limit, as declared and as it comes to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BandwidthReading {
    /// The limit as it was expressed.
    pub limit: super::Limit,
    /// What it comes to against the measured line.
    pub resolved: super::Resolved,
    /// The limit and the line it was measured against, in one sentence.
    ///
    /// Carried rather than left to each surface, so the rule that a share is never
    /// shown without the figure it is a share of is kept in one place instead of
    /// three.
    pub says: String,
}

/// What became of one limit, in one direction, on one client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum BandwidthVerdict {
    /// Nothing was asked for, so there is nothing to hold to.
    #[serde(rename = "unasked")]
    Unasked,
    /// This client has no such direction to limit at all.
    ///
    /// Apart from a limit that was ignored, and deliberately: a Usenet client does
    /// not upload, so an upload limit on one is not a setting it refused.
    #[serde(rename = "nothing-to-limit")]
    NothingToLimit,
    /// It took the limit and is inside it.
    #[serde(rename = "holding")]
    Holding,
    /// It reports a figure other than the one it was given.
    #[serde(rename = "ignored")]
    Ignored,
    /// It reports the right figure and is moving faster than it.
    #[serde(rename = "overrunning")]
    Overrunning,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What asking for a title came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Became {
    /// Asked for, and on its way without anybody saying yes first.
    #[serde(rename = "asked")]
    Asked,
    /// Asked for, and waiting for whoever runs the house to say yes.
    #[serde(rename = "waiting")]
    Waiting,
    /// Somebody already asked for it, so nothing new was asked for.
    #[serde(rename = "on-its-way")]
    OnItsWay,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What standing lemonfiber beside an existing setup came to, or would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BesideReport {
    /// Where each service would listen instead, lowest original port first.
    pub ports: Vec<super::MovedReport>,
    /// Why nothing was written, where nothing was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Where the act stands.
    pub stance: super::Stance,
    /// The settings file the moves were recorded in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub written: Option<String>,
}

/// A repair that has run out of chances, and where to go instead.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Beyond {
    /// The check whose fault has outlasted every attempt at it.
    pub check: String,
    /// What to do about it now that lemonfiber has stopped offering to.
    pub remedy: super::Remedy,
}

/// What a run at boot came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum BootOutcome {
    /// The stack came back, tunnel and forwarded port included.
    #[serde(rename = "passed")]
    Passed,
    /// The stack, or its tunnel, did not come back.
    #[serde(rename = "failed")]
    Failed,
    /// The rest came back, and a part waits for its install from this build.
    #[serde(rename = "waiting-for-install")]
    WaitingForInstall,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// When this machine last started, what the run after it came to, and how long the
/// stack has failed to come back.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BootReport {
    /// When this machine last started, in seconds since the Unix epoch, where it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub booted_at: Option<u64>,
    /// Why a run after that start started nothing, where one did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declined: Option<String>,
    /// How many starts it has failed to come back after since then.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_boots: Option<u32>,
    /// Since when the stack has failed to come back, in seconds since the Unix epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failing_since: Option<u64>,
    /// The newest verification kept against any start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_verified: Option<super::BootVerification>,
    /// The verification kept against that start, where one ran after it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<super::BootVerification>,
}

/// Whether the tunnel came back with the stack.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "state")]
#[non_exhaustive]
pub enum BootTunnel {
    /// Its checks passed.
    #[serde(rename = "held")]
    Held,
    /// A check of it failed.
    #[serde(rename = "not-established")]
    NotEstablished {
        /// Which, in the stack's words.
        said: String,
    },
    /// Nothing checked it: the stack runs no tunnel, did not come up, or could not be
    /// read.
    #[serde(rename = "not-checked")]
    NotChecked,
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<BootTunnel, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for BootTunnel {
    const TAG: Option<&str> = Some("state");
    const KNOWN: &[&str] = &["held", "not-established", "not-checked"];
}

/// What one run at boot that reached the stack came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BootVerification {
    /// When the machine started that the run came after, where it said.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub against: Option<u64>,
    /// When the run finished.
    pub at: u64,
    /// Each service that did not come back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_back: Option<Vec<String>>,
    /// What it came to.
    pub outcome: super::BootOutcome,
    /// The forwarded port before the start and after it, where the stack forwards one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<super::Forwarded>,
    /// The sentence the run raised, where it raised one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub said: Option<String>,
    /// Whether the tunnel came back with the stack.
    pub tunnel: super::BootTunnel,
}

/// What a support request said: what a bundle holds, and where it is if it exists.
///
/// One record with an absent path rather than two shapes, because the two answers
/// are the same answer at two moments: both list what goes in the file and say how
/// large it is, and only one of them has a file to point at. A caller reads whether
/// there is a path to know which it has.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Bundle {
    /// How large the file is, or would be.
    pub bytes: u64,
    /// Everything it holds, gathered, redacted and read back.
    pub contents: super::Contents,
    /// Where it was written, or nothing where a run that writes nothing described it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Where it would be written, on the run that only describes one.
    ///
    /// The other half of what a description is for. What goes in the file and how
    /// large it is answer *whether* to make it; where it lands answers *where to find
    /// it*, and an operator deciding at a shell needs both at the one moment the
    /// answer can still change what they do. Resolved by the same function the run
    /// that writes resolves it with, so the path shown and the path written are one.
    ///
    /// Absent on a run that wrote one — `path` is then where it went — and absent on a
    /// machine that would not say where lemonfiber keeps its own files, which is the
    /// one destination of the three that needs that answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub would_go: Option<String>,
}
