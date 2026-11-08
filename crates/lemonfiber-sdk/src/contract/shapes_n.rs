/// The newest of each kind, by what names them and nothing else.
///
/// What the event stream says. A surface marks a tab from it without reading the
/// items, and reads [`News`] on the screen that lists them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Newest {
    /// The checks most recently found wrong, each with its onset.
    pub problems: Vec<super::NewsCheck>,
    /// The numbers of the household's newest requests.
    pub requests: Vec<i64>,
    /// The kinds that could not be read, as [`News::unread`] names them.
    pub unread: Vec<super::NewsKind>,
    /// The versions of the newest releases in the record this build carries.
    pub updates: Vec<String>,
}

/// What a surface can mark as new, newest first within each kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct News {
    /// The checks found wrong, the most recent onset first.
    pub problems: Vec<super::NewsProblem>,
    /// What the household has asked for, highest number first.
    pub requests: Vec<super::NewsRequest>,
    /// The kinds that could not be read.
    ///
    /// A kind named here has an empty list because nothing could be read, not because
    /// nothing is there. A surface that took the empty list as everything there is
    /// would mark all of it as new once it could be read again.
    pub unread: Vec<super::NewsKind>,
    /// The releases in the record this build carries, newest first.
    pub updates: Vec<super::NewsUpdate>,
}

/// A check found wrong, by the check and when it went wrong.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewsCheck {
    /// The check that raised it.
    pub check: String,
    /// When the stack first saw it wrong since it last saw it right, in whole seconds
    /// since the epoch.
    pub onset: String,
}

/// One of the three kinds of thing a surface can mark as new.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum NewsKind {
    /// Releases of lemonfiber.
    #[serde(rename = "updates")]
    Updates,
    /// What the household has asked for.
    #[serde(rename = "requests")]
    Requests,
    /// Checks found wrong.
    #[serde(rename = "problems")]
    Problems,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One check found wrong, by the check and when it went wrong.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewsProblem {
    /// The check that raised it.
    pub check: String,
    /// When the stack first saw it wrong since it last saw it right, in whole seconds
    /// since the epoch: the same moment the health summary names for it.
    pub onset: String,
    /// What is wrong, in one line.
    pub summary: String,
}

/// One request, by its number.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewsRequest {
    /// Who asked for it, by the name the media server holds them under.
    pub by: String,
    /// The number the request service files it under.
    pub number: i64,
    /// What it is called, where a service has been told about it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// One release, by its version.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewsUpdate {
    /// What it set out to deliver, where the record says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivers: Option<String>,
    /// The version, without the tag's leading letter.
    pub version: String,
}

/// One thing to do next.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Next {
    /// Add more content, now that the shape of it is understood.
    #[serde(rename = "more-content")]
    MoreContent,
    /// Let the rest of the household ask for things themselves.
    #[serde(rename = "household")]
    Household,
    /// Watch it somewhere other than the machine it is running on.
    #[serde(rename = "client-apps")]
    ClientApps,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What a surface shows about the record, given the version asking.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Notes {
    /// Every release the record holds, newest first.
    pub releases: Vec<super::ReleaseSummary>,
    /// What each requirement the running release cites is, and where it is defined.
    pub requirements: std::collections::BTreeMap<String, super::Requirement>,
    /// What the running version changed, where the record holds its release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<super::Release>,
    /// Whether the record describes what this build could have shipped.
    pub state: super::ChangelogState,
}

/// What sort of thing an alert tells the operator.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum NotificationKind {
    /// A condition at warning or worse, at onset and resolution.
    #[serde(rename = "problem")]
    Problem,
    /// Work the operator asked for finishing.
    #[serde(rename = "completion")]
    Completion,
    /// An update to the stack or lemonfiber being available.
    #[serde(rename = "update")]
    Update,
    /// A backup that succeeded.
    #[serde(rename = "backup")]
    Backup,
    /// A household member's request newly waiting on the operator.
    #[serde(rename = "request")]
    Request,
    /// An invitation accepted, declined or lapsed.
    #[serde(rename = "member")]
    Member,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}
