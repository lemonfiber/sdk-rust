/// A service held back until the operator gives what it needs to start.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Waiting {
    /// The service.
    pub service: String,
    /// Everything it waits for.
    pub wants: Vec<super::Wanted>,
}

/// What a first-content walkthrough did — the whole of it, narrated line by line as it
/// happened and gathered here so the ending can be rendered, serialised and exited on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WalkthroughReport {
    /// Whether what was asked for was already here, and so was not acquired again.
    pub already_here: bool,
    /// Where it leaves the operator, where it worked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handover: Option<super::Handover>,
    /// Whether the download was handed to the background rather than waited out.
    pub in_background: bool,
    /// What it walked, where it got as far as choosing something.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    /// Every line it said, in order — the same lines the operator watched arrive, kept so
    /// a machine-readable run is not a silent one.
    pub lines: Vec<super::Line>,
    /// What the import did with the file, where it got that far.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<super::Link>,
    /// What it set out to prove, said so the operator knows what they watched.
    pub proves: String,
    /// Which walk this was.
    pub shape: super::Shape,
    /// Where it ended up.
    pub state: super::WalkthroughState,
    /// Where and why it stopped, where it did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopped: Option<super::Stopped>,
    /// What could have been walked instead, where nothing was chosen — the safe first
    /// attempts, so an operator with an empty library is not left guessing.
    pub suggestions: Vec<String>,
}

/// What has become of a walkthrough.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WalkthroughState {
    /// Presented at the end of setup, not yet answered.
    #[serde(rename = "offered")]
    Offered,
    /// Declined. Available later, and carrying no penalty — a stack that was set up is
    /// set up whether or not anyone watched it fetch something.
    #[serde(rename = "skipped")]
    Skipped,
    /// Looking for releases.
    #[serde(rename = "searching")]
    Searching,
    /// Sending a release to the download client.
    #[serde(rename = "grabbing")]
    Grabbing,
    /// The download is running.
    #[serde(rename = "downloading")]
    Downloading,
    /// Moving the finished download into the library.
    #[serde(rename = "importing")]
    Importing,
    /// It is in the library and playable.
    #[serde(rename = "complete")]
    Complete,
    /// Stopped at a named step, with a diagnosis.
    #[serde(rename = "failed")]
    Failed,
    /// The operator left part-way. Whatever was in flight stays in flight.
    #[serde(rename = "abandoned")]
    Abandoned,
}

/// One step of the walk, ordered from picking something to watching it play.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WalkthroughStep {
    /// Picking something to add, and confirming it is not already here.
    #[serde(rename = "choosing")]
    Choosing,
    /// The indexers are being searched for releases.
    #[serde(rename = "searching")]
    Searching,
    /// A release is being sent to the download client.
    #[serde(rename = "grabbing")]
    Grabbing,
    /// The download is running.
    #[serde(rename = "downloading")]
    Downloading,
    /// The curator is moving the finished download into the library.
    #[serde(rename = "importing")]
    Importing,
    /// The media server is being told to look at what arrived.
    #[serde(rename = "scanning")]
    Scanning,
    /// It is in the library and playable.
    #[serde(rename = "available")]
    Available,
}

/// One thing a service waits for, never with its value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum Wanted {
    /// A setting only the operator holds, or one holding a value outside its choices.
    #[serde(rename = "setting")]
    Setting {
        /// The values it accepts, where it declares choices.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        choices: Option<Vec<String>>,
        /// The setting.
        setting: String,
    },
    /// A secret nobody has entered.
    #[serde(rename = "secret")]
    Secret {
        /// The secret, by its name in the secret store.
        secret: String,
    },
    /// The service it runs through, which is held back itself.
    #[serde(rename = "service")]
    Service {
        /// That service.
        service: String,
    },
}

