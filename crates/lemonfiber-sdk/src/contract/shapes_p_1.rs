/// What a capture came to, against the time a capture is meant to take.
///
/// Reported and never enforced. The room check already walks the trees to decide
/// whether the archive fits, so the bytes are in hand before anything is written and
/// cost nothing extra to say — and what they are measured against is the work, not a
/// clock. A wall-clock gate on a machine whose disk throughput varies by more than the
/// margin either passes for reasons unrelated to this product or fails for them, and
/// neither reading is worth having.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pace {
    /// Whether this capture is inside it.
    pub brisk: bool,
    /// The bytes a capture may move and still be expected to finish in time.
    ///
    /// Carried with the reading rather than left for a reader to look up, so a surface
    /// showing this does not need a second copy of the number to compare against.
    pub budget: u64,
    /// The bytes the captured trees came to, as the room check measured them.
    pub moved: u64,
}

/// Pairing material, and what the operator is told beside it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pairing {
    /// What is worth knowing about the address itself, where anything is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caution: Option<String>,
    /// The fingerprint in the short form a person compares with what the phone shows
    /// after typing the line in, as [`comparable`] derives it.
    pub compare: String,
    /// The material itself.
    pub material: super::PairingMaterial,
    /// What would make every paired phone refuse this machine, said now rather than
    /// discovered then. In words any surface can show: how the certificate is replaced
    /// is each surface's own to say, so this names no command.
    pub replacing: String,
    /// When it stops being good, as a date and a time of day.
    pub until: String,
    /// The material as the one line a code carries and a person types.
    pub written: String,
}

/// What a phone is handed, exactly as it reads it.
///
/// Four fields and no more: a reader refuses one it does not know, which is what keeps a
/// credential from ever riding along under a name nobody thought to forbid.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PairingMaterial {
    /// Where the phone reaches the stack: an `https` address on the household network.
    pub address: String,
    /// When the material stops being good, in seconds since the Unix epoch.
    pub expires: u64,
    /// The certificate that address presents: SHA-256 over its DER encoding, in
    /// lower-case hex. Not the digest of its public key.
    pub fingerprint: String,
    /// The stack's own identifier: opaque, minted once from nothing and kept, and the
    /// same across every issue of the material, a change of address and a replacement of
    /// the certificate. Not the stack's name and not its version, and it carries nothing
    /// about the household or anybody in it.
    pub stack: String,
}

