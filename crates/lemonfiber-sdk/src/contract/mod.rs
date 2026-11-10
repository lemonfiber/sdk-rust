//! The lemonfiber web API's shapes, reads, actions and refusals, generated from the
//! contract artefact taken from the lemonfiber revision [`REVISION`] names.
//! `just generate` writes this module, and CI refuses it where it differs from
//! what that writes.

mod action_names;
mod actions_a;
mod actions_b;
mod actions_c;
mod actions_d;
mod actions_f;
mod actions_g;
mod actions_h;
mod actions_i;
mod actions_m;
mod actions_p;
mod actions_q;
mod actions_r;
mod actions_s;
mod actions_u;
mod actions_w;
pub mod key_callable;
mod reads_a;
mod reads_b;
mod reads_c;
mod reads_e;
mod reads_f;
mod reads_h;
mod reads_l;
mod reads_m;
mod reads_n;
mod reads_o;
mod reads_p;
mod reads_q;
mod reads_r;
mod reads_s;
mod reads_t;
mod reads_u;
mod reads_v;
mod reads_w;
mod refusal_list;
mod refusals_admit;
mod refusals_ask;
mod refusals_bundle;
mod refusals_cred;
mod refusals_device;
mod refusals_form;
mod refusals_gone;
mod refusals_key;
mod refusals_life;
mod refusals_migrate;
mod refusals_play;
mod refusals_plugin;
mod refusals_push;
mod refusals_quota;
mod refusals_rate;
mod refusals_read;
mod refusals_rehearse;
mod refusals_remove;
mod refusals_repair;
mod refusals_restore;
mod refusals_serve;
mod refusals_setting;
mod refusals_setup;
mod refusals_space;
mod refusals_stack;
mod refusals_storage;
mod refusals_undo;
mod refusals_update;
mod refusals_wire;
mod refusals_wish;
mod refusals_word;
mod shapes_a;
pub use shapes_a::*;
mod shapes_b;
pub use shapes_b::*;
mod shapes_c;
pub use shapes_c::*;
mod shapes_d;
pub use shapes_d::*;
mod shapes_e;
pub use shapes_e::*;
mod shapes_f;
pub use shapes_f::*;
mod shapes_g;
pub use shapes_g::*;
mod shapes_h;
pub use shapes_h::*;
mod shapes_i;
pub use shapes_i::*;
mod shapes_j;
pub use shapes_j::*;
mod shapes_k;
pub use shapes_k::*;
mod shapes_l;
pub use shapes_l::*;
mod shapes_m;
pub use shapes_m::*;
mod shapes_n;
pub use shapes_n::*;
mod shapes_o;
pub use shapes_o::*;
mod shapes_p_1;
pub use shapes_p_1::*;
mod shapes_p_2;
pub use shapes_p_2::*;
mod shapes_p_3;
pub use shapes_p_3::*;
mod shapes_q;
pub use shapes_q::*;
mod shapes_r_1;
pub use shapes_r_1::*;
mod shapes_r_2;
pub use shapes_r_2::*;
mod shapes_s_1;
pub use shapes_s_1::*;
mod shapes_s_2;
pub use shapes_s_2::*;
mod shapes_s_3;
pub use shapes_s_3::*;
mod shapes_t;
pub use shapes_t::*;
mod shapes_u;
pub use shapes_u::*;
mod shapes_v;
pub use shapes_v::*;
mod shapes_w;
pub use shapes_w::*;

/// Every read the web API serves: a struct for each, holding what it takes.
pub mod reads {
    pub use super::reads_a::*;
    pub use super::reads_b::*;
    pub use super::reads_c::*;
    pub use super::reads_e::*;
    pub use super::reads_f::*;
    pub use super::reads_h::*;
    pub use super::reads_l::*;
    pub use super::reads_m::*;
    pub use super::reads_n::*;
    pub use super::reads_o::*;
    pub use super::reads_p::*;
    pub use super::reads_q::*;
    pub use super::reads_r::*;
    pub use super::reads_s::*;
    pub use super::reads_t::*;
    pub use super::reads_u::*;
    pub use super::reads_v::*;
    pub use super::reads_w::*;
}

/// Every action the web API takes: a struct for each, holding its arguments.
pub mod actions {
    pub use super::action_names::NAMES;
    pub use super::actions_a::*;
    pub use super::actions_b::*;
    pub use super::actions_c::*;
    pub use super::actions_d::*;
    pub use super::actions_f::*;
    pub use super::actions_g::*;
    pub use super::actions_h::*;
    pub use super::actions_i::*;
    pub use super::actions_m::*;
    pub use super::actions_p::*;
    pub use super::actions_q::*;
    pub use super::actions_r::*;
    pub use super::actions_s::*;
    pub use super::actions_u::*;
    pub use super::actions_w::*;
}

