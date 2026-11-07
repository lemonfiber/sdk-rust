/// Which protocol a transfer is moving over, since the same download reads
/// differently on each — a Usenet download has no peers, a torrent has no server.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum DashboardProtocol {
    /// A Usenet download.
    #[serde(rename = "usenet")]
    Usenet,
    /// A torrent download.
    #[serde(rename = "torrent")]
    Torrent,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A figure a source reports, kept apart from the two ways it can be missing.
///
/// Zero is a value a source gave; stale is the last value a source that has since
/// gone quiet gave; unknown is a source that never answered at all. Collapsing any
/// two of them sends an operator after the wrong problem — a stalled download and
/// a dashboard that simply stopped polling look identical only if the code lets
/// them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "reading")]
#[non_exhaustive]
pub enum DashboardReading {
    /// The source answered this refresh with a value — which may legitimately be
    /// zero.
    #[serde(rename = "known")]
    Known {
        /// The `value` field.
        value: u64,
    },
    /// The source did not answer this refresh; this is the last value it gave.
    #[serde(rename = "stale")]
    Stale {
        /// The `value` field.
        value: u64,
    },
    /// The source has never answered, so nothing can be said about it.
    #[serde(rename = "unknown")]
    Unknown,
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<DashboardReading, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for DashboardReading {
    const TAG: Option<&str> = Some("reading");
    const KNOWN: &[&str] = &["known", "stale", "unknown"];
}

/// A kind of device somebody in the house might watch on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Device {
    /// What is worth knowing before starting, where anything is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caution: Option<String>,
    /// What to use on it.
    pub client: String,
    /// A link that opens the app already pointed at this server, where the app is known to
    /// take one, with `{address}` where the server's address goes. Absent otherwise, and a
    /// hand-off then carries the address alone as its code, which the app is pointed at by
    /// scanning or typing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep_link: Option<String>,
    /// What somebody would call the device they are holding.
    pub device: String,
    /// What to do instead where this is a bad device to be stuck with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instead: Option<String>,
    /// Whether the app named is open source. A closed one may be named, with this false
    /// beside it, and is never the one recommended.
    pub open_source: bool,
    /// How well served it is.
    pub support: super::Support,
}

/// A challenge, as the phone is handed it to sign.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeviceChallenge {
    /// The challenge, in base64url, signed as it is written here.
    pub challenge: String,
    /// When it stops standing, one minute after it was issued.
    pub expires: String,
}

/// An attestation as a phone sends it, tagged by the platform that made it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "platform")]
pub enum DeviceKeyAttestation {
    /// An Android key attestation.
    #[serde(rename = "android")]
    Android {
        /// The chain, each certificate's DER in base64url, the key's own certificate
        /// first.
        chain: Vec<String>,
    },
    /// An iOS App Attest attestation, made with the SHA-256 of exactly the bytes
    /// `public_key` decodes to as its client data hash.
    #[serde(rename = "ios")]
    Ios {
        /// The attestation object's CBOR, in base64url.
        object: String,
    },
}

/// Where a device key lives, as its attestation says.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum DeviceKeyBacking {
    /// A separate secure chip on an Android phone, as the hardware attested.
    #[serde(rename = "strongbox")]
    Strongbox,
    /// The Secure Enclave of an iPhone: App Attest vouched that the genuine app placed
    /// the key there, not that the hardware proved it.
    #[serde(rename = "enclave")]
    Enclave,
    /// The processor's trusted environment on an Android phone, as the hardware
    /// attested; outside the operating system's reach, without a chip of its own.
    #[serde(rename = "tee")]
    Tee,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Every device key this machine has registered, and what became of a revoke.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeviceKeyListing {
    /// In the order they were registered.
    pub keys: Vec<super::ListedDeviceKey>,
    /// Whether this was a rehearsal: what revoking would come to, with nothing revoked.
    pub rehearsed: bool,
    /// The device key this request revoked, where it revoked one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<String>,
}

/// What a registration is asked with.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeviceKeysBody {
    /// The platform's attestation of where the key lives.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attestation: Option<super::DeviceKeyAttestation>,
    /// What to call it: lower-case letters, digits, dots, dashes and underscores.
    pub name: String,
    /// The operator's password, given again.
    pub password: String,
    /// The public half of a P-256 key pair, as its `SubjectPublicKeyInfo` in base64url,
    /// its point uncompressed.
    pub public_key: String,
}