/// A panel's content, or the reason its source could not fill it.
///
/// The difference between "this panel is up to date" and "this panel's source is
/// unreachable" is the whole of degrading honestly: an unavailable panel says so,
/// in its own words, rather than showing stale data as current or blank data as
/// zero — and the panels beside it stay live.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "panel")]
#[non_exhaustive]
pub enum PanelArrayOfDownloader {
    /// The source answered; here is the panel.
    #[serde(rename = "ready")]
    Ready {
        /// The `data` field.
        data: Vec<super::Downloader>,
    },
    /// The source could not be reached, for this stated reason.
    #[serde(rename = "unavailable")]
    Unavailable {
        /// The `data` field.
        data: super::PanelArrayOfDownloaderUnavailableData,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `PanelArrayOfDownloaderUnavailableData` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanelArrayOfDownloaderUnavailableData {
    /// Why the panel could not be filled, in the operator's terms.
    pub reason: String,
}

/// A panel's content, or the reason its source could not fill it.
///
/// The difference between "this panel is up to date" and "this panel's source is
/// unreachable" is the whole of degrading honestly: an unavailable panel says so,
/// in its own words, rather than showing stale data as current or blank data as
/// zero — and the panels beside it stay live.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "panel")]
#[non_exhaustive]
pub enum PanelArrayOfQueue {
    /// The source answered; here is the panel.
    #[serde(rename = "ready")]
    Ready {
        /// The `data` field.
        data: Vec<super::Queue>,
    },
    /// The source could not be reached, for this stated reason.
    #[serde(rename = "unavailable")]
    Unavailable {
        /// The `data` field.
        data: super::PanelArrayOfQueueUnavailableData,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `PanelArrayOfQueueUnavailableData` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanelArrayOfQueueUnavailableData {
    /// Why the panel could not be filled, in the operator's terms.
    pub reason: String,
}

/// A panel's content, or the reason its source could not fill it.
///
/// The difference between "this panel is up to date" and "this panel's source is
/// unreachable" is the whole of degrading honestly: an unavailable panel says so,
/// in its own words, rather than showing stale data as current or blank data as
/// zero — and the panels beside it stay live.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "panel")]
#[non_exhaustive]
pub enum PanelArrayOfService {
    /// The source answered; here is the panel.
    #[serde(rename = "ready")]
    Ready {
        /// The `data` field.
        data: Vec<super::Service>,
    },
    /// The source could not be reached, for this stated reason.
    #[serde(rename = "unavailable")]
    Unavailable {
        /// The `data` field.
        data: super::PanelArrayOfServiceUnavailableData,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `PanelArrayOfServiceUnavailableData` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanelArrayOfServiceUnavailableData {
    /// Why the panel could not be filled, in the operator's terms.
    pub reason: String,
}

/// A panel's content, or the reason its source could not fill it.
///
/// The difference between "this panel is up to date" and "this panel's source is
/// unreachable" is the whole of degrading honestly: an unavailable panel says so,
/// in its own words, rather than showing stale data as current or blank data as
/// zero — and the panels beside it stay live.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "panel")]
#[non_exhaustive]
pub enum PanelArrayOfTransfer {
    /// The source answered; here is the panel.
    #[serde(rename = "ready")]
    Ready {
        /// The `data` field.
        data: Vec<super::Transfer>,
    },
    /// The source could not be reached, for this stated reason.
    #[serde(rename = "unavailable")]
    Unavailable {
        /// The `data` field.
        data: super::PanelArrayOfTransferUnavailableData,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `PanelArrayOfTransferUnavailableData` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanelArrayOfTransferUnavailableData {
    /// Why the panel could not be filled, in the operator's terms.
    pub reason: String,
}

/// A panel's content, or the reason its source could not fill it.
///
/// The difference between "this panel is up to date" and "this panel's source is
/// unreachable" is the whole of degrading honestly: an unavailable panel says so,
/// in its own words, rather than showing stale data as current or blank data as
/// zero — and the panels beside it stay live.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "panel")]
#[non_exhaustive]
pub enum PanelFrontDoorReport {
    /// The source answered; here is the panel.
    #[serde(rename = "ready")]
    Ready {
        /// The `data` field.
        data: super::FrontDoorReport,
    },
    /// The source could not be reached, for this stated reason.
    #[serde(rename = "unavailable")]
    Unavailable {
        /// The `data` field.
        data: super::PanelFrontDoorReportUnavailableData,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `PanelFrontDoorReportUnavailableData` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanelFrontDoorReportUnavailableData {
    /// Why the panel could not be filled, in the operator's terms.
    pub reason: String,
}

/// A panel's content, or the reason its source could not fill it.
///
/// The difference between "this panel is up to date" and "this panel's source is
/// unreachable" is the whole of degrading honestly: an unavailable panel says so,
/// in its own words, rather than showing stale data as current or blank data as
/// zero — and the panels beside it stay live.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "panel")]
#[non_exhaustive]
pub enum PanelHouseholdReport {
    /// The source answered; here is the panel.
    #[serde(rename = "ready")]
    Ready {
        /// The `data` field.
        data: super::HouseholdReport,
    },
    /// The source could not be reached, for this stated reason.
    #[serde(rename = "unavailable")]
    Unavailable {
        /// The `data` field.
        data: super::PanelHouseholdReportUnavailableData,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `PanelHouseholdReportUnavailableData` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanelHouseholdReportUnavailableData {
    /// Why the panel could not be filled, in the operator's terms.
    pub reason: String,
}

/// A panel's content, or the reason its source could not fill it.
///
/// The difference between "this panel is up to date" and "this panel's source is
/// unreachable" is the whole of degrading honestly: an unavailable panel says so,
/// in its own words, rather than showing stale data as current or blank data as
/// zero — and the panels beside it stay live.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "panel")]
#[non_exhaustive]
pub enum PanelStorage {
    /// The source answered; here is the panel.
    #[serde(rename = "ready")]
    Ready {
        /// The `data` field.
        data: super::Storage,
    },
    /// The source could not be reached, for this stated reason.
    #[serde(rename = "unavailable")]
    Unavailable {
        /// The `data` field.
        data: super::PanelStorageUnavailableData,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `PanelStorageUnavailableData` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanelStorageUnavailableData {
    /// Why the panel could not be filled, in the operator's terms.
    pub reason: String,
}

/// A panel's content, or the reason its source could not fill it.
///
/// The difference between "this panel is up to date" and "this panel's source is
/// unreachable" is the whole of degrading honestly: an unavailable panel says so,
/// in its own words, rather than showing stale data as current or blank data as
/// zero — and the panels beside it stay live.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "panel")]
#[non_exhaustive]
pub enum PanelVpn {
    /// The source answered; here is the panel.
    #[serde(rename = "ready")]
    Ready {
        /// The `data` field.
        data: super::Vpn,
    },
    /// The source could not be reached, for this stated reason.
    #[serde(rename = "unavailable")]
    Unavailable {
        /// The `data` field.
        data: super::PanelVpnUnavailableData,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `PanelVpnUnavailableData` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanelVpnUnavailableData {
    /// Why the panel could not be filled, in the operator's terms.
    pub reason: String,
}

/// One part of a traced item — an episode of a series. A film has no parts: the item is
/// the whole, and a trace of it says all there is to say. A series does not, which is the
/// gap this closes: "the show is imported" is true the moment one episode lands, and reads
/// as done while nine are still missing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Part {
    /// Its number within that season.
    pub number: u32,
    /// Which season it belongs to.
    pub season: u32,
    /// How far this one part got, on the same scale as the item as a whole.
    pub stage: super::Stage,
    /// Its title, as a person would name it.
    pub title: String,
}

/// Something a member was part-way through, and how far.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartWay {
    /// Where its backdrop is served, where it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backdrop: Option<String>,
    /// The certificate the door presents, which a client pins, beside any location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub door: Option<super::Pinned>,
    /// The identifier the server tells it apart by, which is what asking to play one
    /// of them names.
    pub id: String,
    /// How long it runs, in whole seconds, where the server knows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    /// Which of the kinds this product deals in it is.
    pub medium: super::Medium,
    /// How far in they got, in whole seconds.
    pub position: u64,
    /// Where its poster is served, where it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster: Option<String>,
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

/// What one member was part-way through, most recent first.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartWayReport {
    /// Whether it could be read at all. An empty list and an unread one are different
    /// answers, and what could not be read is said in `findings`.
    pub available: bool,
    /// What is worth saying about this read, in the words its reader would use.
    pub findings: Vec<String>,
    /// The identifier the media server files them under.
    pub id: String,
    /// The member this was asked for, by the name they are known by.
    pub member: String,
    /// Each title or episode, how far in, and where it is served.
    pub part_way: Vec<super::PartWay>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    pub rehearsed: bool,
}

/// Where a refusal's words were carried, and when.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Passed {
    /// When the attempt was made, absent where the clock could not be written down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// The services they reached, by the names the member knows them by. Empty where
    /// there was nowhere this could send.
    pub to: Vec<String>,
}

/// What one download client said about being paused or resumed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PausedClient {
    /// The client, by the name the stack knows it under.
    pub client: String,
    /// What it read back after it was asked. Absent on a rehearsal, which asks nothing,
    /// and where the client could not be reached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub now: Option<super::Pulling>,
    /// Why it could not be reached, in its own words where it gave any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unreached: Option<String>,
    /// Whether it was fetching before it was asked, where it said.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was: Option<super::Pulling>,
}

/// Which of the two was asked for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Pausing {
    /// Stop every download client fetching.
    #[serde(rename = "pause")]
    Pause,
    /// Let every download client fetch again.
    #[serde(rename = "resume")]
    Resume,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What pausing or resuming every download client came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PausingReport {
    /// Which of the two was asked for.
    pub asked: super::Pausing,
    /// What a resume runs into where a spent cap had stopped the clients: they are let
    /// go as asked, and the cap stops them again the next time the line is checked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caution: Option<String>,
    /// Every download client the stack runs, in the order the stack declares them.
    pub clients: Vec<super::PausedClient>,
    /// The offer this answers: every client and what it said it was doing before it was
    /// asked anything, named so that a request carrying it back acts on those clients
    /// in those states or is refused.
    pub offer: String,
    /// Whether this was a rehearsal: what each client is doing now, with nothing asked
    /// of any of them.
    pub rehearsed: bool,
}

/// Which side of the household's day a moment falls on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Period {
    /// People are up and using the line, so the stack is held back.
    #[serde(rename = "active")]
    Active,
    /// The house is asleep and the line is the stack's.
    #[serde(rename = "quiet")]
    Quiet,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where an in-flight or finished setup stands in its lifecycle.
///
/// The persisted marker a later run reads to tell answers still being gathered
/// from a half-written apply. Only these four are ever written: the two states a
/// run infers instead of storing — no setup at all, and an apply that stopped
/// mid-write — are read off the world rather than trusted from a file (see
/// [`Status`]).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Phase {
    /// Still gathering answers. Resumable, and nothing has touched disk.
    #[serde(rename = "in-progress")]
    InProgress,
    /// Every applicable question answered, awaiting the operator's confirmation.
    #[serde(rename = "reviewing")]
    Reviewing,
    /// Writing configuration and starting services — the one non-atomic phase,
    /// and so the only marker whose persistence signals an interrupted run.
    #[serde(rename = "applying")]
    Applying,
    /// Configuration written and valid.
    #[serde(rename = "applied")]
    Applied,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One file inside a bundle: the name it will carry, and what it holds.
///
/// Held in memory rather than written as it is gathered, because everything is read back
/// before anything is written. A bundle that had already put one file on disk when it
/// found a credential in the next would have to be unwritten, and unwriting is the kind of
/// thing that half-works.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Piece {
    /// What it holds, already redacted.
    pub body: String,
    /// What it is called inside the bundle.
    pub name: String,
}

/// The certificate a guarded door presents, as a client pins it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pinned {
    /// SHA-256 over the certificate's DER encoding, lower-case hex.
    pub fingerprint: String,
}

/// What will be run, and what was left out.
///
/// Serialisable because it is an answer in its own right: asking what a form
/// would do is a question a script asks as readily as a person, and the plan a
/// lifecycle report carries is this same value rather than a retelling of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Plan {
    /// Profiles the closure asked for that the configuration does not support.
    pub dropped: Vec<super::Dropped>,
    /// The services those profiles hold, each with what it needed and who asked.
    ///
    /// The same answer as [`Self::dropped`], a service at a time. A surface showing what
    /// did not start shows services, and one holding its own copy of which service sits
    /// in which profile would be a second copy of the stack's vocabulary.
    pub filtered: Vec<super::Filtered>,
    /// What the stack estimates the services that would start need.
    pub footprint: super::Footprint,
    /// The forms the operator named, in the order they named them.
    pub forms: Vec<String>,
    /// The profiles to activate, sorted so the command is reproducible.
    pub profiles: Vec<String>,
    /// Which of [`Self::services`] are already running, where a form's introspection
    /// asked the engine.
    ///
    /// Starting a form leaves what is already running as it is, so a running service
    /// is not one the start would bring up. Absent where nothing asked, which is every
    /// plan but a preview's; `null` where the engine would not say, which is never said
    /// as nothing running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<Vec<String>>,
    /// The services those profiles start, in the order the stack declares them.
    ///
    /// A service belongs to exactly one profile, so a service two named forms
    /// both reach is here once. That is a property of the manifest rather than
    /// of a pass over this list: the union is over profiles, and a service
    /// appearing twice is not a state this can hold.
    pub services: Vec<String>,
}

/// Somebody watching something now, as the media server lists the session.
///
/// Who and what, and where: what a household recognises about somebody watching. No
/// stream, no bitrate and no transcode reason, because a member is not choosing one and
/// a surface handed those would have to decide not to draw them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Playback {
    /// What the device it plays on calls itself.
    pub device: String,
    /// The episode's number within its season, where the server numbers one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episode: Option<u32>,
    /// Which of the kinds this product deals in it is. An episode is of a series.
    pub medium: super::Medium,
    /// The name the account is known by.
    pub member: String,
    /// The identifier the server files the account under.
    pub member_id: String,
    /// Whether it is paused rather than playing.
    pub paused: bool,
    /// The season an episode is in, where the server numbers one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
    /// The series an episode belongs to, where it is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    /// What is playing, in the words the server holds it under: an episode's own name
    /// where it is an episode.
    pub title: String,
}

/// What is playing now, and whose sessions were asked about.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayingReport {
    /// Whether the media server could be asked at all.
    ///
    /// Nobody watching and a server that would not say are different answers, and
    /// collapsing them would report a quiet house on the day the media server was down.
    /// Anything that could not be read is said in `findings` and this goes false.
    pub available: bool,
    /// What is worth saying about this reading, in the words its reader would use.
    ///
    /// Always written, empty or not, so a reader never has to guess what an absent
    /// field means.
    pub findings: Vec<String>,
    /// The member this was narrowed to, by the name they are known by, or empty where
    /// it is every session in the house.
    pub member: String,
    /// Every session playing something, in the order the media server lists them. How
    /// many are playing is how many these are, rather than a count kept beside them.
    pub sessions: Vec<super::Playback>,
}

/// One capability a placed service asks for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginAsking {
    /// The core capability asked for.
    pub capability: String,
    /// Whether every service that fills it is reached rather than one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub each: Option<bool>,
    /// The medium the filler it reaches files, where it narrows the ask to one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    /// The upstream's own API it talks in, or every API it talks in any of, which it
    /// reaches in place of the adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<super::Apis>,
}

/// One change installing a plugin makes to the machine.
///
/// A path and what goes at it, which is the whole of what an install touches. None is an
/// edit to a file the stack already has: a plugin's route is the proxy's routing table
/// and its dashboard entry the panel list's, each applied over its contract.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginChange {
    /// Where it lands, in full.
    ///
    /// In full rather than relative to the stack, because *what is this about to do
    /// to my machine* is answered by a path somebody can go and look at — and a
    /// relative one is right about a directory the reader has to work out for
    /// themselves.
    pub path: String,
    /// Each placeholder the file as shipped names a secret by, in the order it holds them:
    /// none for anything but a starting file naming a secret, and never a value.
    pub placeholders: Vec<super::PluginPlaceholder>,
    /// What lands there.
    pub puts: super::PluginPuts,
}

/// One check that stands differently after an install than it did before.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginChangedCheck {
    /// How the same check read before the install, or nothing where it was not
    /// raised at all.
    ///
    /// Absent means the check produced no finding beforehand, which is read as it
    /// holding: a finding no longer raised is a fault no longer there, and the same
    /// rule read backwards is that one not yet raised was not yet a fault.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<super::DoctorVerdict>,
    /// The check as it reads now, with everything the diagnosis says about it.
    ///
    /// The finding itself rather than a summary of it, because what an operator does
    /// next is read the remedy, the service's own output and what else is causing it
    /// — and a second, thinner shape of the same fact is where those stop arriving.
    pub now: super::Finding,
}

/// A kind of constraint an expectation can put on a body.
///
/// The same vocabulary a proof's expectation and a contributed check's use, so that
/// "a status alone is not evidence" is one rule rather than three. Read as well as
/// written: a declaration that an assertion fails on a recording names the one of these
/// that fails there.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum PluginConstraint {
    /// The status alone.
    #[serde(rename = "status")]
    Status,
    /// Keys carrying exactly these values.
    #[serde(rename = "json")]
    Json,
    /// Keys that must be present, whatever they hold.
    #[serde(rename = "json_has_keys")]
    JsonHasKeys,
    /// Keys that must hold a given kind of thing.
    #[serde(rename = "json_types")]
    JsonTypes,
    /// Keys that must hold at least a given number.
    #[serde(rename = "json_at_least")]
    JsonAtLeast,
    /// The answer read as an array, with at least a given number of entries.
    #[serde(rename = "json_array_min")]
    JsonArrayMin,
    /// The answer did not parse as JSON at all.
    #[serde(rename = "json_is_absent")]
    JsonIsAbsent,
    /// The content type it is served as.
    #[serde(rename = "content_type")]
    ContentType,
    /// What the body must begin with.
    #[serde(rename = "body_starts_with")]
    BodyStartsWith,
    /// The exit status a run ends with.
    #[serde(rename = "exit")]
    Exit,
    /// What a run's standard output begins with.
    #[serde(rename = "stdout_starts_with")]
    StdoutStartsWith,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What a plugin declared about itself, as its install read it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginDeclaration {
    /// Every capability it claims, core and its own, in the order it declares them.
    ///
    /// Apart from what its services fill: a claim is what it says it can do and has to
    /// demonstrate, and a capability of its own is claimed without anything asking for
    /// it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<String>>,
    /// The licence it is distributed under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Every bundled setting it declares it may change.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<super::PluginOverriding>>,
    /// Every destination a recipe of its could reach that is not one of its own
    /// services: a service of this stack's, or a name outside it.
    ///
    /// Recorded as declared rather than sorted into the two here, because which names
    /// are this stack's is a question about the stack, and the stack a record is read
    /// against is the one on the machine when it is read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaches: Option<Vec<String>>,
    /// Whether anybody reviewed it before it was installed.
    ///
    /// True for a plugin installed by name through a catalogue index whose signature
    /// verified, and false for every install from a source an operator named. Carried
    /// rather than left implicit, because an unreviewed plugin is to be said to be one
    /// for as long as it is installed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed: Option<bool>,
    /// Every credential it says it will hold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<super::PluginSecret>>,
    /// Where its source is published, as the plugin names it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
}

/// The one verdict a declaration may name.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum PluginDeclaredVerdict {
    /// The assertion fails on the recording, on the constraint the declaration names.
    #[serde(rename = "fails")]
    Fails,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What asking for one of an installed plugin's demand recipes came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginDemand {
    /// The plugin whose recipe it is.
    pub plugin: String,
    /// What running it came to, or nothing where it was only read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ran: Option<super::PluginRecipeRan>,
    /// The recipe as the operator agrees to it: its calls, what it asks for, and every
    /// value it would carry where.
    pub recipe: super::PluginRecipe,
}

/// What the verdicts in a report were reached against.
///
/// One value, because one is all this build can produce: nothing here asks a service
/// anything. It is a field rather than a sentence for the reason a verdict is one — a
/// reader handed `demonstrated` has nothing else in the document to tell a recording
/// that answered from a service that did, and the weaker of those two claims must not
/// be readable as the stronger. The prose says it on the page; this says it to
/// whatever consumes the report: an author's own CI, a catalogue, anything counting
/// passes. Naming the axis now is what made the second kind of evidence a change the
/// compiler walked somebody through rather than one they had to remember.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum PluginEvidence {
    /// The recordings the plugin ships. No service was asked anything.
    #[serde(rename = "recordings")]
    Recordings,
    /// The service itself, running on this machine and asked.
    ///
    /// The stronger of the two and the one only an install can reach: an author has
    /// no instance to ask and the catalogue has none either, which is the whole
    /// reason recordings exist. A verdict reached here says the service did the
    /// thing on the machine it is installed on, and it must not be readable as the
    /// weaker claim any more than the weaker may be read as this.
    #[serde(rename = "service")]
    Service,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A recording an assertion fails on, the one constraint of its expectation that fails
/// there, and why.
///
/// For an assertion whose passing state nobody can record, such as a check that a server
/// has an owner where claiming one needs an account the plugin's CI does not hold: the
/// state it exists to find can be recorded, and the declaration says which constraint
/// tells the two states apart. It changes the verdict on that recording and on nothing
/// else — the live service and every other recording are held to the expectation as it
/// is written.
///
/// It names a constraint rather than only a recording, because a recording that began
/// failing for another reason — a truncated file, an error recorded by mistake — would
/// otherwise read as failing as declared, excusing a failure nobody had looked at.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginExpectedFailure {
    /// The key of the expectation that fails on the recording, and one it carries.
    pub constraint: super::PluginConstraint,
    /// The recording the assertion fails on. It may be the assertion's own `fixture`.
    pub fixture: String,
    /// Where within that constraint it fails, written exactly as the expectation writes
    /// it. Present for a key-wise constraint and absent for one about the whole answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    /// Why this recording is one the assertion fails on. Reported with the verdict every
    /// time.
    pub reason: String,
    /// `fails`, and nothing else: passing is what the expectation already says, and
    /// could-not-run is never excused.
    pub verdict: super::PluginDeclaredVerdict,
}

/// One recording an assertion fails on as its manifest declares.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginFailingAsDeclared {
    /// The constraint of the expectation that fails there.
    pub constraint: super::PluginConstraint,
    /// The recording.
    pub fixture: String,
    /// What the recording held there.
    pub held: String,
    /// Where within that constraint, for one that looks at places.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    /// Why the manifest says this recording is one the assertion fails on.
    pub reason: String,
}

/// A credential of another service the operator granted a third-party plugin, named by
/// the setting it is held under and never by its value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginGrant {
    /// When the operator approved it, as the record stamps every change.
    pub at: String,
    /// The setting the credential is held under, as the credentials read names it.
    pub credential: String,
    /// The capability of the plugin's own it was granted to fill.
    pub r#for: String,
    /// The service whose credential it is.
    pub of: String,
}

/// One credential that would cross to a plugin, as the reading states it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginGranting {
    /// What approving it is written as, apart from the offer.
    pub approval: String,
    /// The setting the credential is held under, as the credentials read names it.
    pub credential: String,
    /// The capability of the plugin's own it would be granted to fill.
    pub r#for: String,
    /// The service whose credential it is.
    pub of: String,
}