/// A player's report of how far a member got, as the media server now holds it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WatchedReport {
    /// Whether it was recorded as finished.
    pub ended: bool,
    /// The title or episode, by the identifier the shelf lists it under.
    pub id: String,
    /// How far in, in whole seconds, as recorded.
    pub position: u64,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    pub rehearsed: bool,
}

/// What to do when a declared cap is reached.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WhenExceeded {
    /// Stop fetching until the month turns over.
    #[serde(rename = "pause")]
    Pause,
    /// Keep going, slowly, so what is half-finished can finish.
    #[serde(rename = "throttle")]
    Throttle,
    /// Carry on. Some caps cost money and some only cost speed.
    #[serde(rename = "continue")]
    Continue,
}

/// Who settled a contest between claimants.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Whose {
    /// The stack shipped the choice, in the file the operator can read.
    #[serde(rename = "stack")]
    Stack,
    /// The operator chose, and the change is in the journal.
    #[serde(rename = "operator")]
    Operator,
}

/// One of the stack's links, answered.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Wired {
    /// The service the link runs from — what asked.
    pub by: String,
    /// Where the service that asked came from: this build's stack, or a named plugin.
    pub origin: super::ValueOrigin,
    /// What it reaches.
    pub reaches: super::Reaches,
}

/// One connection, and how it turned out.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Wiring {
    /// What was being connected, such as `SABnzbd into Sonarr`.
    pub connection: String,
    /// How serious the outcome is — information by default, a warning where the
    /// connection breaks the stack.
    pub severity: super::SeedSeverity,
    /// How it turned out.
    pub state: super::SeedState,
}

/// An ask several services claim and nothing has chosen between, so it reaches
/// nothing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WiringContest {
    /// The service that asked.
    pub by: String,
    /// What it asked for.
    pub capability: String,
    /// Every claimant, named — a plugin's with the plugin beside it.
    pub claimants: Vec<String>,
}

/// What this stack wires to what.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WiringReport {
    /// Every capability something asks for and nothing fills, naming what asked.
    ///
    /// Repeated out of the links above rather than left to be found among them: a
    /// stack with one unfilled ask among twenty working ones is a stack whose one
    /// problem is a line in a list, and a consumer that had to notice it would be
    /// the reason nobody did.
    pub unfilled: Vec<super::Unfilled>,
    /// Every link, in the order the stack declares them.
    pub wired: Vec<super::Wired>,
}

/// How an ask was settled.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "settled")]
pub enum WiringSettled {
    /// One claimant, and nothing to settle.
    #[serde(rename = "outright")]
    Outright,
    /// Every claimant, because the link asked for all of them rather than one.
    #[serde(rename = "each")]
    Each,
    /// Several claimants and the link asked for one. Refused until somebody chooses:
    /// install order, precedence and recency are each a way of being right most of
    /// the time, and the times they are wrong are somebody's stack answering to the
    /// wrong software.
    #[serde(rename = "contested")]
    Contested {
        /// Every candidate, named, so a choice is made from a list.
        claimants: Vec<String>,
    },
    /// Several claimants, and a choice is recorded.
    #[serde(rename = "chosen")]
    Chosen {
        /// The ones not chosen, so the choice reads as a choice.
        over: Vec<String>,
        /// Who chose.
        whose: super::Whose,
        /// Why, where the chooser said.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        why: Option<String>,
    },
    /// Nothing claims it. What asked is named beside this, which is the point.
    #[serde(rename = "unfilled")]
    Unfilled,
}

