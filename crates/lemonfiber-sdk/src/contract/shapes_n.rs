/// The newest of each kind, by what names them and nothing else.
///
/// What the event stream says. A surface marks a tab from it without reading the
/// items, and reads [`News`] on the screen that lists them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Newest {
    /// The checks most recently found wrong, each with its onset.
    pub problems: Vec<super::NewsCheck>,
    /// The references of the household's newest requests.
    pub requests: Vec<String>,
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
    /// What the household has asked for, the most recently asked first.
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

/// One request, by its reference.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewsRequest {
    /// When it was asked for, where that is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asked: Option<String>,
    /// Who asked for it, by the name the media server holds them under.
    pub by: String,
    /// The reference the household list names it with: the request service's number
    /// (`42`), or `music-<n>` or `books-<n>` for an ask the core keeps.
    pub request: String,
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

/// How any action is run without a question being put.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NonInteractive {
    /// The words an action is run with, as the command line takes them.
    pub form: String,
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
    /// Something a member asked for has arrived; the one kind a member is told.
    #[serde(rename = "ready")]
    Ready,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One back-end, as the read lists it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyBackend {
    /// Who chose it.
    pub chosen_by: super::NotifyChosenBy,
    /// Every failed delivery since its last confirmed one, newest first.
    pub failures: Vec<super::NotifyDelivery>,
    /// Where it is hosted, absent where it is the household's own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosted: Option<super::NotifyHosted>,
    /// The stack's name for it, the same on every read.
    pub id: String,
    /// What it speaks.
    pub kind: super::NotifyBackendKind,
    /// Its newest delivery, absent where it has never delivered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last: Option<super::NotifyDelivery>,
    /// How far it is proven.
    pub proof: super::NotifyProof,
    /// The delivery still to be tried again, absent where none is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<super::NotifyRetry>,
    /// The host and topic or path it reaches, with no credential.
    pub r#where: String,
}

/// What a back-end speaks.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum NotifyBackendKind {
    /// An ntfy server's topic.
    #[serde(rename = "ntfy")]
    Ntfy,
    /// A gotify server's application.
    #[serde(rename = "gotify")]
    Gotify,
    /// An address that is posted each message as JSON.
    #[serde(rename = "webhook")]
    Webhook,
    /// An Apprise API address.
    #[serde(rename = "apprise")]
    Apprise,
    /// A phone's `UnifiedPush` endpoint.
    #[serde(rename = "unifiedpush")]
    Unifiedpush,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Who chose a back-end.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum NotifyChosenBy {
    /// Whoever runs the house.
    #[serde(rename = "operator")]
    Operator,
    /// A household member, by their account's id.
    #[serde(rename = "member")]
    Member(String),
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<NotifyChosenBy, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for NotifyChosenBy {
    const TAG: Option<&str> = None;
    const KNOWN: &[&str] = &["operator", "member"];
}

/// One delivery to one back-end.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyDelivery {
    /// When it was sent, in seconds since the Unix epoch.
    pub at: u64,
    /// Which kind of alert it carried.
    pub kind: super::NotificationKind,
    /// What became of it.
    pub outcome: super::NotifyOutcome,
}

/// What happened, as a service tells it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyEventsBody {
    /// Whether a problem the service raised about itself is cleared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleared: Option<bool>,
    /// Which of the four happenings it is.
    pub event: super::NotifyHappened,
    /// The title's name, or the service's own words for a problem about itself.
    pub name: String,
    /// What the service said went wrong, where something did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The title it is about, by the service's own reference, where it is about one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// One of the four happenings a service tells the core.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum NotifyHappened {
    /// A title a curator fetched is imported into the library.
    #[serde(rename = "imported")]
    Imported,
    /// A grab or an import failed.
    #[serde(rename = "failed")]
    Failed,
    /// The service raised a problem about itself, or cleared one.
    #[serde(rename = "health")]
    Health,
    /// A title was added to a library of the media server.
    #[serde(rename = "added")]
    Added,
}

/// A back-end on a host that is not the household's own.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyHosted {
    /// The host it reaches.
    pub host: String,
    /// What leaves for it, in the operator's words.
    pub leaves: Vec<String>,
}

/// The ways a member may be told that something they asked for is ready, and nothing
/// technical.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyMineReport {
    /// Each way the operator offered the household, in the order it was offered.
    pub offered: Vec<super::NotifyOffered>,
    /// Whether this member's own phone is registered to be woken.
    pub phone: bool,
    /// Whether this was a rehearsal of a choice, with nothing kept.
    pub rehearsed: bool,
}

/// One way to be told that the operator offered the household, as a member reads it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyOffered {
    /// What it is chosen by.
    pub id: String,
    /// The name the household reads it by.
    pub name: String,
    /// Whether this member is told on it.
    pub receiving: bool,
}

/// What became of one delivery.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum NotifyOutcome {
    /// Its arrival was observed.
    #[serde(rename = "confirmed")]
    Confirmed,
    /// The back-end took it, and its arrival was not observed.
    #[serde(rename = "sent")]
    Sent,
    /// The back-end refused it or did not answer, in its own words.
    #[serde(rename = "failed")]
    Failed(String),
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<NotifyOutcome, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for NotifyOutcome {
    const TAG: Option<&str> = None;
    const KNOWN: &[&str] = &["confirmed", "sent", "failed"];
}

/// How far a back-end is proven.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum NotifyProof {
    /// No test sent to it has been seen arriving.
    #[serde(rename = "unproven")]
    Unproven,
    /// A test was seen arriving, at this instant in seconds since the Unix epoch.
    #[serde(rename = "proven")]
    Proven(u64),
    /// Nothing is delivered to it until it is proven again, and why.
    #[serde(rename = "held")]
    Held(String),
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<NotifyProof, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for NotifyProof {
    const TAG: Option<&str> = None;
    const KNOWN: &[&str] = &["unproven", "proven", "held"];
}

/// Where a message goes: every back-end the stack delivers to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyReport {
    /// Each back-end, in the order it was added.
    pub backends: Vec<super::NotifyBackend>,
    /// Whether this was a rehearsal of a change, with nothing kept.
    pub rehearsed: bool,
    /// Where delivery as a whole stands.
    pub state: super::NotifyState,
    /// Each back-end whose record could not be read, with why.
    pub unread: Vec<super::NotifyUnread>,
}

/// A delivery still to be tried again.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyRetry {
    /// Which attempt that will be, from one.
    pub attempt: u32,
    /// When it is tried next, in seconds since the Unix epoch.
    pub next: u64,
    /// How many attempts it is given.
    pub of: u32,
}

/// Where delivery as a whole stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum NotifyState {
    /// No back-end is configured, so nothing is delivered anywhere.
    #[serde(rename = "unconfigured")]
    Unconfigured,
    /// At least one back-end is proven, and none failed its last delivery or proof.
    #[serde(rename = "ready")]
    Ready,
    /// A configured back-end failed its last delivery or its proof.
    #[serde(rename = "degraded")]
    Degraded,
    /// No configured back-end is accepting messages.
    #[serde(rename = "unreachable")]
    Unreachable,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A back-end whose record could not be read.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyUnread {
    /// The back-end, by its id.
    pub backend: String,
    /// Why.
    pub because: String,
}
