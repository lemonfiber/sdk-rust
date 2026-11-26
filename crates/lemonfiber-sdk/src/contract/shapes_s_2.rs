/// Whether this stack is set up.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum SetUp {
    /// Nothing is configured and no setup has begun.
    #[serde(rename = "none")]
    None,
    /// A setup has begun and not finished: answers are being gathered, are waiting for
    /// review, or an apply stopped part-way.
    #[serde(rename = "part-way")]
    PartWay,
    /// The stack is configured and no setup is part-way through.
    #[serde(rename = "done")]
    Done,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A change to one or more settings, made in one act.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettingChange {
    /// What this reading names itself, part by part.
    pub agreement: String,
    /// Whether it was written.
    pub applied: bool,
    /// Every service whose entry the change regenerates.
    pub regenerates: Vec<String>,
    /// Every running service the change restarts.
    pub restarts: Vec<String>,
    /// Each setting the act moves, in name order.
    pub settings: Vec<super::SettingMove>,
    /// Every service held back that the change starts, having given the last thing it
    /// waited for.
    pub starts: Vec<String>,
    /// Why the services it restarts were not restarted, where they were not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unrestarted: Option<String>,
    /// The services held back once the change is made, each with what it waits for.
    pub waiting: Vec<super::Waiting>,
}

/// One value a setting accepts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SettingChoice {
    /// A value that is always a choice.
    ServiceConstant(Box<super::ServiceConstant>),
    /// Values that are choices only where `needs` is met.
    SettingChoiceNeeding(Box<super::SettingChoiceNeeding>),
    /// A value this client was not generated with, as it arrived.
    Unrecognised(serde_json::Value),
}

/// Values that are choices only where the other settings `needs` names hold one of the
/// values it lists.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettingChoiceNeeding {
    /// What they are choices under.
    pub needs: std::collections::BTreeMap<String, Vec<super::ServiceConstant>>,
    /// The values.
    pub values: Vec<super::ServiceConstant>,
}

/// What a setting falls back to while it holds no operator value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "of")]
#[non_exhaustive]
pub enum SettingDefault {
    /// One of the core's own settings, with what the core reads while none is recorded.
    #[serde(rename = "core")]
    Core {
        /// The value.
        value: String,
    },
    /// The one default every reader whose `needs` are met declares. Empty is empty by
    /// default.
    #[serde(rename = "declared")]
    Declared {
        /// The value.
        value: String,
    },
    /// Each reader's own, where the readers whose `needs` are met declare different ones.
    #[serde(rename = "per-reader")]
    PerReader {
        /// Every reader, those whose `needs` are unmet included.
        readers: Vec<super::ReaderDefault>,
    },
    /// Nothing declares one, so nothing is written for it.
    #[serde(rename = "none")]
    None,
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<SettingDefault, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for SettingDefault {
    const TAG: Option<&str> = Some("of");
    const KNOWN: &[&str] = &["core", "declared", "per-reader", "none"];
}

/// One setting a change moves.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettingMove {
    /// What moving it costs, where it costs anything.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consequence: Option<String>,
    /// What it falls back to while it holds no operator value, read with the change in
    /// place.
    pub default: super::SettingDefault,
    /// The setting.
    pub setting: String,
    /// What it is to hold, or nothing where it is cleared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// What it holds now.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was: Option<String>,
}

/// One setting.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettingRead {
    /// Every value it accepts, where its manifest declares them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<super::Choosable>>,
    /// What the core reads while no value is recorded, for one of the core's own settings
    /// that holds anything then; a service's setting has its manifest's on each reader.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// The environment file's key it is held under, where it is held there.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Its name.
    pub name: String,
    /// Each service that reads it.
    pub readers: Vec<super::Reader>,
    /// The operator's value, where one is recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Whose it is.
    pub whose: super::Owner,
}

/// One setting, as it is safe to show.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettingReport {
    /// The setting's name.
    pub key: String,
    /// Where the value came from, beside the value rather than behind a second
    /// request — reading a setting and reading what put it there are one act.
    pub origin: super::ValueOrigin,
    /// Whether the value was withheld.
    pub secret: bool,
    /// Its value, or a note that it is set and withheld.
    pub value: String,
}

