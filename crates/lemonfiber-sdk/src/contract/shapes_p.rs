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
pub enum Pausing {
    /// Stop every download client fetching.
    #[serde(rename = "pause")]
    Pause,
    /// Let every download client fetch again.
    #[serde(rename = "resume")]
    Resume,
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
pub enum Period {
    /// People are up and using the line, so the stack is held back.
    #[serde(rename = "active")]
    Active,
    /// The house is asleep and the line is the stack's.
    #[serde(rename = "quiet")]
    Quiet,
}

/// Where an in-flight or finished setup stands in its lifecycle.
///
/// The persisted marker a later run reads to tell answers still being gathered
/// from a half-written apply. Only these four are ever written: the two states a
/// run infers instead of storing — no setup at all, and an apply that stopped
/// mid-write — are read off the world rather than trusted from a file (see
/// [`Status`]).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
pub enum PluginDeclaredVerdict {
    /// The assertion fails on the recording, on the constraint the declaration names.
    #[serde(rename = "fails")]
    Fails,
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

/// What an install came to, and what it took to get there.
///
/// **The three lists below are stated whether the run wrote anything or not, and that
/// is the whole of what makes a rehearsal worth running.** A rehearsal that reported
/// less than the real run would be a preview of a different operation; one that
/// reported it from code of its own would be a second derivation free to disagree
/// with the one that acts. So they are filled in one place, from the manifest and from
/// the very list of writes the install is carried out from, and the surface says them
/// in whichever tense `recorded` calls for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginInstall {
    /// What those verdicts were reached against, or nothing where none were reached.
    ///
    /// Carried rather than assumed, because the two kinds of evidence are not the
    /// same claim: an author's read asks the recordings a plugin ships, and an
    /// install asks the service running on this machine. The weaker must not be
    /// readable as the stronger, and a reader handed a verdict has nothing else in
    /// the document to tell them apart.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub against: Option<super::PluginEvidence>,
    /// Every ask its services would make, each with what it would reach and how that
    /// would be settled.
    pub asks: Vec<super::Wired>,
    /// Every change it makes to the machine, in the order it makes them.
    pub changes: Vec<super::PluginChange>,
    /// Every ask of the stack's the install leaves contested that is not contested
    /// now, as it would then stand.
    ///
    /// A plugin's service that claims what the stack asks for is a candidate like any
    /// other, so installing it leaves the ask refused until somebody chooses — which is
    /// a change to what the stack does, and stated with the rest before it happens.
    pub contests: Vec<super::WiringContest>,
    /// Every credential of another service that would cross to the plugin, each approved
    /// apart from the offer as its `approval` names it.
    pub granting: Vec<super::PluginGranting>,
    /// Every bundled thing the plugin declares it will change.
    ///
    /// The full extent rather than a sample of it: a manifest may change a bundled
    /// setting only through a recipe, and a recipe reaching one no `[[override]]`
    /// names is refused before anything is written.
    pub overrides: Vec<super::PluginOverriding>,
    /// Every proof that has to hold before the plugin is installed, and on a run
    /// that asked them, what each came to.
    pub proofs: Vec<super::PluginProving>,
    /// Every install recipe the act ran, in order, with what each step came to; an
    /// empty list where none ran, which is every rehearsal and every plugin declaring no
    /// install recipe.
    ///
    /// A recipe that did not hold ends the act with a problem carrying the same account,
    /// so this is the account of recipes that held.
    pub recipes_ran: Vec<super::PluginRecipeRan>,
    /// Whether it was written down. A rehearsal leaves this false.
    pub recorded: bool,
    /// What putting the install back came to, where something failed and it was.
    ///
    /// The rollback layer's own report rather than a shape of this verb's: what went
    /// back, and what did not with the reason each is still standing. Absent on a run
    /// that had nothing to put back, which is both a rehearsal and an install that
    /// held.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reversed: Option<super::UndoReversal>,
    /// What asking the egress guard among its services to hold its control server to the
    /// credential its adapter was handed came to, where it was more than held at once:
    /// the guard restarted to take it up, or why it did not. Absent where none of its
    /// services is a guard speaking its contract, and on every rehearsal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secured: Option<String>,
    /// Every adapter that writes the files of the service it fronts.
    pub sharing: Vec<super::PluginSharing>,
    /// Every service taking a privileged shape, with what it is given, each approved
    /// apart from the offer.
    pub taking: Vec<super::PluginTaking>,
    /// What the stack's own checks made of the install, or nothing on a run that
    /// asked them nothing.
    ///
    /// The other half of what an install has to establish, and the half a plugin
    /// cannot establish for itself: its proofs say the plugin works, and this says the
    /// stack still does. Absent on a rehearsal, which writes nothing and so has
    /// nothing to hold a reading against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<super::PluginVerification>,
    /// The services held back once the install is made, each with what it waits for.
    pub waiting: Vec<super::Waiting>,
    /// What the install settled, said whether or not it was written down.
    pub would: super::PluginInstalled,
}

/// One plugin's install, as it was decided.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginInstalled {
    /// Every row it adds to a register lemonfiber already runs, as the install settled
    /// them, so the doctor reads them off this record rather than off the plugin's files.
    /// Defaulted for a register written before this field existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributions: Option<Vec<super::Contribution>>,
    /// What the plugin declared about itself: where it is published, whether it was
    /// reviewed, what it claims, what it may change, where it may reach and what it
    /// will hold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declared: Option<super::PluginDeclaration>,
    /// What it does for the operator, in its author's words. Absent alike.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The source it was installed from, as the operator named it.
    ///
    /// Empty for a record written before this was kept. A rehearsal's account carries
    /// the source it was asked about, because that is what it would record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Each credential of another service the operator granted it, as a third-party plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted: Option<Vec<super::PluginGrant>>,
    /// When it was installed, as the record stamps every change: whole seconds since
    /// the epoch.
    ///
    /// The install's own stamp, the one its changes are journalled under, so the
    /// listing and the history name the same moment. Empty where the record predates
    /// it; a rehearsal's account carries the moment it was asked, which is the stamp
    /// the install would have run under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<String>,
    /// The SHA-256 of the manifest it was installed from, in lower-case hexadecimal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    /// What the plugin calls itself, for a person to read.
    ///
    /// Absent on a record written before it was kept, and never filled in from the id:
    /// a name is the author's, and one made up here would be lemonfiber's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The plugin's id: the name it is installed and journalled under.
    pub plugin: String,
    /// Every core capability its services fill, as the install settled them, core names
    /// only, which is what a removal reads to say what would go unfilled. Defaulted for a
    /// register written before the field existed, which reads as filling nothing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provides: Option<Vec<String>>,
    /// Every recipe it declares: each call in order with the adapter it reaches through,
    /// and every value that could leave for somewhere else.
    ///
    /// Empty for a plugin that declares none and for a record written before these were
    /// kept, so a list is always there to read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipes: Option<Vec<super::PluginRecipe>>,
    /// Every proof it declares that asks one of its services a question.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_proofs: Option<Vec<super::PluginRequestProof>>,
    /// The commit it was installed at, where it came from a git source.
    ///
    /// The one commit the revision named at install resolved to, so what was installed
    /// can be told from whatever that source serves now. Empty for a plugin installed
    /// from a directory, which has no revision to name, and for a record written before
    /// this was kept.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// Every proof it declares that runs the image's own program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_proofs: Option<Vec<super::PluginRunProof>>,
    /// What was placed, one entry per service the plugin declares.
    pub services: Vec<super::PluginPlaced>,
    /// What signed it: the key the catalogue index it was resolved through verified
    /// against, named with its fingerprint; empty where nothing signed it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed: Option<String>,
    /// The digest of the plugin's tree it was installed from, `sha256:<hex>`; empty
    /// where the tree could not be read whole, or the record predates it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<String>,
    /// The plugin's own content version, as it stood when it was installed.
    pub version: String,
}