/// What a quality command did to the stored choice.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Disposition {
    /// The choice was only shown; nothing was asked to change.
    #[serde(rename = "shown")]
    Shown,
    /// The choice was recorded.
    #[serde(rename = "recorded")]
    Recorded,
    /// The choice would be recorded; this was a rehearsal, so it was not.
    #[serde(rename = "rehearsed")]
    Rehearsed,
    /// The choice needs transcoding this host cannot do well, so it was held
    /// rather than recorded without explicit confirmation.
    #[serde(rename = "held")]
    Held,
    /// The recorded preset was re-asserted over the quality sync config, overwriting
    /// a hand-edit where an ordinary run would have preserved it.
    #[serde(rename = "reapplied")]
    Reapplied,
    /// A re-assert that was a rehearsal: it reports whether it would overwrite a
    /// hand-edit, and writes nothing.
    #[serde(rename = "would-reapply")]
    WouldReapply,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What checking a finding disturbs, and the narrowing a diagnosis runs it under.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Disruptive {
    /// The whole seconds the disturbance is bounded to, where the stack bounds it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounded: Option<u64>,
    /// What checking it disturbs, in the stack's words.
    pub disturbs: String,
    /// The check or group a `diagnose` action takes as `only` to run it.
    pub only: String,
}

/// What acting on this stack would take away, one answer per way of acting.
///
/// Named by the situation rather than by the verb, because the verb has two
/// spellings: the command line stops named services with `stop` and the HTTP
/// surface spells that same operation `down`. These bytes are what both surfaces
/// answer with, so a table keyed by verb would be right for one caller and wrong
/// for the other. A field each caller maps its own word onto is right for both.
///
/// Carried on the status reading because a surface deciding whether to stop
/// something has already read that, and because none of these lengths depends on
/// what the stack is currently doing — they are the clocks the run is held to,
/// which is a property of the configuration. A second read to learn a length is
/// a read a surface would skip, and then it would estimate.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Disturbances {
    /// Fetching a form's images.
    pub fetching: super::TakesAway,
    /// Restarting services.
    pub restarting: super::TakesAway,
    /// Bringing services up, whether a whole form or named services inside one.
    pub starting: super::TakesAway,
    /// Taking services down, interrupting anything still arriving.
    pub stopping: super::TakesAway,
    /// Taking the stack down once everything still arriving has landed.
    ///
    /// Whole forms only: stopping named services has no such wait to ask for,
    /// and a caller that asks for both at once is refused.
    pub stopping_after_downloads: super::TakesAway,
    /// Changing the running set, which stops whatever falls outside the new one.
    pub switching: super::TakesAway,
}

/// The family a check belongs to, so a run can be narrowed to one of them.
///
/// These are the diagnostic categories the product recognises; the checks that
/// fill each one arrive over time, so a category may name more than lemonfiber
/// can yet establish.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum DoctorCategory {
    /// Docker present, the daemon reachable, the platform understood.
    #[serde(rename = "environment")]
    Environment,
    /// The data root: reachable, writable, one filesystem, room to grow.
    #[serde(rename = "storage")]
    Storage,
    /// Ports free, bindings matching policy, services reachable.
    #[serde(rename = "network")]
    Network,
    /// Torrent traffic genuinely leaves through the tunnel.
    #[serde(rename = "vpn")]
    Vpn,
    /// Each credential still valid.
    #[serde(rename = "credentials")]
    Credentials,
    /// Health, crash loops, version skew, the wiring between services.
    #[serde(rename = "services")]
    Services,
    /// Provider quota, subscription validity, indexer responsiveness.
    #[serde(rename = "providers")]
    Providers,
    /// Stuck items, repeated import failures, orphaned downloads.
    #[serde(rename = "queue")]
    Queue,
    /// Drift from lemonfiber-managed state, permissions, manifest validity.
    #[serde(rename = "config")]
    Config,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What a diagnostic run found, and what it amounts to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DoctorReport {
    /// Each finding, in the order the checks produced them.
    pub findings: Vec<super::Finding>,
    /// What the findings amount to, as one word.
    pub overall: super::Overall,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}