/// The action a setting is written with.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum SettingWrite {
    /// `config-set`, which writes the configuration.
    #[serde(rename = "config-set")]
    ConfigSet,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Every setting and secret the stack's services read, and what changing a setting came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettingsReport {
    /// The change asked for, where one was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change: Option<super::SettingChange>,
    /// Each of lemonfiber's own guards this act moved onto its plugin after its own
    /// change, or on a reading would move, for good and with no yes of its own; one it
    /// could not move stays on offer and says why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guards: Option<Vec<super::StackMoveTaken>>,
    /// Whether this was a rehearsal.
    pub rehearsed: bool,
    /// Every secret a service's environment names, in name order, never with its value.
    pub secrets: Vec<super::SecretRead>,
    /// Every setting, in name order.
    pub settings: Vec<super::SettingRead>,
    /// The operator's overlay and every service it changes, where there is one: merged
    /// over the project this run would start, and held to nothing lemonfiber vets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unvetted: Option<super::Unvetted>,
    /// The services held back until the operator gives what they need to start, each
    /// with what it waits for, as status names them.
    pub waiting: Vec<super::Waiting>,
}

/// What became of a rotation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "settled")]
#[non_exhaustive]
pub enum Settled {
    /// The replacement was proven and is now the value in force.
    #[serde(rename = "replaced")]
    Replaced {
        /// What the service did while proving it — an observation, never the value.
        observed: String,
    },
    /// The service answered and refused the replacement. Nothing was changed.
    #[serde(rename = "refused")]
    Refused {
        /// What the service said, with any credential in it withheld.
        detail: String,
    },
    /// Nothing usable answered, so the replacement could not be proven. Nothing
    /// was changed, because an unproven replacement is not a better one.
    #[serde(rename = "unproven")]
    Unproven {
        /// Why nothing could be concluded.
        detail: String,
    },
    /// The replacement is in force, and did not reach everything that reads it: the
    /// service replaced the credential itself and the replacement did not answer, or a
    /// file that names it could not be written.
    ///
    /// The one way of not landing that keeps nothing: a service asked to replace its
    /// own key drops the old one the moment it makes the new one, and a value already
    /// held cannot be called replaced while a copy of it was never written. What is owed
    /// is the command that hands the new one out.
    #[serde(rename = "replaced-unproven")]
    ReplacedUnproven {
        /// What did not answer, and what to run once it does.
        detail: String,
    },
    /// Nothing was attempted, because this run only said what a rotation would do.
    ///
    /// Its own outcome rather than one of the refusals above, because it is not a
    /// refusal: nothing went wrong, and what an operator is being told is what would
    /// happen if they ran it again meaning it. Carrying its own three fields rather
    /// than one sentence, because "it would rotate the administrator's password" is not a
    /// report — where the value lives is what would be written over, and what is owed
    /// afterwards is the half nobody finds out about until a consumer stops working.
    ///
    /// No value appears here and none is generated to put here. A replacement minted
    /// to describe a rotation is a secret that exists because somebody asked a
    /// question, and it would then have to be kept or thrown away — and one thrown
    /// away may be one the service has already taken.
    #[serde(rename = "rehearsed")]
    Rehearsed {
        /// What would still need doing before every consumer held the replacement.
        afterwards: Vec<String>,
        /// What a real run would do, step by step, in lemonfiber's own words.
        detail: String,
        /// Where the value that would be replaced is kept.
        location: String,
    },
    /// Nothing in this stack holds a credential by that name.
    #[serde(rename = "unknown")]
    Unknown {
        /// The names that would have been accepted.
        known: Vec<String>,
    },
    /// A replacement for this one does not come from here.
    ///
    /// Either the operator's provider issued it, in which case inventing one would
    /// produce a credential no service has ever heard of; or the service that holds
    /// it offers no way to change it in place. Either way what is owed is a
    /// sentence saying where a replacement does come from, not an attempt.
    #[serde(rename = "elsewhere")]
    Elsewhere {
        /// Where a replacement comes from, and what to do once it exists.
        detail: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged, deserialize_with = "crate::open::unrecognised::<Settled, _>")]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Settled {
    const TAG: Option<&str> = Some("settled");
    const KNOWN: &[&str] = &[
        "replaced",
        "refused",
        "unproven",
        "replaced-unproven",
        "rehearsed",
        "unknown",
        "elsewhere",
    ];
}

