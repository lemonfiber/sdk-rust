/// What a reader needs to know before reading a word of the bundle.
///
/// An operator pasting last week's bundle into this week's thread is the commonest way one
/// of those threads goes wrong, and nothing in the contents tells either of them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Taken {
    /// When, as a service writes a moment.
    pub at: String,
    /// The lemonfiber that wrote it.
    pub lemonfiber: String,
    /// The stack it was written from.
    pub stack: String,
}

/// How long one way of acting on the stack takes something away for.
///
/// Two cases rather than a length and a flag, because *no bound* is not a long
/// bound and a surface offered a number plus a "really, though?" beside it will
/// show the number. A reader that handles both arms has said both things; one
/// that handles only the first does not compile.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "bound")]
#[non_exhaustive]
pub enum TakesAway {
    /// It ends, and this is the longest the run is held to.
    #[serde(rename = "bounded")]
    Bounded {
        /// The length, in seconds.
        ///
        /// Seconds rather than the engine's own duration shape, because every
        /// caller of this turns it into a sentence and none of them wants
        /// nanoseconds to do it.
        seconds: u64,
    },
    /// Nothing bounds it, and this is what it is waiting for.
    #[serde(rename = "open-ended")]
    OpenEnded {
        /// What has to happen before it ends.
        until: super::Awaiting,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// What a set of files occupies, counted both ways.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tally {
    /// How many names were counted.
    pub files: u64,
    /// The bytes the names add up to — what this would take with nothing shared.
    pub logical: u64,
    /// The bytes the underlying files add up to — what the volume has actually
    /// lost to them.
    pub physical: u64,
    /// How many of those names pointed at a file already counted.
    pub shared: u64,
}

/// How the screen itself is doing, which is a different question from how the
/// stack is doing.
///
/// The stack's own verdict is [`crate::health::Standing`]; this is only whether the
/// picture can be trusted to be current. Kept apart because they disagree in both
/// directions: a healthy stack can be shown through half-failing telemetry, and a
/// perfectly refreshing screen can be reporting a stack that is on fire.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Telemetry {
    /// Telemetry is current and refreshing normally.
    #[serde(rename = "live")]
    Live,
    /// Some sources are unavailable; their panels are marked.
    #[serde(rename = "degraded")]
    Degraded,
    /// The engine cannot be reached, though the CLI path may still work.
    #[serde(rename = "disconnected")]
    Disconnected,
    /// Configured, but nothing is running.
    #[serde(rename = "no-stack")]
    NoStack,
    /// No configuration; setup is offered instead.
    #[serde(rename = "unconfigured")]
    Unconfigured,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A word this product uses, and what somebody meeting it needs to know.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Term {
    /// What other services in this stack call the same thing.
    ///
    /// A curator and a Usenet client do not agree on words, and an operator moving between
    /// their screens should not have to work out that two of them are one.
    pub also_called: Vec<String>,
    /// More, for somebody who asks — never needed in order to act.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep: Option<String>,
    /// The other forms this product itself writes the word in, where a state or a
    /// stage is named by one — `grabbed` for `grab`, `seeding` for `seed`.
    ///
    /// Apart from [`Self::also_called`], which is another service's word and one this
    /// product must never write as its own. These are this product's own words, and a
    /// surface explaining a word it was sent looks for the word it was sent here, so
    /// that nothing on the far side has to guess which term an inflection belongs to.
    pub forms: Vec<String>,
    /// One sentence: what it is for and what it costs or gains.
    ///
    /// Enough to act on. Somebody who reads only this should not be stuck.
    pub short: String,
    /// The word as it appears in the interface.
    pub word: String,
}

/// How a bundle was made: what was bounded, what was replaced, and what its operator asked
/// to have shown as it is.
///
/// Carried in the bundle rather than known only to the command that wrote it, because the
/// person who reads one is usually not the person who chose any of this.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Terms {
    /// Whether media filenames were shown.
    pub filenames: super::Filenames,
    /// The settings the operator asked to have shown as they are.
    pub revealed: Vec<String>,
    /// How much of the logs was taken, said as it would be said aloud.
    pub window: String,
}