/// Every code a refusal may carry, with the status it is answered with.
pub mod refusals {
    pub use super::refusal_list::ALL;
    pub use super::refusals_admit::*;
    pub use super::refusals_ask::*;
    pub use super::refusals_bundle::*;
    pub use super::refusals_cred::*;
    pub use super::refusals_device::*;
    pub use super::refusals_form::*;
    pub use super::refusals_gone::*;
    pub use super::refusals_key::*;
    pub use super::refusals_life::*;
    pub use super::refusals_migrate::*;
    pub use super::refusals_play::*;
    pub use super::refusals_plugin::*;
    pub use super::refusals_push::*;
    pub use super::refusals_quota::*;
    pub use super::refusals_rate::*;
    pub use super::refusals_read::*;
    pub use super::refusals_rehearse::*;
    pub use super::refusals_remove::*;
    pub use super::refusals_repair::*;
    pub use super::refusals_restore::*;
    pub use super::refusals_serve::*;
    pub use super::refusals_setting::*;
    pub use super::refusals_setup::*;
    pub use super::refusals_space::*;
    pub use super::refusals_stack::*;
    pub use super::refusals_storage::*;
    pub use super::refusals_undo::*;
    pub use super::refusals_update::*;
    pub use super::refusals_wire::*;
    pub use super::refusals_wish::*;
    pub use super::refusals_word::*;
}

/// The wire version this contract describes.
pub const API_VERSION: u32 = 1;

/// The lemonfiber revision this contract was taken from.
pub const REVISION: &str = "10531156453569a9e96110e9dddb5d472faf8aeb";

/// Every kind the contract describes.
pub const KINDS: &[&str] = &[
    "admission",
    "adoption",
    "alert",
    "alerts",
    "archives",
    "askable",
    "askable-kinds",
    "asked",
    "backup",
    "bandwidth",
    "beside",
    "boot",
    "bundle",
    "capabilities",
    "catalogue",
    "certificate",
    "clients",
    "config",
    "credentials",
    "dashboard",
    "device-challenge",
    "device-key",
    "device-keys",
    "doctor",
    "error",
    "forms",
    "front-door",
    "glossary",
    "grant",
    "handoff",
    "held",
    "history",
    "hosting",
    "household",
    "import",
    "invitation",
    "job",
    "key-claim",
    "keys",
    "lifecycle",
    "log",
    "migration",
    "minted-key",
    "music",
    "news",
    "news-items",
    "outbound",
    "pairing",
    "part-way",
    "pausing",
    "playing",
    "plugins",
    "preview",
    "provenance",
    "pull",
    "push-endpoint",
    "push-endpoints",
    "quality",
    "queue",
    "removal",
    "repair",
    "replacement",
    "reset",
    "restore",
    "seed",
    "self-update",
    "settings",
    "setup",
    "space",
    "stack",
    "start",
    "status",
    "step",
    "stop-seeding",
    "stored",
    "stuck",
    "substitution",
    "title",
    "trace",
    "undo",
    "uninstall",
    "update",
    "upgrade",
    "version",
    "walkthrough",
    "watch",
    "watched",
    "watcher",
    "wiring",
    "wizard",
    "word",
];