/// What setup settled, read from the configuration as it stands, and whether this stack
/// is set up.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettledReport {
    /// Where the library is kept, a path on this machine or an NFS export as `DATA_ROOT`
    /// names it, or nothing where no data root is configured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_root: Option<String>,
    /// Which ways of downloading are configured.
    pub protocols: super::Protocols,
    /// The user and group the stack's services run as.
    pub service_user: super::ServiceUser,
    /// Whether this stack is set up.
    pub setup: super::SetUp,
}

/// One answer, tagged by the step it belongs to.
///
/// Read back as well as built, because a surface that is not in this process
/// submits one: the tag is the step, so an answer names the question it belongs
/// to rather than arriving as a field a reader has to guess the meaning of.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SetupAnswerBody {
    /// The house's zone and locale, as the operator confirmed or gave them.
    #[serde(rename = "house")]
    House(Box<super::SetupHouse>),
    /// The protocol choice.
    #[serde(rename = "protocols")]
    Protocols(Box<super::Protocols>),
    /// Whether a VPN carries the torrents, and where none does, that the operator
    /// accepted what that means.
    #[serde(rename = "vpn")]
    Vpn(Box<super::SetupVpn>),
    /// The data location.
    #[serde(rename = "data-location")]
    DataLocation(String),
    /// The indexer credential the operator settled on, or none where they entered
    /// nothing.
    #[serde(rename = "credentials")]
    Credentials(Option<super::Indexer>),
    /// The Usenet provider the operator settled on, or none where they entered
    /// nothing.
    #[serde(rename = "provider")]
    Provider(Option<super::Provider>),
    /// The container user and group, or none where it is not needed.
    #[serde(rename = "service-user")]
    ServiceUser(Option<(u32, u32)>),
    /// How the library is served.
    #[serde(rename = "library")]
    Library(Box<super::Library>),
    /// How much the operator wants to be told about.
    #[serde(rename = "notifications")]
    Notifications(Box<super::Appetite>),
    /// Whether the household uses it.
    #[serde(rename = "household")]
    Household(bool),
    /// Whether to start on boot.
    #[serde(rename = "autostart")]
    Autostart(bool),
}

/// The egress guard's settings and secrets, as setup's VPN step was answered with them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetupGuarded {
    /// Each of its secrets given, by the name the secret store holds it under, with its
    /// value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<std::collections::BTreeMap<String, String>>,
    /// Each of its settings given, by name, with its value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<std::collections::BTreeMap<String, String>>,
}

/// Where the house is: the zone the stack runs in and the house's language and region.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetupHouse {
    /// The house's language and region.
    pub locale: super::Locale,
    /// The zone the stack runs in, which it writes as `TZ`.
    pub zone: super::Zone,
}

/// How a setup run ended.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum SetupOutcome {
    /// The reviewed answers were written.
    #[serde(rename = "applied")]
    Applied,
    /// The plan was seen and not applied; nothing was written.
    #[serde(rename = "abandoned")]
    Abandoned,
    /// Nothing was asked, because this machine was already set up.
    #[serde(rename = "already-set-up")]
    AlreadySetUp,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// The way out of an interrupted apply a caller picked.
///
/// A named field rather than a bare word, so the body is an object like every other
/// this surface takes and a second thing to decide can be added without the shape
/// changing under a caller.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetupRecoverBody {
    /// Which of the three ways out.
    pub choice: super::Choice,
}

/// What a setup run came to, and what it settled on.
///
/// **Deliberately not the settings themselves.** Setup writes an indexer key and a
/// service password among them, and a report a script can read is a report a script
/// can log — into a file, a CI transcript, somebody's terminal history. So this says
/// what was *decided* and never what was *entered*, and the fields are chosen one at
/// a time rather than by serialising a struct that might later gain a secret.
///
/// The indexer's address is left out for that reason rather than because it is
/// itself a secret: it is entered beside its key, and the two travel together in
/// every place an operator copies them from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetupReport {
    /// Where the library was put, where a location was chosen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_root: Option<String>,
    /// How the run ended.
    pub outcome: super::SetupOutcome,
    /// Which ways of downloading the stack was set up for.
    pub protocols: super::Protocols,
    /// The user the services run as, as `uid:gid`, where one was set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_user: Option<String>,
}