/// Which of the four removals was asked for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Tier {
    /// Nothing removed. The services are stopped and everything stays where it is.
    #[serde(rename = "stop")]
    Stop,
    /// The containers, the networks between them, and the images that were pulled.
    #[serde(rename = "services")]
    Services,
    /// Each service's own configuration, lemonfiber's own state, and the credentials
    /// in both.
    #[serde(rename = "configuration")]
    Configuration,
    /// The library and the downloads.
    #[serde(rename = "media")]
    Media,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What one title is, as a member's own account reads it.
///
/// What a person deciding whether to watch it wants, and nothing about how it is
/// stored: no file, no container, no bitrate.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Title {
    /// Where its backdrop is served, where it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backdrop: Option<String>,
    /// The certificate it carries where the operator lives, where it carries one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// The certificate the door presents, which a client pins, beside any location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub door: Option<super::Pinned>,
    /// The genres the server files it under.
    pub genres: Vec<String>,
    /// The identifier the server tells it apart by, which is what asking to play one
    /// of them names.
    pub id: String,
    /// Which of the kinds this product deals in it is.
    pub medium: super::Medium,
    /// How long it runs, in whole minutes, where the server knows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<u32>,
    /// What it is about, where the server holds a description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    /// Where its poster is served, where it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster: Option<String>,
    /// When it came out, as a calendar date, where the server knows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub released: Option<String>,
    /// A series' seasons, each with its episodes, in order. Empty for anything else.
    pub seasons: Vec<super::Season>,
    /// Where it streams from, where it plays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_from: Option<String>,
    /// What it is called, in the words the server holds it under.
    pub title: String,
    /// Why no location is stated, where none is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlocated: Option<String>,
    /// The year it came out, where the server knows one. Absent rather than guessed:
    /// two films share a title far more often than they share a title and a year.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
}

/// Whether a title is a series or a film, as a member's search and ask name it.
///
/// The web surface's own name for [`Kind`], whose name an SDK already gives to the kinds
/// of envelope: the same two words on the wire, under a name nothing else takes.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum TitleKind {
    /// A series.
    #[serde(rename = "tv")]
    Tv,
    /// A film.
    #[serde(rename = "movies")]
    Movies,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One title, as the member it was asked for may see it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TitleReport {
    /// The identifier the media server files them under.
    pub id: String,
    /// The member this was asked for, by the name they are known by.
    pub member: String,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    pub rehearsed: bool,
    /// The title, with where it and each of its episodes is served.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<super::Title>,
}

/// How sure the correlation behind a trace is — a release renamed between services can
/// only be matched fuzzily, and a guess presented as fact is worse than a marked one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum TraceConfidence {
    /// Joined on identifiers the services agree on.
    #[serde(rename = "certain")]
    Certain,
    /// Joined by fuzzy matching; the trace may not be the item asked for.
    #[serde(rename = "uncertain")]
    Uncertain,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One moment in a traced item's history: what happened and when. Where [`TraceStage`]
/// is the linear progress, this is the log a curator kept — the grabs, the failed
/// downloads, the import and any later removal — so a repeated attempt is seen as the
/// pattern it is rather than flattened to a single furthest stage.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraceMoment {
    /// When the service reported it.
    pub at: String,
    /// What happened.
    pub outcome: super::TraceOutcome,
}

/// A notable thing that happened to an item, as an \*arr's history records it. Where the
/// furthest stage answers "how far did it get?", the sequence of outcomes answers "what
/// has been tried?" — a release grabbed more than once, a download that failed and was
/// tried again, a file imported and later removed. Repeated failed grabs are a pattern
/// worth seeing, not something a single furthest-stage reading can show.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum TraceOutcome {
    /// A release was sent to the download client.
    #[serde(rename = "grabbed")]
    Grabbed,
    /// The download client failed the release it was handed.
    #[serde(rename = "download-failed")]
    DownloadFailed,
    /// The item was imported to the library on disk.
    #[serde(rename = "imported")]
    Imported,
    /// The item's file was removed.
    #[serde(rename = "removed")]
    Removed,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where one item is in the pipeline: how far it got, why it stopped if it did, and the
