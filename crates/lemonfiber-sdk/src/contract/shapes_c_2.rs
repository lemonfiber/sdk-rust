/// How far a rotation reached one consumer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "reach")]
#[non_exhaustive]
pub enum CredentialReach {
    /// It now holds the replacement.
    #[serde(rename = "updated")]
    Updated,
    /// It will hold the replacement once one more thing happens, and that thing is
    /// named. A consumer reading the value out of a container's environment has it
    /// fixed at the moment the container was created, so recording a new one is
    /// only half of reaching it.
    #[serde(rename = "pending")]
    Pending {
        /// What still has to happen, written as the command that does it.
        detail: String,
    },
    /// It could not be updated. Named rather than dropped, because a consumer left
    /// holding the old value is the failure this list exists to surface.
    #[serde(rename = "failed")]
    Failed {
        /// Why it could not be.
        detail: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<CredentialReach, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for CredentialReach {
    const TAG: Option<&str> = Some("reach");
    const KNOWN: &[&str] = &["updated", "pending", "failed"];
}

/// What revoking one grant came to, or would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CredentialRevoked {
    /// The name this reading goes by, which the revoke takes as its yes.
    pub agreement: String,
    /// Whether the grant was ended, rather than read or rehearsed.
    pub ended: bool,
    /// The grant it ends.
    pub grant: super::CredentialGranted,
    /// The plugin's services that read the credential, restarted once it is withdrawn.
    pub restarts: Vec<String>,
    /// Why its services did not restart, where they did not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unrestarted: Option<String>,
}

/// Whose credential asked, as a client is told it.
///
/// Said rather than left to be inferred from which requests are permitted: a client
/// guessing a member from a missing read would guess wrong the day that read moves.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CredentialScope {
    /// The operator: the password's session, or the token printed at this machine.
    #[serde(rename = "operator")]
    Operator,
    /// A key that reads and acts on nothing.
    #[serde(rename = "read")]
    Read,
    /// A key that reads, and calls the actions the contract publishes for a key.
    #[serde(rename = "act")]
    Act,
    /// A household member: their own session, or a key scoped to them.
    #[serde(rename = "member")]
    Member,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where a credential stands, as far as lemonfiber can tell without spending it.
///
/// Six states rather than a boolean because the operator's next move differs for
/// each: an absent credential is one to supply, a stale one is one to prove, an
/// invalid one is one to replace, and the two rotation states exist so a run
/// interrupted half-way through a replacement is legible rather than mysterious.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CredentialState {
    /// Required by what this stack runs, and not supplied.
    #[serde(rename = "absent")]
    Absent,
    /// Present, and proven against the service it authenticates to.
    #[serde(rename = "active")]
    Active,
    /// Present, and never proven — or not proven since it was last written.
    #[serde(rename = "stale")]
    Stale,
    /// Present, and refused by the service the last time it was offered.
    #[serde(rename = "invalid")]
    Invalid,
    /// A replacement is being proven; the existing value is still the one in force.
    #[serde(rename = "rotating")]
    Rotating,
    /// Replaced. What is left is the old value, waiting to be destroyed.
    #[serde(rename = "superseded")]
    Superseded,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// How much a service's absence costs.
///
/// Serialisable as well as readable, because it reaches an operator: a status
/// report that says a service is down without saying whether that matters
/// leaves them to guess, and the manifest already holds the answer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Criticality {
    /// Its failure has consequences outside the machine.
    #[serde(rename = "critical")]
    Critical,
    /// The form does not work without it.
    #[serde(rename = "core")]
    Core,
    /// The form works, badly.
    #[serde(rename = "important")]
    Important,
    /// Makes things better.
    #[serde(rename = "enhancing")]
    Enhancing,
    /// Nice to have.
    #[serde(rename = "optional")]
    Optional,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}