/// Whether a VPN carries the torrent traffic, and where none does, that the
/// operator was told what that costs and chose to go on.
///
/// Two states rather than a bool, because the second is not merely "no". Torrents
/// expose the home address to every peer, so going without is a decision the
/// operator has to make knowingly — and one a later run must not quietly re-ask,
/// nor a diagnosis mistake for an oversight. Recording the acceptance is what
/// makes it theirs.
///
/// Read from what a progress file written before the egress guard's questions holds
/// too: the bare `"carrying"` reads as carrying with nothing given.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SetupVpn {
    /// Carrying, with what was given.
    SetupVpnCarrying(Box<super::SetupVpnCarrying>),
    /// Carrying with nothing given, or nothing carrying it.
    SetupVpnSaid(Box<super::SetupVpnSaid>),
}

/// Carrying, with what was given.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SetupVpnCarrying {
    /// A VPN carries it, through the egress guard set up with what was given.
    #[serde(rename = "carrying")]
    Carrying(Box<super::SetupGuarded>),
}

/// Either answer, in a word.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SetupVpnSaid {
    /// A VPN carries it, with nothing given for the egress guard.
    #[serde(rename = "carrying")]
    Carrying,
    /// Nothing does, and the operator confirmed they understood that.
    #[serde(rename = "absent")]
    Absent,
}

/// Which walkthrough this stack is offered.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Shape {
    /// The full walk: search, grab, download, import, and see it in the library.
    #[serde(rename = "pipeline")]
    Pipeline,
    /// The walk for a stack that acquires nothing: point at media already on disk and
    /// confirm the media server can see it.
    #[serde(rename = "library-only")]
    LibraryOnly,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// How the line is shared, and what that costs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sharing {
    /// What a spent cap is doing to the figures above, where one is spent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acting: Option<String>,
    /// Whether this run wrote the limits to the clients or only read them.
    pub applied: bool,
    /// The monthly cap, where one was declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<super::Cap>,
    /// What the line was measured to carry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<super::Capacity>,
    /// What is worth knowing about that reading before trusting it.
    pub cautions: Vec<String>,
    /// What each download client was asked and what it is doing about it.
    pub clients: Vec<super::Holding>,
    /// The download limit.
    pub down: super::BandwidthReading,
    /// What that means for the household.
    pub means: String,
    /// What the stack itself moved this month.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metered: Option<super::Metered>,
    /// What throttling the upload costs, where an upload limit is in force.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    /// Where the month stands against it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reached: Option<super::Reached>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// The override, where one is running or has just run out.
    pub respite: super::RespiteStanding,
    /// What the override amounts to, in words.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub respite_says: Option<String>,
    /// Where the line stands.
    pub restraint: super::Restraint,
    /// The household's hours, where any were declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rhythm: Option<super::Rhythm>,
    /// What is outside every limit here.
    pub untouched: Vec<String>,
    /// The upload limit, which is declared apart and defaults lower.
    pub up: super::BandwidthReading,
    /// The zone the clients read those hours in, where the stack says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// How far a claim has been shown.
///
/// The four the vocabulary's own states name. `Claimed` is not a weaker
/// `Demonstrated`: it is what a declaration is before anything has asked the service,
/// which is a fact about this moment rather than about the service.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Shown {
    /// A service declares it, and nothing has asked the service yet.
    #[serde(rename = "claimed")]
    Claimed,
    /// The probes ran and passed against the evidence the report names.
    ///
    /// Which is not the same claim in each case, and the report is where the two are
    /// told apart: a recording that answers is evidence about a moment somebody wrote
    /// down, and a service that answers is evidence about the service. This word alone
    /// does not distinguish them and must not be read as though it did.
    #[serde(rename = "demonstrated")]
    Demonstrated,
    /// The probes could not be run. The claim stands unverified and is not met.
    #[serde(rename = "unproven")]
    Unproven,
    /// The probes ran and failed. The claim is false.
    #[serde(rename = "refuted")]
    Refuted,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A code to sign in by, or the sign-in by a code still waiting.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SignInCode {
    /// Where the member approves the code, where one was just asked for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_at: Option<String>,
    /// The code the member enters where they approve it, where one was just asked for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// When the code lapses.
    pub lapses_at: super::Instant,
    /// The handle that finishes signing in by the code.
    pub waiting: String,
}