/// What is installed, and what installing one came to.
///
/// One answer for the reading and for the verb, because they are one question: an
/// operator who has just installed something wants to see it among what they had, and
/// a rehearsal that showed only the new entry would not say what it is joining.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginInstalls {
    /// What this run's reading names itself, so an answer to it can say which reading
    /// it answered; nothing on the reading of what is installed, which offers nothing.
    ///
    /// Named part by part, so an answer refused because something moved is told which
    /// part did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement: Option<String>,
    /// What asking for one of a plugin's demand recipes came to, or nothing where none
    /// was asked for.
    ///
    /// Boxed for the reason the install is: it carries a whole recipe, and every other
    /// run's report would otherwise be as large as the one run that asks for one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub demand: Option<super::PluginDemand>,
    /// Each of lemonfiber's own guards this run moved onto its plugin after its own
    /// change, or on a reading would move, for good and with no yes of its own; one it
    /// could not move stays on offer and says why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guards: Option<Vec<super::StackMoveTaken>>,
    /// What this run's install came to, or nothing where it only read.
    ///
    /// Boxed for the reason the update is: it carries a whole account, and every other
    /// run's report would otherwise be as large as the one run that installs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install: Option<super::PluginInstall>,
    /// Every plugin the record holds.
    ///
    /// What it holds, rather than what it would hold: a rehearsal wrote nothing, so
    /// what it settled is in `install` and not here. A listing that counted it
    /// would report an install that did not happen.
    pub installed: Vec<super::PluginInstalled>,
    /// Every answer an installed plugin's adapter gave outside its contract, kept until a
    /// proof it passes clears it: the plugin fills none of those capabilities meanwhile.
    ///
    /// Filled on the reading of what is installed, as `substituted` is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonconforming: Option<Vec<super::PluginNonconforming>>,
    /// What proving a plugin again came to, or nothing where nothing was proved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<super::PluginReproof>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// What this run's removal came to, or nothing where it removed nothing.
    ///
    /// Beside the install rather than in place of it, and never both at once: an
    /// install and a removal are two verbs with two accounts, and a field that held
    /// whichever happened would make a reader ask which one this was before they could
    /// read it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal: Option<super::PluginRemoval>,
    /// Whether each installed plugin's source can still be fetched, asked now.
    ///
    /// Filled on the reading of what is installed and nowhere else, for the reason
    /// `substituted` is: it is the one read an operator makes of what each plugin is
    /// doing, and the one moment this machine asks anybody where a plugin came from. A
    /// run that installs, updates or removes one leaves it empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<super::PluginSource>>,
    /// Every capability the operator chose an installed plugin's service to fill.
    ///
    /// Filled on the reading of what is installed, which is the one read of what each
    /// plugin is doing; a run that installs, updates or removes one leaves it empty,
    /// because none of them changes a choice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substituted: Option<Vec<super::PluginSubstituted>>,
    /// The operator's overlay and every service it changes, where there is one: merged
    /// over the project this run would start, and held to nothing lemonfiber vets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unvetted: Option<super::Unvetted>,
    /// What this run's update came to, or nothing where it updated nothing.
    ///
    /// A third field rather than an install and a removal filled in together, for the
    /// reason those two are apart: an update is one operation with one account.
    ///
    /// Boxed because it carries a whole install's account beside the reversal, and
    /// every other run's report would otherwise be as large as the one run that updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<super::PluginUpdate>,
}

