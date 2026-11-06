//! The lemonfiber web API's shapes, reads, actions and refusals, generated from the
//! contract artefact taken from the lemonfiber revision [`REVISION`] names.
//! `just generate` writes this module, and CI refuses it where it differs from
//! what that writes.

pub mod actions;
pub mod key_callable;
pub mod reads;
pub mod refusals;
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
mod shapes_p;
pub use shapes_p::*;
mod shapes_q;
pub use shapes_q::*;
mod shapes_r;
pub use shapes_r::*;
mod shapes_s;
pub use shapes_s::*;
mod shapes_t;
pub use shapes_t::*;
mod shapes_u;
pub use shapes_u::*;
mod shapes_v;
pub use shapes_v::*;
mod shapes_w;
pub use shapes_w::*;

/// The wire version this contract describes.
pub const API_VERSION: u32 = 1;

/// The lemonfiber revision this contract was taken from.
pub const REVISION: &str = "abd7d75f6b2627327b6935f5322f696016413234";

/// Every kind the contract describes.
pub const KINDS: &[&str] = &[
    "admission",
    "adoption",
    "alert",
    "alerts",
    "archives",
    "askable",
    "asked",
    "backup",
    "bandwidth",
    "beside",
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
    "quality",
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
    "wiring",
    "wizard",
    "word",
];

/// The data of an envelope, typed by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
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
    /// The `quality` kind.
    #[serde(rename = "quality")]
    Quality(Box<QualityReport>),
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