/// Where a setup run stands, and what it is still asking for.
///
/// The answer to every step of setup driven from outside this process: a surface
/// asks where the walk is, submits one answer, and is told where the walk is now.
/// Nothing here is a copy of the wizard's own state — it is read off the wizard
/// each time, so a surface cannot hold a stale one and act on it.
///
/// **The answers themselves are never in it.** Setup gathers an indexer key and a
/// provider password, and this report is one a script can log, so it says what was
/// *decided* and never what was *entered* — the same line [`SetupReport`] holds.
/// What will be written is in `plan`, with every credential withheld exactly as
/// `config show` withholds one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WizardReport {
    /// Whether that step asks a question, as opposed to only informing.
    pub asks: bool,
    /// The step the operator is on.
    pub at: super::WizardStep,
    /// What the VPN step asks of the egress guard where one carries the torrents, in the
    /// order it asks it: empty where nothing fills the guard or it asks nothing.
    pub guard: Vec<super::GuardQuestion>,
    /// Whether this machine has setup left to do. False once configuration
    /// exists and nothing is part-way through, which is when a surface directs
    /// the operator to reconfiguration instead of asking the first question again.
    pub offered: bool,
    /// Where this run stands in its lifecycle. `applying` read back here means an
    /// apply stopped part-way, because an apply that is still running is one this
    /// answer is waiting on.
    pub phase: super::Phase,
    /// What applying will write, in the order it will be written, with any value
    /// nobody has argued for showing withheld.
    pub plan: Vec<super::SettingReport>,
    /// What proving the credential just given came to, where one was given.
    ///
    /// Setup tests an indexer key and a Usenet login against their live services as
    /// they are entered, and this is what the service answered — never what was
    /// entered. Absent for every other answer, and for a step that gave none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<super::Validation>,
    /// Whether every applicable question is answered, so the plan can be applied.
    pub ready_for_review: bool,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: super::Ran,
    /// Every question that applies on this machine and has no answer yet, in the
    /// order they are put.
    pub unanswered: Vec<super::WizardStep>,
    /// What an apply that stopped part-way had already written, each said plainly.
    ///
    /// The partial state a recovery is chosen about, so whoever chooses has seen it.
    /// Empty for every other phase, and empty too for an apply that stopped before
    /// it wrote anything.
    pub written: Vec<String>,
}

/// A step of setup, in the order the operator meets it.
///
/// Some steps only inform (they detect and state, and the operator acknowledges);
/// others ask a question whose answer the wizard records. The apply-and-onward
/// steps — writing config, pulling images, wiring services — are not modelled
/// here yet: they arrive with the features they drive, and this machine covers
/// the read-only phase that precedes them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WizardStep {
    /// States what is about to happen and roughly how long it takes. Informs.
    #[serde(rename = "welcome")]
    Welcome,
    /// Detects the environment: OS, Docker, Compose, daemon reachability. Informs.
    #[serde(rename = "preflight")]
    Preflight,
    /// The account checklist derived from the protocol choice. Informs.
    #[serde(rename = "prerequisites")]
    Prerequisites,
    /// Usenet, torrents, both, or neither.
    #[serde(rename = "protocols")]
    Protocols,
    /// Whether a VPN carries the torrent traffic. Asked only where torrents were
    /// chosen, and after the checklist has explained what one is for.
    #[serde(rename = "vpn")]
    Vpn,
    /// Where downloads and the library are kept.
    #[serde(rename = "data-location")]
    DataLocation,
    /// The indexer credential, tested against the live service. Asked only where a
    /// download protocol was chosen.
    #[serde(rename = "credentials")]
    Credentials,
    /// The Usenet provider login, tested over NNTP. Asked only where Usenet was
    /// chosen.
    #[serde(rename = "provider")]
    Provider,
    /// The user and group the containers run as. Asked only where it is visible.
    #[serde(rename = "service-user")]
    ServiceUser,
    /// Whether to run the media server, and if so how.
    #[serde(rename = "library")]
    Library,
    /// Whether others in the home will use it.
    #[serde(rename = "household")]
    Household,
    /// How much the operator wants to be told about — one question, three presets.
    #[serde(rename = "notifications")]
    Notifications,
    /// Whether to start on boot.
    #[serde(rename = "autostart")]
    Autostart,
    /// The complete summary, before anything is written. Informs.
    #[serde(rename = "review")]
    Review,
}