/// One answer a plugin's adapter gave outside a contract it speaks.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginNonconforming {
    /// When it answered so, as RFC 3339.
    pub at: String,
    /// The capability it was asked as.
    pub capability: String,
    /// The operation it was asked.
    pub operation: String,
    /// The plugin whose adapter answered.
    pub plugin: String,
    /// What was outside the contract.
    pub why: String,
}

/// One bundled thing a plugin declares it will change.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginOverriding {
    /// Which bundled setting it changes.
    pub setting: String,
    /// What changing it is for.
    pub why: String,
}

/// One value a recipe could carry to one destination, as the operator agrees to it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginPair {
    /// What approving this pair is written as, on the command line and over the web,
    /// where it carries the value to a host outside the stack or carries a release.
    /// Absent on any other pair to a service in this stack, which takes nothing off the
    /// machine or away from the service it came from, and asks for no approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<String>,
    /// The service in this stack the released value was read from, where the pair
    /// releases it. Absent on every other pair.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Whose value it is, as its input or the step that captures it says; empty where
    /// neither does.
    pub origin: String,
    /// Why the value is carried away from the service it was read from, in the manifest's
    /// own sentence, where the pair releases it. Absent on every other pair.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    /// Where it may be carried, by the name the manifest gives it.
    pub to: String,
    /// What the value is called within the recipe.
    pub value: String,
}

/// One service of an installed plugin, as it was placed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginPlaced {
    /// Every capability it asks for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<super::PluginAsking>>,
    /// What it asks for there, where it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asks_for: Option<String>,
    /// Where inside the container its one configuration directory is mounted.
    ///
    /// Resolved rather than optional. The record answers where the directory is, and
    /// a run that re-derived the fallback would answer for a container it did not
    /// write the day that fallback moved.
    pub config_path: String,
    /// The port inside its container it speaks its contracts on, where it declared one
    /// apart from where it listens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<u16>,
    /// The line its dashboard entry shows beside it: the one its wiring names, or what
    /// the plugin says it does for the operator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The digest that fixes what runs.
    pub digest: String,
    /// Its container fields beyond those above.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<super::ServiceFields>,
    /// The service of the same plugin it stands in front of, as an adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fronts: Option<String>,
    /// The path it answers over HTTP to say it is up, where its health is asked that way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    /// Whether its wiring hid it from the dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// The icon its dashboard entry shows, where its wiring names one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The registry path, carrying no pin of its own.
    pub image: String,
    /// The port it answers on inside the stack's network, where it declared one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listens: Option<u16>,
    /// The media it files, in the stack manifest's vocabulary, which decides what it
    /// comes to in each service that asks for what it provides; none in an older record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_types: Option<Vec<String>>,
    /// What it is called, for a reader, which is what its dashboard entry is listed as;
    /// a record written before this was kept lists it by its id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The API it answers the stack's other services in, where it names one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    /// Networks an older record named for it. Nothing reads them: each service's place on
    /// the networks is derived with the project, and a record written now holds none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
    /// Every core capability this one service fills, which is what makes it a candidate
    /// when the stack asks for one.
    ///
    /// Per service rather than read off the plugin's whole list, because a wiring
    /// reaches a service and not a plugin: of a plugin's two services, the one that
    /// fills a capability is the one an ask for it would reach. Defaulted for a record
    /// written before this was kept, which reads as filling nothing — a service nothing
    /// is wired to, rather than one wired to on a guess.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provides: Option<Vec<String>>,
    /// How it is reached, or nothing where it has no listener.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reached: Option<super::PluginReached>,
    /// Where it reaches outside the machine, where it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaches: Option<String>,
    /// The stack's own service it moves in place of, where it names one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaces: Option<String>,
    /// The service's id, which is the name its container is written under.
    pub service: String,
    /// The privileged shape lemonfiber writes for it, where it took one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<super::PluginShape>,
    /// Each capability contract it answers as an adapter, as `capability@major`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaks: Option<Vec<String>>,
    /// The digest of the starting files kept for its run proofs, where it has any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting: Option<String>,
    /// The readable name that digest went by when it was installed.
    pub tag: String,
    /// Whether the library is mounted for it.
    pub takes_data: bool,
    /// Each field of the `upstream.json` lemonfiber writes for it as an adapter, to the
    /// id of the plugin's secret that field holds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<std::collections::BTreeMap<String, String>>,
}

/// One placeholder a starting file names a secret by, as the plugin ships it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginPlaceholder {
    /// The form its value is written in.
    pub form: super::PluginPlaceholderForm,
    /// The secret it names, by its id.
    pub secret: String,
}

/// The form a placeholder asks its secret to be written in.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PluginPlaceholderForm {
    /// The value itself.
    #[serde(rename = "plain")]
    Plain,
    /// PBKDF2-HMAC-SHA512 of the value under a fresh salt, as a service keeping only a
    /// hash of its password reads it.
    #[serde(rename = "pbkdf2")]
    Pbkdf2,
}