/// The data of an envelope, typed by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
#[non_exhaustive]
pub enum Payload {
    /// The `admission` kind.
    #[serde(rename = "admission")]
    Admission(Box<Admitted>),
    /// The `adoption` kind.
    #[serde(rename = "adoption")]
    Adoption(Box<AdoptReport>),
    /// The `alert` kind.
    #[serde(rename = "alert")]
    Alert(Box<Alert>),
    /// The `alerts` kind.
    #[serde(rename = "alerts")]
    Alerts(Box<AlertReport>),
    /// The `archives` kind.
    #[serde(rename = "archives")]
    Archives(Box<Listing>),
    /// The `askable` kind.
    #[serde(rename = "askable")]
    Askable(Box<AskableReport>),
    /// The `askable-kinds` kind.
    #[serde(rename = "askable-kinds")]
    AskableKinds(Box<AskableKindsReport>),
    /// The `asked` kind.
    #[serde(rename = "asked")]
    Asked(Box<AskedReport>),
    /// The `backup` kind.
    #[serde(rename = "backup")]
    Backup(Box<BackupReport>),
    /// The `bandwidth` kind.
    #[serde(rename = "bandwidth")]
    Bandwidth(Box<Sharing>),
    /// The `beside` kind.
    #[serde(rename = "beside")]
    Beside(Box<BesideReport>),
    /// The `boot` kind.
    #[serde(rename = "boot")]
    Boot(Box<BootReport>),
    /// The `bundle` kind.
    #[serde(rename = "bundle")]
    Bundle(Box<Bundle>),
    /// The `capabilities` kind.
    #[serde(rename = "capabilities")]
    Capabilities(Box<Capabilities>),
    /// The `catalogue` kind.
    #[serde(rename = "catalogue")]
    Catalogue(Box<CatalogueReport>),
    /// The `certificate` kind.
    #[serde(rename = "certificate")]
    Certificate(Box<CertificateReport>),
    /// The `clients` kind.
    #[serde(rename = "clients")]
    Clients(Box<Guidance>),
    /// The `config` kind.
    #[serde(rename = "config")]
    Config(Box<ConfigReport>),
    /// The `credentials` kind.
    #[serde(rename = "credentials")]
    Credentials(Box<Inventory>),
    /// The `dashboard` kind.
    #[serde(rename = "dashboard")]
    Dashboard(Box<Snapshot>),
    /// The `device-challenge` kind.
    #[serde(rename = "device-challenge")]
    DeviceChallenge(Box<DeviceChallenge>),
    /// The `device-key` kind.
    #[serde(rename = "device-key")]
    DeviceKey(Box<RegisteredDeviceKey>),
    /// The `device-keys` kind.
    #[serde(rename = "device-keys")]
    DeviceKeys(Box<DeviceKeyListing>),
    /// The `doctor` kind.
    #[serde(rename = "doctor")]
    Doctor(Box<DoctorReport>),
    /// The `error` kind.
    #[serde(rename = "error")]
    Error(Box<Problem>),
    /// The `forms` kind.
    #[serde(rename = "forms")]
    Forms(Box<FormsReport>),
    /// The `front-door` kind.
    #[serde(rename = "front-door")]
    FrontDoor(Box<FrontDoorReport>),
    /// The `glossary` kind.
    #[serde(rename = "glossary")]
    Glossary(Box<Vocabulary>),
    /// The `grant` kind.
    #[serde(rename = "grant")]
    Grant(Box<GrantReport>),
    /// The `handoff` kind.
    #[serde(rename = "handoff")]
    Handoff(Box<HandoffReport>),
    /// The `held` kind.
    #[serde(rename = "held")]
    Held(Box<HeldReport>),
    /// The `history` kind.
    #[serde(rename = "history")]
    History(Box<HistoryReport>),
    /// The `hosting` kind.
    #[serde(rename = "hosting")]
    Hosting(Box<HostingReport>),
    /// The `household` kind.
    #[serde(rename = "household")]
    Household(Box<HouseholdReport>),
    /// The `import` kind.
    #[serde(rename = "import")]
    Import(Box<ImportReport>),
    /// The `invitation` kind.
    #[serde(rename = "invitation")]
    Invitation(Box<Invitation>),
    /// The `job` kind.
    #[serde(rename = "job")]
    Job(Box<Started>),
    /// The `key-claim` kind.
    #[serde(rename = "key-claim")]
    KeyClaim(Box<KeyClaim>),
    /// The `keys` kind.
    #[serde(rename = "keys")]
    Keys(Box<KeyListing>),
    /// The `lifecycle` kind.
    #[serde(rename = "lifecycle")]
    Lifecycle(Box<LifecycleReport>),
    /// The `log` kind.
    #[serde(rename = "log")]
    Log(Box<LogLine>),
    /// The `migration` kind.
    #[serde(rename = "migration")]
    Migration(Box<MigrationReport>),
    /// The `minted-key` kind.
    #[serde(rename = "minted-key")]
    MintedKey(Box<MintedKey>),
    /// The `music` kind.
    #[serde(rename = "music")]
    Music(Box<MusicReport>),
    /// The `news` kind.
    #[serde(rename = "news")]
    News(Box<Newest>),
    /// The `news-items` kind.
    #[serde(rename = "news-items")]
    NewsItems(Box<News>),
    /// The `outbound` kind.
    #[serde(rename = "outbound")]
    Outbound(Box<Leaving>),
    /// The `pairing` kind.
    #[serde(rename = "pairing")]
    Pairing(Box<Pairing>),
    /// The `part-way` kind.
    #[serde(rename = "part-way")]
    PartWay(Box<PartWayReport>),
    /// The `pausing` kind.
    #[serde(rename = "pausing")]
    Pausing(Box<PausingReport>),
    /// The `playing` kind.
    #[serde(rename = "playing")]
    Playing(Box<PlayingReport>),
    /// The `plugins` kind.
    #[serde(rename = "plugins")]
    Plugins(Box<PluginInstalls>),
    /// The `preview` kind.
    #[serde(rename = "preview")]
    Preview(Box<Plan>),
    /// The `provenance` kind.
    #[serde(rename = "provenance")]
    Provenance(Box<ProvenanceReport>),
    /// The `pull` kind.
    #[serde(rename = "pull")]
    Pull(Box<String>),
    /// The `push-endpoint` kind.
    #[serde(rename = "push-endpoint")]
    PushEndpoint(Box<RegisteredPushEndpoint>),
    /// The `push-endpoints` kind.
    #[serde(rename = "push-endpoints")]
    PushEndpoints(Box<PushEndpointListing>),
    /// The `quality` kind.
    #[serde(rename = "quality")]
    Quality(Box<QualityReport>),
    /// The `queue` kind.
    #[serde(rename = "queue")]
    Queue(Box<QueueHealth>),
    /// The `removal` kind.
    #[serde(rename = "removal")]
    Removal(Box<HouseholdRemoval>),
    /// The `repair` kind.
    #[serde(rename = "repair")]
    Repair(Box<RepairReport>),
    /// The `replacement` kind.
    #[serde(rename = "replacement")]
    Replacement(Box<ReplaceReport>),
    /// The `reset` kind.
    #[serde(rename = "reset")]
    Reset(Box<ResetReport>),
    /// The `restore` kind.
    #[serde(rename = "restore")]
    Restore(Box<Restoration>),
    /// The `seed` kind.
    #[serde(rename = "seed")]
    Seed(Box<SeedReport>),
    /// The `self-update` kind.
    #[serde(rename = "self-update")]
    SelfUpdate(Box<UpdateReport>),
    /// The `settings` kind.
    #[serde(rename = "settings")]
    Settings(Box<SettingsReport>),
    /// The `setup` kind.
    #[serde(rename = "setup")]
    Setup(Box<SetupReport>),
    /// The `space` kind.
    #[serde(rename = "space")]
    Space(Box<Reckoning>),
    /// The `stack` kind.
    #[serde(rename = "stack")]
    Stack(Box<StackReport>),
    /// The `start` kind.
    #[serde(rename = "start")]
    Start(Box<String>),
    /// The `status` kind.
    #[serde(rename = "status")]
    Status(Box<StatusReport>),
    /// The `step` kind.
    #[serde(rename = "step")]
    Step(Box<Line>),
    /// The `stop-seeding` kind.
    #[serde(rename = "stop-seeding")]
    StopSeeding(Box<Letting>),
    /// The `stored` kind.
    #[serde(rename = "stored")]
    Stored(Box<Stored>),
    /// The `stuck` kind.
    #[serde(rename = "stuck")]
    Stuck(Box<StuckReport>),
    /// The `substitution` kind.
    #[serde(rename = "substitution")]
    Substitution(Box<SubstitutionReport>),
    /// The `title` kind.
    #[serde(rename = "title")]
    Title(Box<TitleReport>),
    /// The `trace` kind.
    #[serde(rename = "trace")]
    Trace(Box<TraceReport>),
    /// The `undo` kind.
    #[serde(rename = "undo")]
    Undo(Box<UndoReversal>),
    /// The `uninstall` kind.
    #[serde(rename = "uninstall")]
    Uninstall(Box<Uninstall>),
    /// The `update` kind.
    #[serde(rename = "update")]
    Update(Box<StackUpdateReport>),
    /// The `upgrade` kind.
    #[serde(rename = "upgrade")]
    Upgrade(Box<UpgradeReport>),
    /// The `version` kind.
    #[serde(rename = "version")]
    Version(Box<VersionReport>),
    /// The `walkthrough` kind.
    #[serde(rename = "walkthrough")]
    Walkthrough(Box<WalkthroughReport>),
    /// The `watch` kind.
    #[serde(rename = "watch")]
    Watch(Box<SupervisionReport>),
    /// The `watched` kind.
    #[serde(rename = "watched")]
    Watched(Box<WatchedReport>),
    /// The `watcher` kind.
    #[serde(rename = "watcher")]
    Watcher(Box<WatcherReport>),
    /// The `wiring` kind.
    #[serde(rename = "wiring")]
    Wiring(Box<WiringReport>),
    /// The `wizard` kind.
    #[serde(rename = "wizard")]
    Wizard(Box<WizardReport>),
    /// The `word` kind.
    #[serde(rename = "word")]
    Word(Box<Term>),
}
