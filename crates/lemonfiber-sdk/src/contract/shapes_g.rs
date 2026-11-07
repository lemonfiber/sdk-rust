/// What became of a download the client was asked to let go.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Gone {
    /// What it occupied, as the client reported it.
    pub bytes: u64,
    /// What the client is no longer holding.
    pub name: String,
    /// Whether this was a rehearsal, which asks the client for nothing.
    pub rehearsed: bool,
}

/// A grant to play on a member's own account, the token the device plays with, and how
/// long it lasts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantReport {
    /// Whether a session was opened for the device. A rehearsal opens none.
    pub granted: bool,
    /// The last day the grant holds unless the member's client speaks to the core
    /// before then, as `YYYY-MM-DD`.
    pub lasts_until: String,
    /// The member the device now plays as, by the name they are known by.
    pub member: String,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    pub rehearsed: bool,
    /// The token the device presents at the guarded front door, as
    /// `Authorization: Bearer <token>`. Answered once, here, and kept by nothing in the
    /// core; absent where nothing was granted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// The entries of one kind, under the name an operator reads them by.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Group {
    /// The changes, in the order they were made.
    pub entries: Vec<super::Entry>,
    /// What this group of changes is: new, fixed, faster, or maintenance.
    pub title: String,
}

/// One question the VPN step asks of the service filling `network.egress-guard`.
///
/// Asked in order. A choice offers only the values whose needs the answers before it
/// meet, and a value or a secret is asked only where its own needs are met.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "asks")]
#[non_exhaustive]
pub enum GuardQuestion {
    /// One of a setting's values.
    #[serde(rename = "choice")]
    Choice {
        /// Every value it accepts, each with what it is a choice under.
        choices: Vec<super::Choosable>,
        /// What it holds where it is left unanswered.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<String>,
        /// The setting.
        setting: String,
    },
    /// A setting's value, which only the operator holds.
    #[serde(rename = "value")]
    Value {
        /// The values of other settings under which it is asked.
        needs: std::collections::BTreeMap<String, Vec<String>>,
        /// The setting.
        setting: String,
    },
    /// A secret, entered without being shown.
    #[serde(rename = "secret")]
    Secret {
        /// The values of settings under which it is asked.
        needs: std::collections::BTreeMap<String, Vec<String>>,
        /// The name the secret store holds it under.
        secret: String,
        /// What holding it is for.
        why: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<GuardQuestion, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for GuardQuestion {
    const TAG: Option<&str> = Some("asks");
    const KNOWN: &[&str] = &["choice", "value", "secret"];
}

/// The guidance in full, for a surface that shows all of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Guidance {
    /// Every device, in the order somebody is likely to be holding one.
    pub devices: Vec<super::Device>,
    /// What this will not do for them.
    pub nothing_is_installed: String,
    /// True of every device, said once.
    pub only_at_home: String,
    /// Why playback here is likely to struggle before any app is chosen, or `None`
    /// where the preset in force asks for nothing this platform cannot serve.
    ///
    /// Absent far more often than present, and it must be: a caution shown to
    /// everybody says nothing about anybody's machine, and a reader who meets one
    /// every time stops reading it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub straining: Option<super::Straining>,
    /// What to do when it does not work, keyed by the symptom.
    pub trouble: Vec<super::Trouble>,
}