/// Everything the dashboard shows at one moment.
///
/// Each source's panel is filled or marked unavailable on its own, so one dead
/// source degrades one region rather than the screen. The surface builds this from
/// what it gathered; the standing is read from the same facts so it cannot
/// disagree with the panels.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
    /// What the operator has been told, newest first: what is owed them where a
    /// channel is refusing, then what has already been said.
    pub alerts: Vec<super::Alert>,
    /// The one address to hand somebody who lives here.
    ///
    /// On the screen rather than only behind a question, because the operator who
    /// needs it is not the one who thought to ask: they have just been asked "what
    /// do I open?" by somebody in the next room. Built from the same reading as the
    /// panels beside it, so the screen and `front-door` cannot name different doors.
    pub door: super::PanelFrontDoorReport,
    /// Every download client the stack runs, and whether each is paused.
    pub downloaders: super::PanelArrayOfDownloader,
    /// The one-line health summary — the same computation every other surface
    /// uses, so no two of them can grade the same stack differently.
    ///
    /// Always present, unlike the panels: a stack that could not be reached has a
    /// summary, and it says `unknown`. An absent summary would leave the operator
    /// to infer health from a blank space, which is the one reading this must never
    /// be open to.
    pub health: super::HealthSummary,
    /// What the household has asked for that is not moving.
    ///
    /// On the screen rather than only behind a question, for the reason the door
    /// beside it is: a request waiting on a decision or failed after one is waiting
    /// on the operator, and an operator who has to think to ask is one who finds out
    /// when somebody comes to complain.
    pub household: super::PanelHouseholdReport,
    /// The per-service queues.
    pub queue: super::PanelArrayOfQueue,
    /// Each run-and-exit service, with how its last run ended.
    pub scheduled: Vec<super::ScheduledService>,
    /// Every service and what it is doing.
    pub services: super::PanelArrayOfService,
    /// The storage picture.
    pub storage: super::PanelStorage,
    /// What in the pipeline has stopped, worst first — assessed across the
    /// download clients and the curators together, because the failure that matters
    /// most is invisible inside either.
    pub stuck: Vec<super::Stuck>,
    /// Whether the screen itself can be trusted to be current.
    pub telemetry: super::Telemetry,
    /// The active transfers.
    pub transfers: super::PanelArrayOfTransfer,
    /// The VPN, or `None` where no VPN is configured and the panel is omitted
    /// rather than shown permanently red.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpn: Option<super::PanelVpn>,
}

/// What sort of thing one line of a manifest is.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Sort {
    /// A container the engine is holding.
    #[serde(rename = "container")]
    Container,
    /// A network the containers were on.
    #[serde(rename = "network")]
    Network,
    /// An image that was pulled.
    #[serde(rename = "image")]
    Image,
    /// A directory or file on this machine.
    #[serde(rename = "path")]
    Path,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where a figure for the line came from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Source {
    /// The operator gave it, presumably off the plan they pay for.
    #[serde(rename = "declared")]
    Declared,
    /// The fastest the stack has been seen to move, which is what it achieved
    /// rather than what the line is sold as.
    #[serde(rename = "observed")]
    Observed,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What one line of the accounting is about.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "of")]