/// One proof that has to hold before a plugin is reported installed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginProving {
    /// What it asks, as the method and the path it is asked at.
    pub asks: String,
    /// What asking it came to, or nothing where it was not asked.
    ///
    /// Absent on a rehearsal, which asks nothing. That is a different fact from a
    /// proof that was asked and established nothing, and the two must not read alike:
    /// one is an account of what would happen, the other is a service that did not
    /// answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub came_to: Option<super::PluginVerdict>,
    /// What it establishes, in one line.
    pub establishes: String,
    /// Which of the plugin's services it asks, where the manifest settles that.
    ///
    /// Nothing where it does not, which is a manifest the reader has already refused
    /// — carried as an absence rather than as a guess, so that a report built from a
    /// manifest nobody held to the reader says *this was not settled* instead of
    /// naming whichever service came first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub of: Option<String>,
    /// The proof's id, which its verdict is reported against.
    pub proof: String,
    /// Why it is worth asserting.
    pub why: String,
}

/// What an install puts at one path.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PluginPuts {
    /// A directory brought into being, which is where one service keeps its own
    /// configuration.
    #[serde(rename = "directory")]
    Directory,
    /// The project written again, holding the container lemonfiber derives for the
    /// plugin.
    #[serde(rename = "document")]
    Document,
    /// The key lemonfiber asks one of the plugin's adapters with, minted as it is
    /// written and never shown.
    #[serde(rename = "key")]
    Key,
    /// A service's own secrets file, readable by its owner alone, its values never shown.
    #[serde(rename = "secrets")]
    Secrets,
    /// A file the plugin ships for one of its services, written where none is yet; one
    /// naming a secret is readable by its owner alone and shown with its placeholders.
    #[serde(rename = "starting")]
    Starting,
    /// The credentials an adapter reads its upstream's with, readable by its owner alone,
    /// their values never shown.
    #[serde(rename = "upstream")]
    Upstream,
}

/// How an installed service is reached, where it is reached at all.
///
/// The tier is the arm, so the label a tier earns lives only in the arm entitled to
/// one. Only a household service is proxied — the bundled policy is that an admin
/// surface does not get a name on the household network — and a record able to carry
/// a loopback service with a hostname would be a record able to describe the thing
/// that policy exists to prevent.
///
/// **The group is on both arms, and that is not an oversight.** Only the proxy is
/// the household tier's alone; the bundled dashboard carries an entry for an
/// operator surface too, with the address it links to rendered from the tier — nine
/// of the shipped stack's own entries point at this machine. A record that kept the
/// group for the wider tier alone would leave a loopback service off the panel its
/// bundled neighbours are on.
///
/// A tier and never an address, either way: lemonfiber renders one from the other
/// exactly as it does for a bundled service, so the two-tier policy stays a property
/// of the system rather than a request a plugin made.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "tier")]
pub enum PluginReached {
    /// From this machine and nowhere else. No route, and no label to route to.
    #[serde(rename = "loopback")]
    Loopback {
        /// The group on the bundled dashboard, where the manifest named one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        group: Option<String>,
        /// The port the service listens on.
        port: u16,
    },
    /// From the household, through the stack's own proxy, at this label.
    #[serde(rename = "household")]
    Household {
        /// The group on the bundled dashboard, where the manifest named one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        group: Option<String>,
        /// The single label in front of the operator's domain.
        hostname: String,
        /// The port the service listens on.
        port: u16,
    },
}

/// One recipe, as an operator agrees to what it does.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRecipe {
    /// The recipe's id within the plugin.
    pub id: String,
    /// Every value it asks the operator for when it runs, in the order it declares them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<super::PluginRecipeInput>>,
    /// When it runs: as part of an install and an update, or only when asked for by name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on: Option<super::PluginRecipeOn>,
    /// Every value it could carry, and where to.
    pub pairs: Vec<super::PluginPair>,
    /// Every call, in the order the recipe makes them.
    pub steps: Vec<super::PluginStep>,
    /// What it accomplishes, in one line.
    pub title: String,
    /// Why it is worth running.
    pub why: String,
}

/// One value a recipe asks the operator for when it runs.
///
/// Never the value: a reading says what will be asked, and what is given for it is
/// handed to the recipe and said back by nothing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRecipeInput {
    /// What the operator is asked for it.
    pub ask: String,
    /// What the input is called, which is what a value is given for it under.
    pub name: String,
    /// Whether the value may be a secret, so a surface takes it without showing it.
    pub secret: bool,
}

/// When a recipe runs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PluginRecipeOn {
    /// During an install and an update, after the plugin's proofs and the stack's checks
    /// hold and before the install is recorded. What a recipe that says nothing does,
    /// because a first-run flow is what a recipe is for.
    #[serde(rename = "install")]
    Install,
    /// Only when the operator asks for it by name, with approvals of its own.
    #[serde(rename = "demand")]
    Demand,
}

