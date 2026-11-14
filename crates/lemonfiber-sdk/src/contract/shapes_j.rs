/// Why work ended before it finished.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum JobEnding {
    /// Its name was released.
    #[serde(rename = "released")]
    Released,
    /// Nothing asked about it for a whole lease, so it was let go.
    #[serde(rename = "let-go")]
    LetGo,
    /// The services it followed stopped saying anything.
    #[serde(rename = "quiet")]
    Quiet,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// How large a step from one version to another is.
///
/// Named after the part of the version that moved rather than after a size, because
/// that is the fact an operator weighs: a first-number change is where a project puts
/// the work that breaks configurations, and the two behind it are where it puts the
/// work that does not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Jump {
    /// The first number moved.
    #[serde(rename = "major")]
    Major,
    /// The second number moved, and the first did not.
    #[serde(rename = "minor")]
    Minor,
    /// Nothing before the third number moved.
    #[serde(rename = "patch")]
    Patch,
    /// One of the tags is not a run of numbers, so the size of the step is unknown.
    #[serde(rename = "untellable")]
    Untellable,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}