/// How a single check turned out.
///
/// "Could not check" (`Unverified`) is its own variant rather than a level of
/// severity, so a check that could not run can never be mistaken for one that
/// passed — the dishonesty this whole subsystem exists to prevent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "outcome")]
#[non_exhaustive]
pub enum DoctorVerdict {
    /// Verified working, with the evidence worth showing.
    #[serde(rename = "pass")]
    Pass {
        /// What was observed, where stating it helps — an address, a port.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        note: Option<String>,
    },
    /// Working, but degraded or risky.
    #[serde(rename = "warn")]
    Warn {
        /// The problem that produced this one, where several share a root.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cause: Option<super::Problem>,
        /// The stable identifier for this kind of problem.
        code: super::Code,
        /// The underlying technical detail, available but never leading.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        detail: Option<String>,
        /// What it means for the operator.
        meaning: String,
        /// What to do, most likely first.
        remedies: Vec<super::Remedy>,
        /// How much it matters.
        severity: super::ProblemSeverity,
        /// Where it stands with respect to being fixed.
        state: super::ProblemState,
        /// Every step a run declares, with what each came to, where the problem ended a run
        /// of steps part-way; absent from every other problem.
        ///
        /// Data beside the detail rather than in it, so a client reads what changed
        /// somewhere going back cannot reach without parsing a sentence written for a person.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        steps: Option<Vec<super::ProblemStep>>,
        /// What happened, in one plain sentence.
        summary: String,
    },
    /// Not working.
    #[serde(rename = "fail")]
    Fail {
        /// The problem that produced this one, where several share a root.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cause: Option<super::Problem>,
        /// The stable identifier for this kind of problem.
        code: super::Code,
        /// The underlying technical detail, available but never leading.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        detail: Option<String>,
        /// What it means for the operator.
        meaning: String,
        /// What to do, most likely first.
        remedies: Vec<super::Remedy>,
        /// How much it matters.
        severity: super::ProblemSeverity,
        /// Where it stands with respect to being fixed.
        state: super::ProblemState,
        /// Every step a run declares, with what each came to, where the problem ended a run
        /// of steps part-way; absent from every other problem.
        ///
        /// Data beside the detail rather than in it, so a client reads what changed
        /// somewhere going back cannot reach without parsing a sentence written for a person.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        steps: Option<Vec<super::ProblemStep>>,
        /// What happened, in one plain sentence.
        summary: String,
    },
    /// Could not be established. Never a pass.
    #[serde(rename = "unverified")]
    Unverified {
        /// Why it could not be determined.
        reason: String,
        /// What the operator can do to get an answer.
        remedy: super::Remedy,
    },
    /// A prerequisite was absent, so the check did not apply.
    #[serde(rename = "skipped")]
    Skipped {
        /// Why the check did not apply.
        reason: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<DoctorVerdict, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for DoctorVerdict {
    const TAG: Option<&str> = Some("outcome");
    const KNOWN: &[&str] = &["pass", "warn", "fail", "unverified", "skipped"];
}

/// One download client, and whether it is paused.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Downloader {
    /// The client, by the name the stack knows it under.
    pub client: String,
    /// Whether it is paused, as the client reads it back.
    pub state: super::DownloaderState,
}

/// Whether a download client is fetching, as it reads it back.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum DownloaderState {
    /// It said it is paused.
    #[serde(rename = "paused")]
    Paused,
    /// It said it is fetching.
    #[serde(rename = "fetching")]
    Fetching,
    /// It could not be asked, which is a different thing from either.
    #[serde(rename = "unknown")]
    Unknown,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A profile left out of a closure, and what it would have needed.
///
/// The provider travels with the profile because a name on its own sends the
/// operator looking for a fault. What they have is a stack not configured for
/// one of the two ways of downloading, which is a sentence rather than a word.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Dropped {
    /// The provider it cannot run without.
    pub needs: super::StackProtocol,
    /// The profile that will not run.
    pub profile: String,
}

/// The `Duration` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Duration {
    /// The `nanos` field.
    pub nanos: u32,
    /// The `secs` field.
    pub secs: u64,
}