/// What one recipe came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRecipeRan {
    /// Whether every step it made came to what it should.
    pub held: bool,
    /// The recipe's id.
    pub recipe: String,
    /// Every step it declares, in order, with what each came to.
    pub steps: Vec<super::PluginStepRan>,
    /// Why it did not, where it did not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// What taking a plugin off the machine came to, or would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRemoval {
    /// Every service that stops when it goes, named before any of them does.
    ///
    /// By service rather than by plugin, because a service is what an operator notices
    /// stopping: a plugin that brought two containers takes two things away, and the
    /// plugin's name alone would not say which of the addresses they use goes quiet.
    /// None of them comes back — a removal is not a restart — which is why this is
    /// stated before the run rather than discovered after it.
    pub interrupts: Vec<String>,
    /// Every capability that would have nothing filling it afterwards.
    ///
    /// Stated before it happens rather than reported after, which is the requirement
    /// and also the only useful order: an operator told afterwards that their requests
    /// no longer reach anything has been informed rather than asked.
    pub leaves: Vec<super::PluginUnfilled>,
    /// The plugin this is about.
    pub plugin: String,
    /// Whether the record of what is installed was written without it.
    ///
    /// False on a rehearsal and on a run that got as far as putting the files back and
    /// no further, which are two different machines and are told apart by what the
    /// reversal says rather than by a second flag here.
    pub removed: bool,
    /// What putting its changes back came to, or would come to.
    ///
    /// The rollback layer's own report rather than a shape of this verb's: a removal is
    /// a reversal with a name on it, and an account of its own would be a second
    /// description of the same work.
    pub went_back: super::UndoReversal,
}

/// What asking an installed plugin's adapters again came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginReproof {
    /// Whether its adapters were asked, rather than only said to be.
    pub asked: bool,
    /// Whether every answer kept against the plugin was cleared, which a proof that did
    /// not hold never does.
    pub cleared: bool,
    /// Every answer kept against the plugin when it was proved, which a pass clears.
    pub kept: Vec<super::PluginNonconforming>,
    /// The plugin proved.
    pub plugin: String,
    /// Each proof asked, or that would be asked, with what it came to.
    pub proofs: Vec<super::PluginProving>,
}

/// What is asked, and where.
///
/// Three fields and no more, and the third is the one worth explaining. A service that
/// answers XML unless a caller asks for JSON cannot satisfy a capability whose probe
/// requires a JSON assertion, and until this field there was nowhere to ask: Plex
/// answers `text/xml` at every path, including the one its health probe uses, unless
/// the request carries `Accept: application/json`.
///
/// **It is one media type and not a header map, and the difference is the point.** A
/// probe declares who it is asked as, and `none` on every `guarded` probe has meant what
/// it says partly because nothing could be presented. A map of headers would make that a
/// convention a reviewer has to hold — any service may name its credential header
/// whatever it likes, so no list of refused names could ever be closed — where one named
/// field keeps it a property of the format. A probe still cannot present anything,
/// because there is nowhere to write it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRequest {
    /// The one representation the answer is asked for, as a media type.
    ///
    /// Absent where the service needs no asking, which is most of them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// The HTTP method.
    pub method: String,
    /// The path on the service being asked.
    pub path: String,
    /// Each place in the answer whose value the service makes afresh on every run, a JSON
    /// Pointer into its JSON, to the RE2 shape the whole value there is held to wherever a
    /// recording is taken again. Never sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub varies: Option<std::collections::BTreeMap<String, String>>,
}

/// One proof that asks one of the plugin's services a question, kept so the doctor can ask
/// it again without the plugin's source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRequestProof {
    /// What the answer must be.
    pub expect: super::Expect,
    /// The proof's id.
    pub proof: String,
    /// What it asks.
    pub request: super::PluginRequest,
    /// The service it asks.
    pub service: String,
    /// What it establishes, in one line.
    pub title: String,
    /// Why it is worth asserting.
    pub why: String,
}

/// One starting file whose placeholders an update changes, as the version installed
/// shipped it and as the one coming ships it: the placeholders alone, never what the
/// file on the machine holds.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginReshipped {
    /// Its placeholders as the version coming ships it, none where it ships none.
    pub becomes: Vec<super::PluginPlaceholder>,
    /// Where the file is, in full.
    pub path: String,
    /// Its placeholders as the version installed shipped it, none where it shipped none.
    pub was: Vec<super::PluginPlaceholder>,
}

/// Where an update did not hold: what putting the version it replaced back came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRestored {
    /// Whether everything its record says it placed is on the machine again.
    pub placed: bool,
    /// Whether its containers are running again.
    ///
    /// Apart from `placed`, because the two fail differently: a document that would not
    /// land is a disk, and a container that would not start is the engine — and an
    /// operator fixes them in different places.
    pub running: bool,
    /// The version put back, which is the one the record still names.
    pub version: String,
}

/// A run of the image's own program, as a proof declares it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRun {
    /// Each argument handed to the image's entrypoint, one string each: one to 64, each at
    /// most 4096 bytes and none holding a NUL.
    pub args: Vec<String>,
    /// How long the run may take, from 1 to 120 seconds. Default 30.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_s: Option<u32>,
}

/// One proof that runs the image's own program, kept so the doctor and a later proof can
/// run it again without the plugin's source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRunProof {
    /// What the run must come to.
    pub expect: super::Expect,
    /// The proof's id.
    pub proof: String,
    /// What the image's own program is handed.
    pub run: super::PluginRun,
    /// The service whose image it runs.
    pub service: String,
    /// What it establishes, in one line.
    pub title: String,
    /// Why it is worth asserting.
    pub why: String,
}

