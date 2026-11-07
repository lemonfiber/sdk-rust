/// How large a step from one version to another is.
///
/// Named after the part of the version that moved rather than after a size, because
/// that is the fact an operator weighs: a first-number change is where a project puts
/// the work that breaks configurations, and the two behind it are where it puts the
/// work that does not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
}