/// stages it passed through — the answer to "where is my show?".
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraceReport {
    /// How sure the trace is of the item it followed.
    pub confidence: super::TraceConfidence,
    /// How much of the item is actually here, season by season — present for an item
    /// made of parts, absent for a film, which is the whole item and has none.
    ///
    /// The furthest stage alone cannot answer this: a series is "imported" the moment one
    /// episode lands, which reads as done while the rest are missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage: Option<super::Coverage>,
    /// Disagreements between the services about this item, each in plain language — a
    /// media server holding what no service is monitoring, and the like. Orthogonal to
    /// the linear pipeline: not where the item got to, but where two services' views of
    /// it contradict, surfaced rather than silently reconciled.
    pub findings: Vec<String>,
    /// The furthest stage the item reached.
    pub furthest: super::Stage,
    /// The notable events in its history, oldest first — the grabs, failed downloads,
    /// imports and removals. Repeated attempts show here as the pattern they are, which
    /// the single furthest stage cannot.
    pub history: Vec<super::TraceMoment>,
    /// The term the item was searched for by.
    pub item: String,
    /// Whether a monitored item matched the term at all — a false here is itself the
    /// answer: nobody asked for it.
    pub matched: bool,
    /// What a search would spend to tell nothing at the quality from nothing at all,
    /// where the trace stopped at `monitored` with that open and a search would spend
    /// something.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable: Option<super::Searchable>,
    /// The stages it passed through, in order.
    pub stages: Vec<super::TraceStage>,
    /// Why it stopped, where it plainly has — or absent where it is progressing or done.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stall: Option<String>,
}

/// One stage a traced item reached, named as the operator would read it: the stage,
/// the service that recorded it, and when.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraceStage {
    /// When it happened, as the service reported it — absent for a stage inferred
    /// rather than timed, such as being monitored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// The service that recorded it.
    pub service: String,
    /// The stage reached.
    pub stage: super::Stage,
}

/// One active download, as the dashboard shows it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Transfer {
    /// The time left, or `None` where it is stalled and there is none to give.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eta: Option<super::Duration>,
    /// What is being downloaded.
    pub name: String,
    /// How far along, as a percentage from zero to a hundred.
    pub progress: u8,
    /// How it is being downloaded.
    pub protocol: super::DashboardProtocol,
    /// The current speed in bytes per second — a [`Reading`], because a genuine
    /// zero (stalled) and a source that has gone quiet mean opposite things here,
    /// and this is the very figure that difference is about.
    pub speed: super::DashboardReading,
}

/// One host tree captured from a setup lemonfiber does not manage.
///
/// Both halves are needed to find it again: the archive path says where it sits
/// inside the archive, and the host path says where it was read from. Nothing
/// derives the second from the first, because a tree outside lemonfiber's layout
/// has no layout to derive it from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tree {
    /// Where it sits inside the archive.
    pub archive_path: String,
    /// Where it was read from, on the machine whose setup was taken over.
    pub host_path: String,
}

/// What became of asking one service to re-search its existing content.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "state")]
#[non_exhaustive]
pub enum Triggered {
    /// The re-search was accepted and now runs in the service's background.
    #[serde(rename = "started")]
    Started,
    /// The service had not finished starting — no key yet — so nothing was asked of
    /// it; running the upgrade again once it is up will reach it.
    #[serde(rename = "not-started")]
    NotStarted,
    /// The service refused the command or could not be reached.
    #[serde(rename = "failed")]
    Failed {
        /// The service's own account of why.
        detail: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// Something somebody reports, and what is likely behind it.
///
/// Keyed by the symptom rather than the cause: the person asking has the symptom,
/// and which cause it is is the thing they cannot yet say.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trouble {
    /// What is likely behind it, most likely first.
    pub causes: Vec<super::Cause>,
    /// What somebody says is happening, in their words.
    pub symptom: String,
}