/// One credential a plugin says it will hold, without a value and with no place for one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginSecret {
    /// Whether a recipe of the plugin's captures the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured: Option<bool>,
    /// The capability of the plugin's own that a credential crossing to it is needed to fill.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#for: Option<String>,
    /// What the value is, within the plugin.
    pub id: String,
    /// How many seconds a code the operator enters to claim a part holds, where it is
    /// one: asked for only to claim the part, and held no longer than the claim or this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lapses_s: Option<u32>,
    /// Whether lemonfiber made the value at install.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minted: Option<bool>,
    /// Whether only the operator holds the value and the service reading it needs one to
    /// start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needed_to_start: Option<bool>,
    /// The values of the plugin's settings under which it is asked for and written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs: Option<std::collections::BTreeMap<String, Vec<super::ServiceConstant>>>,
    /// Whose credential it is.
    pub of: String,
    /// The variable an earlier release kept its value under in the stack's environment
    /// file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was: Option<String>,
    /// What holding it is for.
    pub why: String,
}

/// A privileged shape lemonfiber writes for a service.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PluginShape {
    /// `NET_ADMIN` and the tunnel device.
    #[serde(rename = "egress-guard")]
    EgressGuard,
    /// The host's render devices and the group that owns them.
    #[serde(rename = "render")]
    Render,
}

/// An adapter that writes the files of the service it fronts, through that service's
/// whole configuration directory mounted into it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginSharing {
    /// The adapter.
    pub adapter: String,
    /// Where that service's configuration directory lands inside the adapter.
    pub at: String,
    /// The service whose files it writes.
    pub upstream: String,
}

/// Whether one installed plugin's source can still be fetched.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginSource {
    /// The source the record says it came from.
    pub from: String,
    /// The plugin, by its id.
    pub plugin: String,
    /// What asking it came to.
    pub standing: super::PluginSourceStanding,
}

/// What asking a plugin's source came to.
///
/// **Unreachable is not untrusted.** A plugin whose source has gone keeps running as it
/// was installed; what it loses is the way to a newer version, and that is said now
/// rather than found out at the next update.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "standing")]
pub enum PluginSourceStanding {
    /// It answered, so the plugin can be updated from it.
    #[serde(rename = "reachable")]
    Reachable,
    /// It did not answer, or is no longer there, so the plugin cannot be updated from it.
    #[serde(rename = "unreachable")]
    Unreachable {
        /// What asking it said.
        why: String,
    },
    /// Nothing was asked, so nothing is known either way.
    #[serde(rename = "unasked")]
    Unasked {
        /// Why nothing was asked.
        why: String,
    },
}

/// One call a recipe makes, in the order it makes them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginStep {
    /// The step's id within its recipe.
    pub id: String,
    /// The HTTP method it calls with.
    pub method: String,
    /// The API the destination is reached in, as the installed plugin providing it
    /// declares it, or nothing where none does or it is a name outside the stack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    /// The path it calls.
    pub path: String,
    /// Where it calls: a service of this stack's or the plugin's own, or a name outside
    /// both, as the manifest wrote it. Never a resolved address.
    pub to: String,
}

/// What one step came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginStepRan {
    /// What it came to.
    pub came: super::StepCame,
    /// Whether it reached somewhere other than this plugin's own services, which an
    /// install going back cannot undo.
    pub landed: bool,
    /// The status its last answer carried, where anything answered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// The step's id.
    pub step: String,
    /// Where it calls, by the name the manifest gives it.
    pub to: String,
    /// How many times it was made.
    pub tries: u32,
    /// Why it came to what it did, where that wants saying.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// A capability the operator chose one of a plugin's services to fill.
///
/// The only way anything a plugin brought comes to fill what the stack asks for in
/// place of the stack's own: a plugin cannot choose, and wiring by name is not
/// something a plugin can introduce. So what a plugin substituted is what the operator
/// substituted with it, and it is read off the recorded choices rather than off the
/// plugin.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginSubstituted {
    /// The capability it fills.
    pub capability: String,
    /// The plugin whose service it is.
    pub plugin: String,
    /// The service chosen.
    pub service: String,
}

/// One service taking a privileged shape, as the reading states it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginTaking {
    /// What approving it is written as, apart from the offer.
    pub approval: String,
    /// The devices it is given.
    pub devices: Vec<String>,
    /// The kernel capabilities it is given.
    pub grants: Vec<String>,
    /// Whether it is added to the group owning those devices on the engine's host.
    pub grouped: bool,
    /// The service taking it.
    pub service: String,
    /// The shape it takes.
    pub shape: super::PluginShape,
}

/// A capability that would have nothing filling it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginUnfilled {
    /// The core name nothing would fill.
    pub capability: String,
    /// The plugin that is filling it now, which is the one going.
    ///
    /// Named rather than left to the reader, because the sentence an operator has to
    /// act on is *this is the only thing filling it* and a capability on its own does
    /// not say that.
    pub filled_by: String,
}