#[non_exhaustive]
pub enum SpaceCategory {
    /// One directory beneath the data root, named as the operator named it.
    ///
    /// Per directory rather than one figure for the library, because several
    /// libraries commonly share a volume and "the library is large" tells nobody
    /// which of them is growing.
    #[serde(rename = "tree")]
    Tree {
        /// The `name` field.
        name: String,
    },
    /// What the download clients still have to write.
    #[serde(rename = "landing")]
    Landing,
    /// Completed downloads the client is still seeding.
    #[serde(rename = "seeding")]
    Seeding,
    /// Downloads on disk that no service ever took.
    #[serde(rename = "orphaned")]
    Orphaned,
    /// Archives whose extracted contents sit beside them.
    #[serde(rename = "extracted")]
    Extracted,
    /// The services' own configuration and databases.
    #[serde(rename = "services")]
    Services,
    /// What the operator said to leave alone.
    #[serde(rename = "unmanaged")]
    Unmanaged,
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<SpaceCategory, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for SpaceCategory {
    const TAG: Option<&str> = Some("of");
    const KNOWN: &[&str] = &[
        "tree",
        "landing",
        "seeding",
        "orphaned",
        "extracted",
        "services",
        "unmanaged",
    ];
}

/// Something a cleanup could not take.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpaceLeft {
    /// Where it is.
    pub at: String,
    /// What the platform said, verbatim.
    pub why: String,
}

/// A stack file the operator edited, preserved rather than overwritten, with the
/// change an upgrade would make shown against it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackEdit {
    /// The lines that differ between the operator's file and what lemonfiber would
    /// write — theirs marked `-`, lemonfiber's `+`, the matching head and tail left
    /// out. Empty where the two differ only in ways `lines` does not see.
    pub diff: String,
    /// The file's path within the stack directory.
    pub path: String,
}

/// A part of the stack that the plain Compose words alone do not bring up as lemonfiber
/// does, and why.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackException {
    /// Why, in the operator's words.
    pub because: String,
    /// The part, by its id.
    pub part: String,
}

/// A move in place this run took, or tried to take and could not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackMoveTaken {
    /// The plugin the bundle installs.
    pub plugin: String,
    /// The stack's own service it took, or would have taken, the place of.
    pub service: String,
    /// Whether the plugin now runs in the service's place.
    pub taken: bool,
    /// Why it was not taken, where it was not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// A download provider a profile can depend on.
///
/// Serialisable as well as readable, for the same reason [`Criticality`] is: it
/// reaches an operator. A profile left out of a closure is only half reported
/// without the provider it wanted.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum StackProtocol {
    /// Needs a Usenet provider.
    #[serde(rename = "usenet")]
    Usenet,
    /// Needs a VPN and a torrent client.
    #[serde(rename = "torrent")]
    Torrent,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// How this stack runs without lemonfiber.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackReport {
    /// The `.env` file beside it.
    pub env: String,
    /// Every part the words alone do not bring up as lemonfiber does. Empty where there
    /// is none.
    pub exceptions: Vec<super::StackException>,
    /// How any action is run without a question being put.
    pub non_interactive: super::NonInteractive,
    /// The `docker compose` words that bring the stack up with no lemonfiber running, as
    /// lemonfiber's own start would run them.
    pub plain_up: Vec<String>,
    /// The directory the Compose project is written to.
    pub project: String,
    /// Whether this machine runs the stack this build ships or the operator's own.
    pub source: super::StackSource,
}

/// Which stack this machine runs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum StackSource {
    /// The stack this build ships.
    #[serde(rename = "bundled")]
    Bundled,
    /// The operator's own stack directory, by its path.
    #[serde(rename = "own")]
    Own(String),
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<StackSource, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for StackSource {
    const TAG: Option<&str> = None;
    const KNOWN: &[&str] = &["bundled", "own"];
}

/// What the web API composes its routes and its answers from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackStanding {
    /// Whether household members may mint keys for themselves.
    pub members_mint: bool,
    /// The origin a browser plays a title from, which the page's policy allows, where
    /// the core knows one: built from the stack's front door, never from a request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plays_from: Option<super::PlayOrigin>,
    /// The stack's own identifier, the one pairing material carries; absent where this
    /// machine has nowhere to keep one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
    /// Each setting a command is refused without that is off on this machine.
    pub switched_off: Vec<String>,
    /// Whether a way to be told is configured.
    pub told: bool,
    /// Whether a phone's push endpoint is waiting to be woken.
    pub waiting: bool,
}
