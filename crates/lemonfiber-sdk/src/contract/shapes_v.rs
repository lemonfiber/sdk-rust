/// What proving a credential against its live service established — never the
/// input, only the outcome.
///
/// Read back as well as built. A surface that is not in this process asks setup to
/// prove a credential and is told what came of it, so the four outcomes are tagged
/// by name rather than distinguished by which field is present — the same reason an
/// answer carries the step it belongs to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "outcome")]
#[non_exhaustive]
pub enum Validation {
    /// Proven working, carrying the capability observed while proving it.
    #[serde(rename = "valid")]
    Valid {
        /// The observed fact — what the service did, not that it merely answered.
        observed: String,
    },
    /// The service answered and refused: the credential is wrong for it.
    #[serde(rename = "rejected")]
    Rejected {
        /// What the service said, in terms the operator can act on.
        detail: String,
    },
    /// Nothing usable answered, so nothing can be concluded about the credential.
    #[serde(rename = "unreachable")]
    Unreachable {
        /// Why nothing usable came back.
        detail: String,
    },
    /// It authenticated, but cannot do the job it is for — exhausted, limited, or
    /// otherwise unable.
    #[serde(rename = "degraded")]
    Degraded {
        /// What it can no longer do, and why where the service says.
        detail: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<Validation, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Validation {
    const TAG: Option<&str> = Some("outcome");
    const KNOWN: &[&str] = &["valid", "rejected", "unreachable", "degraded"];
}

/// Where a value in force came from.
///
/// Published under a name of its own because the generated schema keys on the type's
/// bare name, and a second `Origin` in the crate would be merged with this one into a
/// single definition carrying the variants of both — a published contract saying a
/// credential may be *unknown* and a setting may be *service*, neither of which is
/// true, and neither of which the additive-only surface check would refuse.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "origin")]
#[non_exhaustive]
pub enum ValueOrigin {
    /// This build's own, out of what lemonfiber ships rather than out of a choice.
    #[serde(rename = "bundled")]
    Bundled,
    /// The operator settled it, whether by answering for it or by editing it since.
    #[serde(rename = "operator")]
    Operator,
    /// A named plugin set it.
    #[serde(rename = "plugin")]
    Plugin {
        /// Which one, so the thread back to it is a name rather than a search.
        named: String,
    },
    /// It could not be established, and what stopped it.
    #[serde(rename = "unknown")]
    Unknown {
        /// What stopped it being established, so the gap reads as a reason rather
        /// than as a shrug.
        why: String,
    },
    /// An installed plugin set it over a value that was there before, and that value
    /// is carried with it: what is in force and what it replaced are read together.
    #[serde(rename = "overridden")]
    Overridden {
        /// Which plugin set what is in force.
        named: String,
        /// What it replaced, and where that came from.
        replaced: Box<super::ValueReplaced>,
    },
    /// A plugin set it and is no longer installed, and the value is still in force.
    ///
    /// Only where the record of what is installed was read and does not hold that
    /// plugin. A record that would not read cannot say a plugin is gone, so that is
    /// an unknown rather than this.
    #[serde(rename = "orphaned")]
    Orphaned {
        /// Which plugin set it.
        named: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<ValueOrigin, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for ValueOrigin {
    const TAG: Option<&str> = Some("origin");
    const KNOWN: &[&str] = &[
        "bundled",
        "operator",
        "plugin",
        "unknown",
        "overridden",
        "orphaned",
    ];
}

/// The value a plugin's change replaced, and where that value came from.
///
/// Its own origin rather than assumed to be this build's default: before a plugin
/// set a value, the operator may have, or another plugin, and calling that value
/// *bundled* would tell somebody putting it back that they are returning to a
/// default when they are returning to a choice.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ValueReplaced {
    /// Where the replaced value came from, read from the record of the change that
    /// wrote it, and unknown where nothing recorded one.
    pub from: Box<super::ValueOrigin>,
    /// What it held, where there was a value and it may be shown. Nothing where nothing
    /// was set — this build's default was in force — or where it is withheld.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Whether a value is withheld because the setting holds a credential. A replaced
    /// credential is shown as sealed and never in clear, as the one in force is.
    pub withheld: bool,
}

/// What versions are in play: the binary, the stack it operates, and what changed.
///
/// The changelog is here rather than behind a request of its own because it answers
/// the second half of the same question. "Which version am I on" is asked by
/// somebody deciding whether to move, and what they need next is what the version
/// they are on actually brought — so every surface that already reaches this read
/// reaches both halves, and none of the three had to learn a new question.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VersionReport {
    /// When the snapshot of Google's attestation status list this build carries was
    /// taken, which an Android device key's chain is checked against; nothing where it
    /// does not read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attestation_status: Option<String>,
    /// The running binary's version.
    pub binary: String,
    /// What this build's release changed, and every release there has been.
    pub changelog: super::Notes,
    /// What the container engine reports, when it could be asked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compose: Option<String>,
    /// The version of the stack this build operates.
    pub stack: String,
    /// The manifest schema generations this build reads.
    pub supported_schema: Vec<u32>,
}

/// The watch a run would keep, and what it would do at the end of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vigil {
    /// The invocation it would run the moment that location went, word for word.
    ///
    /// Built by the same path a real watch stops the services through, rather than
    /// described beside it: an argv reported from a second reckoning is one nobody
    /// runs, and the one nobody runs is the one that stops being right.
    pub command: Vec<String>,
    /// How often it would look, in seconds.
    pub every: u64,
    /// The data location it would hold.
    pub root: String,
}

/// Every word this product explains, for somebody who asked what there is to ask
/// about.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vocabulary {
    /// The words, in the order somebody meets them.
    pub words: Vec<super::Term>,
}

/// One volume, as one run measured it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Volume {
    /// The path that was measured.
    pub at: String,
    /// The bytes already committed to landing here.
    pub committed: u64,
    /// Bytes free, or nothing where the volume could not be read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<u64>,
    /// Where it stands.
    pub level: super::Level,
    /// The effective limit — the mount's own size, which on a dataset given a
    /// quota is the quota rather than the device beneath it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Where the volume holding it is mounted, which is what the limit belongs to.
    pub point: String,
    /// What would be free once the committed content has landed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projected: Option<u64>,
    /// What the reading is worth.
    pub reading: super::Freshness,
    /// Which of the two this is.
    pub role: super::Role,
}

/// What the VPN is doing, and whether the download client is actually behind it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vpn {
    /// The country that address is in.
    pub country: String,
    /// Whether the download client's own egress address matches the tunnel's —
    /// the one thing that proves traffic is genuinely leaving through it.
    pub egress_matches: bool,
    /// The tunnel's exit address as the outside world sees it.
    pub exit_ip: String,
    /// The port the provider forwards, where forwarding is on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forwarded_port: Option<u16>,
}