/// What updating a plugin came to, or would come to, as one account.
///
/// **One account, because it is one operation.** An update is the version installed
/// going back and another coming on, and a report that gave those as a removal and an
/// install side by side would invite reading them as two things that might each have
/// happened. What an operator has to be able to read off this is which version the
/// machine is on, and there are exactly two answers: the new one, where
/// `install.recorded` is true, or the one it replaced, which `restored` says the state
/// of.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginUpdate {
    /// The version the record named before this run.
    pub from: String,
    /// The new version's own account: what it writes, what it has to prove, what it
    /// proved and what the stack's checks made of it — the same one an install gives,
    /// because it is the same work. `recorded` is whether the update holds.
    pub install: super::PluginInstall,
    /// Every service of the installed version that stops, named before any of them
    /// does.
    pub interrupts: Vec<String>,
    /// The plugin this is about.
    pub plugin: String,
    /// Each starting file whose placeholders the new version changes, as the installed
    /// version shipped it and as the new one ships it. The file on the machine is left as
    /// it is, so this is what it was seeded from set against what the new version would
    /// seed it from.
    pub reshipped: Vec<super::PluginReshipped>,
    /// Where the update did not hold, what putting the version it replaced back came
    /// to. Absent on a rehearsal and on an update that held.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored: Option<super::PluginRestored>,
    /// What stopped the new version before its proofs could be asked, where something
    /// did: a write that would not land, a container that would not start, or a record
    /// that could not be written. A proof or a check that did not hold is in `install`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopped: Option<String>,
    /// The version this run installs, or would.
    pub to: String,
    /// What putting the installed version's changes back came to, or would come to.
    ///
    /// Where the plugin's own configuration directory holds what its service wrote, it
    /// is named here as still standing, which on an update is the point: the new
    /// version is started against the same directory, and an update that took it would
    /// be a reinstall that lost everything the old one knew.
    pub went_back: super::UndoReversal,
}

/// What one assertion came to, whatever answered it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "outcome")]
pub enum PluginVerdict {
    /// The recording answers what the binding declares.
    #[serde(rename = "passed")]
    Passed,
    /// It does not, in every way it does not.
    #[serde(rename = "failed")]
    Failed {
        /// Every way the recorded answer is not the declared one.
        faults: Vec<String>,
    },
    /// It could not be run, and so established nothing either way.
    #[serde(rename = "unproven")]
    Unproven {
        /// What stopped it being run.
        why: String,
    },
    /// It fails on the recordings its manifest declares it fails on, on the constraint
    /// each declaration names and on nothing else, and holds on every other recording it
    /// was run against.
    ///
    /// Apart from passed and from failed, and counted as neither. The assertion does not
    /// hold there, so a pass would say the opposite of what the recording shows; and the
    /// failure is the one its author described and gave a reason for, so it does not
    /// fail the run. Only ever reached against recordings: the live service is held to
    /// the expectation as it is written.
    #[serde(rename = "failing-as-declared")]
    FailingAsDeclared {
        /// Each declared recording, what it held, and why it is one the assertion fails on.
        declared: Vec<super::PluginFailingAsDeclared>,
    },
    /// The part an adapter claims is left open, because nobody entered the code its claim
    /// takes: whoever reaches it first can claim it.
    ///
    /// Apart from passed and from failed, and counted as neither: the adapter holds to its
    /// contract, and the part is not yet the household's. Only ever reached against the
    /// live service, by an install's or a re-proof's claim.
    #[serde(rename = "open")]
    Open {
        /// The part, and the code its claim takes.
        why: String,
    },
}

/// What the stack's own checks made of an install.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginVerification {
    /// Every check this install made worse.
    ///
    /// Empty is the answer an install needs, and it is the common one. What is here
    /// is what takes the install back.
    pub broke: Vec<super::PluginChangedCheck>,
    /// Every check nothing could be concluded about across the two readings.
    ///
    /// Reported and never acted on. *I could not tell* is not *it is still broken*,
    /// and an install reversed because a check could not reach a provider it also
    /// could not reach an hour ago would be punishing a plugin for the weather. It is
    /// said out loud rather than dropped, because the assurance an operator thought
    /// they had is the thing that went.
    pub unsettled: Vec<super::PluginChangedCheck>,
}

/// What happens to what a household member asks for.
///
/// Three, and they are the three the request service can actually be put into. A
/// household is in one of them because of two settings taken together — whether requests
/// arrive unseen, and whether a period limits how many — so the words here are a reading
/// of that pair rather than a fourth setting kept beside it.
///
/// Choosing per person is not a fourth policy. It is one of these three chosen for one
/// member rather than for the house, which is why what a surface offers is a policy and,
/// separately, who it is for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Policy {
    /// Everything asked for arrives, and nothing is counted against anybody.
    #[serde(rename = "trusted")]
    Trusted,
    /// Everything asked for arrives until the period's limit is spent, and what
    /// would go past it is refused at the moment of asking.
    #[serde(rename = "within-a-limit")]
    WithinALimit,
    /// Everything asked for waits for the operator to rule on it.
    #[serde(rename = "everything-waits")]
    EverythingWaits,
}

/// One preset in force, and what it means for the media it applies to — the
/// operator's question answered in their own terms, with no scoring vocabulary.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PresetChoice {
    /// What it means, in the operator's terms rather than the tool's.
    pub means: String,
    /// Whether this host would have to transcode it in software — the caution
    /// stated before a choice a household cannot smoothly play.
    pub needs_transcoding_here: bool,
    /// The preset's plain-language name.
    pub preset: String,
    /// The resolution and encode it targets.
    pub resolution: String,
    /// What this applies to: `everything`, or a specific media type.
    pub scope: String,
    /// Roughly how much disk an hour of it takes.
    pub size_per_hour: String,
    /// What playback costs, in plain terms.
    pub transcoding: String,
}

