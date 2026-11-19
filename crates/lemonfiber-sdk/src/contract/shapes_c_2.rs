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