/// What a restore would do, shown before anything is overwritten.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Preview {
    /// What this listing is, so consent given for it can name which listing it read.
    ///
    /// Carried on every listing rather than only on the ones that would re-point
    /// something: a surface that has to look for it is a surface that can fail to
    /// find it, and a restore that would overwrite the same configuration in place
    /// is still one somebody may agree to.
    pub agreement: String,
    /// Whether the archive is old enough that a compatibility warning applies.
    pub downgrade: bool,
    /// The archive's own account of itself — its scope, version and contents.
    pub manifest: super::BackupManifest,
    /// The data-root difference, where the archive was taken against another one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relocation: Option<super::Relocation>,
}

/// Something that went wrong, in the form an operator can act on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Problem {
    /// The problem that produced this one, where several share a root.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<Box<super::Problem>>,
    /// The stable identifier for this kind of problem.
    pub code: super::Code,
    /// The underlying technical detail, available but never leading.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// What it means for the operator.
    pub meaning: String,
    /// What to do, most likely first.
    pub remedies: Vec<super::Remedy>,
    /// How much it matters.
    pub severity: super::ProblemSeverity,
    /// Where it stands with respect to being fixed.
    pub state: super::ProblemState,
    /// Every step a run declares, with what each came to, where the problem ended a run
    /// of steps part-way; absent from every other problem.
    ///
    /// Data beside the detail rather than in it, so a client reads what changed
    /// somewhere going back cannot reach without parsing a sentence written for a person.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<super::ProblemStep>>,
    /// What happened, in one plain sentence.
    pub summary: String,
}

/// How much a problem matters.
///
/// Four levels, deliberately. More would not be applied consistently, and
/// inconsistent severity is worse than coarse severity.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProblemSeverity {
    /// Informational; nothing is required.
    #[serde(rename = "advisory")]
    Advisory,
    /// Degraded or risky, still working.
    #[serde(rename = "warning")]
    Warning,
    /// Something is broken.
    #[serde(rename = "error")]
    Error,
    /// Consequences outside the machine, or data at risk.
    #[serde(rename = "critical")]
    Critical,
}

/// Where a problem stands with respect to being fixed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProblemState {
    /// A remedy is available here.
    #[serde(rename = "actionable")]
    Actionable,
    /// The operator must act, somewhere else.
    #[serde(rename = "guided")]
    Guided,
    /// lemonfiber can fix this itself.
    #[serde(rename = "remediable")]
    Remediable,
    /// No known remedy; escalation is offered instead.
    #[serde(rename = "unknown")]
    Unknown,
    /// Acknowledged, and not re-shown until it recurs.
    #[serde(rename = "suppressed")]
    Suppressed,
}

/// What one step of a run that stopped part-way came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProblemStep {
    /// What it came to.
    pub came: super::StepCame,
    /// Whether it reached somewhere other than the plugin's own services, which going
    /// back cannot undo.
    pub landed: bool,
    /// The recipe the step belongs to.
    pub recipe: String,
    /// The step's id within it.
    pub step: String,
}

/// One consumer, and how far the rotation reached it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Propagation {
    /// What authenticates with the credential.
    pub consumer: String,
    /// How far the rotation reached it.
    pub reach: super::CredentialReach,
}

/// The honest account of what the credential store protects against.
///
/// Two lists and a sentence, carried as data rather than printed here, so the API
/// serves the same words the terminal prints and neither can drift into a claim the
/// other does not make.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Protection {
    /// What it protects against.
    pub against: Vec<String>,
    /// What it does not protect against.
    pub not_against: Vec<String>,
    /// What the storage is, before any claim about it.
    pub summary: String,
}

/// Which download protocols the operator actually has accounts for.
///
/// A form names both, because a form describes what it *does* rather than what
/// this operator has paid for. Narrowing happens afterwards, so a tunnel is
/// never started with credentials that were never supplied.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Protocols {
    /// A VPN and torrent client are configured.
    pub torrent: bool,
    /// A Usenet provider is configured.
    pub usenet: bool,
}

/// Where every service in this stack comes from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProvenanceReport {
    /// The services, in the order the stack declares them.
    ///
    /// Every service the manifest holds rather than the ones some form would start:
    /// what is *in* this stack is the question being asked, and an answer narrowed to
    /// what is running would leave the operator unable to ask about the service they
    /// are deciding whether to run.
    pub services: Vec<super::ServiceProvenance>,
}

/// A Usenet provider login the operator supplied, and whether it was proven.
///
/// Its password rides here the way the stack holds its other secrets; `validated`
/// records whether the live login took before it was kept, so a later diagnosis
/// knows an unproven one went in unverified.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Provider {
    /// The provider's hostname.
    pub host: String,
    /// The account password.
    pub pass: String,
    /// The port it answers NNTP on.
    pub port: u16,
    /// Whether to connect over TLS, as it must be to carry the password.
    pub tls: bool,
    /// The account username.
    pub user: String,
    /// Whether a live login proved the account before it was kept.
    ///
    /// Defaulted when it is absent, for the same reason the indexer's is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validated: Option<bool>,
}

/// Whether a client is fetching at all, where a cap made it a question.
///
/// Apart from the limits above rather than folded in with them, because it answers
/// a different question and a client held to a crawl is not a client that stopped.
/// A word that named both would be the vocabulary this whole path exists to avoid.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Pulling {
    /// It is fetching, or would start on the next thing handed to it.
    #[serde(rename = "fetching")]
    Fetching,
    /// Nothing is moving and nothing new would start.
    #[serde(rename = "stopped")]
    Stopped,
}
