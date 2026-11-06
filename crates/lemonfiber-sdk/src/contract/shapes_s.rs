/// How much of the stack a backup covers.
///
/// Whole-stack is the common case, but restoring one service is often what is
/// actually wanted — one curator's configuration mangled while the rest is fine —
/// so the scope is recorded in the archive and honoured on the way back.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "scope")]
pub enum Scope {
    /// Every service's configuration, plus lemonfiber's own and the stack.
    #[serde(rename = "whole_stack")]
    WholeStack,
    /// One named service's configuration alone.
    #[serde(rename = "service")]
    Service {
        /// The service whose configuration this covers.
        name: String,
    },
    /// An existing setup's own configuration, at the host paths it keeps it in.
    ///
    /// The one scope whose sources are not lemonfiber's layout. A capture taken
    /// before a takeover has to cover the tree that is already there — lemonfiber's
    /// own holds nothing worth protecting until the takeover has happened — so the
    /// host path each tree was read from is recorded here, in the manifest, rather
    /// than inferred from a layout that does not describe it.
    ///
    /// Recording those paths is also what makes putting one back an ordinary
    /// extraction the operator performs deliberately, rather than something
    /// lemonfiber does on their behalf into a tree it does not manage.
    #[serde(rename = "existing")]
    Existing {
        /// The Compose project the capture was taken from.
        project: String,
        /// The host trees captured, in the order the survey reported them.
        trees: Vec<super::Tree>,
    },
}

/// What a search that could tell this trace's stall apart would spend, and what it covers.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Searchable {
    /// What the search covers: the curator's wanted list at the quality in force.
    pub covers: String,
    /// What the search spends, in the stack's words.
    pub spends: String,
}

/// One season of a series, with its episodes.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Season {
    /// Its episodes, in order, each with where it is served.
    pub episodes: Vec<super::Episode>,
    /// What the server tells it apart by.
    pub id: String,
    /// What it is called.
    pub name: String,
    /// Its number in the series, where it has one. Specials often have none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
}

/// How much of one season is actually here, and what is outstanding — the season-level
/// answer, which for a series is the one an operator can act on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SeasonCoverage {
    /// How many of the wanted parts are here.
    pub have: u64,
    /// The wanted parts that are not here yet, each carrying the stage it rests at, so
    /// one that stalled is told apart from one still downloading.
    pub outstanding: Vec<super::Part>,
    /// The season number. Season zero is where a service files specials.
    pub season: u32,
    /// How many parts nobody asked for — unmonitored and not on disk.
    pub unmonitored: u64,
    /// How many parts were asked for, or are already here — the denominator. Parts
    /// nobody asked for are counted separately rather than inflating this, so a season
    /// with every wanted episode present reads as complete even where specials are not.
    pub wanted: u64,
}

/// A key's secret, as it is handed over once.
///
/// It has no `Debug` that prints it and no way back from its digest, and nothing here
/// writes it anywhere: the one reply that carries it is the only place it appears.
pub type Secret = String;

/// Where a secret's value came from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SecretFrom {
    /// The operator entered it.
    #[serde(rename = "operator")]
    Operator,
    /// A recipe captured it.
    #[serde(rename = "recipe")]
    Recipe,
    /// It is another service's credential, granted to cross.
    #[serde(rename = "granted")]
    Granted,
    /// lemonfiber made it at install.
    #[serde(rename = "minted")]
    Minted,
}

/// One secret, without its value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SecretRead {
    /// Where its value came from, where one is held or granted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<super::SecretFrom>,
    /// Whether a value is held.
    pub held: bool,
    /// Its name.
    pub name: String,
    /// Each service that reads it.
    pub readers: Vec<super::Reader>,
    /// Whose it is.
    pub whose: super::Owner,
}

/// What a seed pass amounted to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SeedReport {
    /// Whether drift could be assessed, or the expected-state record was lost.
    pub assessment: super::Assessment,
    /// Whether this pass only said what it would do.
    ///
    /// A flag rather than a second shape, because every connection above means the
    /// same thing either way: what the service holds is what it holds, and what
    /// lemonfiber would write is what it would write. What changes is that two of the
    /// states are reachable only here, and that the last line of the report is an
    /// instruction to run it for real rather than to run it again.
    pub rehearsed: bool,
    /// Every connection attempted, and how each turned out.
    pub wirings: Vec<super::Wiring>,
}

/// How serious a reported connection is.
///
/// Drift is normal and usually the operator's own harmless edit, so it is reported
/// as information rather than a failure. It escalates to a warning only when the
/// drift breaks the stack — a root folder pointing where nothing exists, a download
/// client that no longer answers — and a warning that cannot be acted on is noise,
/// so a warning always names both what broke and what to do about it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "severity")]
pub enum SeedSeverity {
    /// Nothing is broken: the connection is settled, or its drift is the operator's
    /// own edit that still works.
    #[serde(rename = "informational")]
    Informational,
    /// The connection breaks the stack. Both the breakage and a remediation are
    /// named, because a warning the operator cannot act on is noise.
    #[serde(rename = "warning")]
    Warning {
        /// What is broken, in the operator's terms.
        breakage: String,
        /// What to do about it.
        remediation: String,
    },
}

/// How one connection turned out after a seed pass.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "state")]
pub enum SeedState {
    /// Written and read back.
    #[serde(rename = "wired")]
    Wired,
    /// Written, and in force once `service` started again, as the adapter that took it
    /// answered.
    #[serde(rename = "restarted")]
    Restarted {
        /// The service started again.
        service: String,
    },
    /// Present and correct; nothing was done.
    #[serde(rename = "already-wired")]
    AlreadyWired,
    /// Present but operator-changed; preserved.
    #[serde(rename = "drifted")]
    Drifted,
    /// A hand edit to the configuration a preset is held through, left in place: a run
    /// holding the curators to their preset does not write over it, and only
    /// `lemonfiber quality reapply` puts the preset back.
    #[serde(rename = "kept")]
    Kept {
        /// The edited configuration, as its upstream's configuration directory names it.
        path: String,
    },
    /// Present and still lemonfiber's own value, but behind lemonfiber's intent —
    /// it should be brought up to date. Reported until an update path applies it,
    /// and never overwritten in the meantime.
    #[serde(rename = "stale")]
    Stale,
    /// Both the service's value and lemonfiber's intent moved away from the
    /// baseline. The conflict is presented — the value the operator set beside the
    /// one lemonfiber would write — and the value left as it is; lemonfiber does not
    /// resolve it on its own.
    ///
    /// Both values are shown in the report and serialized with it, so a
    /// secret-bearing field must not report a conflict through this variant: a
    /// conflict in a secret is to be reported without either value on show, and
    /// wants a masked shape of its own rather than this one.
    #[serde(rename = "conflicted")]
    Conflicted {
        /// The value lemonfiber would write in its place.
        ours: String,
        /// The value the service now holds, as the operator set it; `None` where
        /// they cleared it.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        yours: Option<String>,
    },
    /// An operator's edit adopted as the accepted state, kept across seeds and
    /// restores. Settled: lemonfiber leaves it as it is.
    #[serde(rename = "adopted")]
    Adopted,
    /// A value the service already held that lemonfiber never wrote — the operator's
    /// own, pre-existing. Adopted as the baseline this run rather than reported as
    /// drift, so an existing setup is taken on instead of flagged wholesale. Its
    /// value is not shown, so a secret among the adopted is never put on display.
    #[serde(rename = "unmanaged")]
    Unmanaged,
    /// Not there, or not at what lemonfiber would have it be, and this run only said
    /// so.
    ///
    /// The one outcome a pass that writes nothing can reach where a pass that writes
    /// would have written. What a real run would leave the service holding sits beside
    /// what it holds now, because a report saying a connection would be made without
    /// saying what it would be made *to* is a count rather than an account — and a
    /// count is what an operator asking for a rehearsal already has.
    ///
    /// Both values are serialized, so a secret-bearing field must not report through
    /// this variant carrying one. It does not have to: `ours` is absent exactly where
    /// a real run would generate the value rather than read it, which is the only
    /// place a credential arises — the media server's admin account. A value minted to
    /// describe a rehearsal is a secret that exists because somebody asked a question, and
    /// it would then have to be kept or thrown away.
    ///
    /// `yours` is absent where the service holds nothing, and where this run could not
    /// ask without writing — reading the household's telling means signing in as the
    /// owner, and a session is state on somebody else's service.
    #[serde(rename = "would-wire")]
    WouldWire {
        /// What a real run would leave it holding, or `None` where that value would be
        /// generated rather than read.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ours: Option<String>,
        /// What the service holds now, or `None` where it holds nothing or could not
        /// be asked.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        yours: Option<String>,
    },
    /// An operator's own value a real run would take on as the accepted state, and
    /// this one did not.
    ///
    /// Apart from [`Self::WouldWire`] because it is the other direction: nothing would
    /// be written to the service at all, and what would move is lemonfiber's record of
    /// what it expects. Its value is not shown, exactly as [`Self::Unmanaged`] does not
    /// show one, so a secret among the adopted is never put on display by a question.
    #[serde(rename = "would-adopt")]
    WouldAdopt,
    /// An area the operator declared unmanaged. Nothing was read from the service and
    /// nothing was written to it, and the reason they gave is carried so a report says
    /// whose decision it was.
    ///
    /// Apart from [`Self::Unmanaged`], which lemonfiber *infers* from a value it has
    /// no record of having written, and which it adopts as the baseline so that later
    /// runs recognise it. This one is a decision somebody wrote down, and it holds
    /// whether or not lemonfiber would have had anything to say — nothing is adopted,
    /// because adopting would be the first half of managing it again.
    #[serde(rename = "observed")]
    Observed {
        /// Why the operator said to leave it alone, in their own words.
        reason: String,
    },
    /// Something on this machine fills what a service asks for, and nothing lemonfiber
    /// does connects the two — the filler names no API of its own, or one lemonfiber pairs
    /// with nothing the asker speaks.
    ///
    /// Settled, because no run changes it: the stack, or what is installed, has to. Said
    /// rather than left out, because a filler nothing reaches and a filler lemonfiber
    /// forgot would otherwise read the same, and the operator who installed one to stand
    /// in for another is the one who needs to know which.
    #[serde(rename = "unmatched")]
    Unmatched {
        /// What fills it, what asked, and why nothing connects them.
        reason: String,
    },
    /// Prerequisite unavailable; a later run will complete it.
    #[serde(rename = "skipped")]
    Skipped {
        /// Why it could not be attempted.
        reason: String,
    },
    /// Attempted and rejected, carrying the service's own words.
    #[serde(rename = "failed")]
    Failed {
        /// What the service said.
        detail: String,
    },
    /// Refused by lemonfiber's own policy, carrying the reason a re-run will not
    /// resolve — such as two curators pointed at one root folder. Nothing it names
    /// was written, whether it was refused before the write or the write itself
    /// found nothing to land in.
    #[serde(rename = "refused")]
    Refused {
        /// Why it was refused, in lemonfiber's own words.
        reason: String,
    },
}

/// Where one completed download stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "standing")]
pub enum SeedingStanding {
    /// Nothing ever linked it into a library: it was never imported, and removing
    /// it loses nothing.
    #[serde(rename = "never_imported")]
    NeverImported,
    /// It was imported and is still seeding, so removing it has a consequence
    /// outside this machine.
    #[serde(rename = "seeding")]
    Seeding {
        /// What it has uploaded against what it downloaded, in hundredths, as the
        /// client reports it.
        ///
        /// A whole number rather than a fraction because every report this product
        /// makes is compared for equality somewhere, and a fraction cannot be —
        /// two figures a client would call the same would not be. The hundredth is
        /// finer than any decision made on a ratio.
        ratio: u32,
    },
    /// The operator asked for this one to be left alone.
    #[serde(rename = "left_alone")]
    LeftAlone,
}

/// Where a copy of lemonfiber stands, in the words the specification uses.
///
/// Nothing known is the default, and it is the right one: a report built before
/// anything has been read has not established that this copy is current, and a
/// default that said so would be a claim made by an empty value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SelfUpdateStanding {
    /// The version running is the newest known.
    #[serde(rename = "current")]
    Current,
    /// A newer version exists, and replacing this copy is lemonfiber's own to do.
    #[serde(rename = "update-available")]
    UpdateAvailable,
    /// A newer version exists and a package manager owns this copy, so the update
    /// is that tool's to make.
    #[serde(rename = "managed-externally")]
    ManagedExternally,
    /// Availability could not be determined, which is not a fault and stops nothing.
    #[serde(rename = "check-failed")]
    CheckFailed,
}

/// One service, as it stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Service {
    /// How much its absence costs, so a summary can weigh it.
    pub criticality: super::Criticality,
    /// The services it needs before it can work: the one whose network it runs inside.
    /// Carried so a failure can be attributed to the thing underneath it rather
    /// than counted as one more independent thing wrong.
    pub depends_on: Vec<String>,
    /// What it does for the operator, in the stack's own words.
    ///
    /// Carried on the service rather than looked up where it is shown, because this
    /// is the one struct every surface reads: a listing, the machine-readable reply,
    /// the web API and the terminal's panel all render this, and a description
    /// fetched separately by each of them would be four chances to render three.
    ///
    /// The stack's words rather than lemonfiber's, for the reason its absence cost is:
    /// a stack that adds a service should not need a lemonfiber release before it can
    /// say what that service is for.
    pub describes: String,
    /// How it exited, where it has exited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<i32>,
    /// Every form it is running for, in the order the stack declares them.
    ///
    /// All of them rather than one, because a service two forms share is there for
    /// both, and stopping one of them leaves it running for the other. Empty where no
    /// form it belongs to is up: a service nobody's form holds is not missing from one.
    pub forms: Vec<String>,
    /// The service's identifier, which is also its Compose service name.
    pub id: String,
    /// What it is called in front of an operator.
    pub name: String,
    /// The profile that declared it.
    pub profile: String,
    /// What it is doing.
    pub state: super::ServiceState,
}

/// The certificate lemonfiber makes for a service before it first starts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceCertificate {
    /// Where the certificate is written, relative to the configuration directory.
    pub certificate: String,
    /// Where its private key is written, relative to the configuration directory.
    pub key: String,
    /// The host names it is made out to.
    pub names: Vec<String>,
}

/// A value written exactly as declared.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ServiceConstant {
    /// A string.
    Text(String),
    /// An integer.
    Integer(i64),
    /// A boolean.
    Boolean(bool),
}

/// The container fields an install record keeps for a service, beyond those every
/// record already holds.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceFields {
    /// The certificate lemonfiber makes for it, where it declares one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<super::ServiceCertificate>,
    /// Whether its configuration directory is read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_ro: Option<bool>,
    /// Its environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::BTreeMap<String, super::ServiceValue>>,
    /// Its memory ceiling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem_limit_mib: Option<u32>,
    /// Its further mounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<Vec<super::ServiceMount>>,
    /// The network it runs inside.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<super::ServiceNetwork>,
    /// How tightly it is held.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posture: Option<super::ServicePosture>,
    /// Its further published ports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish: Option<Vec<super::ServicePublish>>,
    /// The paths beneath its configuration directory it rebuilds when they are missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebuilt: Option<Vec<String>>,
    /// Where the whole configuration directory of the service it fronts lands in it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shares_config: Option<String>,
    /// Its scratch paths in memory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<String>>,
    /// Who it runs as, where declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_mode: Option<super::ServiceUserModeWritten>,
}

/// The two variables an image reads the operator's uid and gid from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceIdVariables {
    /// The variable it reads the operator's gid from.
    pub gid_env: String,
    /// The variable it reads the operator's uid from.
    pub uid_env: String,
}

/// A further path mounted from the service's own mount tree.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceMount {
    /// The relative path inside the mount tree.
    pub from: String,
    /// Whether it is mounted read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ro: Option<bool>,
    /// The absolute path inside the container.
    pub to: String,
}

/// The network a service runs inside instead of its own: its filler's, or a fence's.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceNetwork {
    /// Whether it fetches whatever address it is handed, and so runs inside a fence that
    /// refuses every private address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fenced: Option<bool>,
    /// The capability whose filler's network it runs inside.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
}

/// How tightly the container is held.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServicePosture {
    /// Six kernel capabilities, no privilege gained, a bounded process count.
    #[serde(rename = "standard")]
    Standard,
    /// A read-only root and no kernel capability at all, besides.
    #[serde(rename = "confined")]
    Confined,
}

/// Where one service comes from, as the stack declares it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceProvenance {
    /// The digest of the image that runs, where the stack names one.
    ///
    /// Beside the tag rather than instead of it: the tag is the version somebody reads,
    /// and the digest is the one image that version was when it was pinned, which is
    /// what is pulled and what somebody verifies against the registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The service's id, which is also its Compose service name.
    pub id: String,
    /// The image it runs, without a tag.
    pub image: String,
    /// The SPDX identifier of the licence it is published under.
    ///
    /// Stated rather than summarised as *open source*, because the identifier is what
    /// somebody checks against the project — and because the four in this stack are
    /// not interchangeable to anybody deciding what to do with what they run.
    pub license: String,
    /// What it is called in front of an operator.
    pub name: String,
    /// The exact tag this stack pins it at.
    ///
    /// Kept apart from the image rather than written as one reference, so that a
    /// caller comparing what is pinned against what a project has released is
    /// comparing versions rather than parsing them out of a string. The two are
    /// printed together for a person, because a version without the image it belongs
    /// to names nothing that can be fetched.
    pub pinned: String,
    /// The project it is built from.
    ///
    /// The whole point of the entry for anybody verifying: the licence is a string
    /// this stack wrote down, and this is where somebody goes to find out whether the
    /// project still agrees with it.
    pub upstream: String,
}

/// A further port a service publishes on its tier.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServicePublish {
    /// Where it answers inside the container, `port` where absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listens: Option<u16>,
    /// Where it is published on the host.
    pub port: u16,
}

/// A secret the manifest declares.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceSecretOf {
    /// The declared secret's name.
    pub secret: String,
}

/// An operator setting, with what is written while none is recorded.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceSetting {
    /// What is written while the operator has recorded no value.
    pub default: super::ServiceConstant,
    /// The values of other settings under which it is written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs: Option<std::collections::BTreeMap<String, Vec<super::ServiceConstant>>>,
    /// The values its service accepts, where it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<super::SettingChoice>>,
    /// What a recording gives the service in the operator's place, where only the
    /// operator holds the value and the service needs one to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recorded_with: Option<super::RecordedWith>,
    /// The setting's name.
    pub setting: String,
    /// The variable an earlier release kept its value under in the stack's environment
    /// file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was: Option<String>,
}

/// What one service is actually doing.
///
/// Ordered from worst to best, so a form's condition is the minimum across its
/// services and needs no comparison table. The declaration order is therefore
/// load-bearing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServiceState {
    /// Exited without being asked to, and is not coming back on its own.
    #[serde(rename = "failed")]
    Failed,
    /// Exiting and restarting repeatedly.
    #[serde(rename = "crash-looping")]
    CrashLooping,
    /// Running, and its own probe says it is not working.
    #[serde(rename = "unhealthy")]
    Unhealthy,
    /// No container exists.
    #[serde(rename = "absent")]
    Absent,
    /// A container exists and is not running.
    #[serde(rename = "stopped")]
    Stopped,
    /// Running, and still inside its probe's start period.
    #[serde(rename = "starting")]
    Starting,
    /// Running, and declaring no probe by which to know more.
    ///
    /// Distinct from healthy on purpose. A service that cannot be asked is not
    /// a service that answered, and rendering the two identically is how a
    /// dashboard comes to be believed about something it never checked.
    #[serde(rename = "running")]
    Running,
    /// Running and passing its own probe.
    #[serde(rename = "healthy")]
    Healthy,
    /// The operating system owns this one, not lemonfiber.
    #[serde(rename = "host-managed")]
    HostManaged,
}

/// The address of a capability's filler.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceUrlOf {
    /// The medium the filler asked for files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    /// The upstream's own API, where the ask reaches the upstream an adapter fronts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    /// The capability asked for.
    pub url_of: String,
}

/// A user mode a manifest writes by name.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServiceUserMode {
    /// No user: the image decides.
    #[serde(rename = "image")]
    Image,
    /// `PUID` and `PGID` set to the operator's.
    #[serde(rename = "puid-env")]
    PuidEnv,
    /// The operator's uid and gid as the container's user.
    #[serde(rename = "uid")]
    Uid,
}

/// How a manifest writes [`UserMode`]: a mode by name, or the variables an image reads
/// the operator's ids from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ServiceUserModeWritten {
    /// A mode by name.
    ServiceUserMode(super::ServiceUserMode),
    /// The variables its image reads the operator's uid and gid from.
    ServiceIdVariables(super::ServiceIdVariables),
}

/// One environment variable's value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ServiceValue {
    /// A constant.
    ServiceConstant(super::ServiceConstant),
    /// An operator setting.
    ServiceSetting(super::ServiceSetting),
    /// A capability's filler's address.
    ServiceUrlOf(super::ServiceUrlOf),
    /// A declared secret.
    ServiceSecretOf(super::ServiceSecretOf),
}

/// What a caller offers at the door.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SessionBody {
    /// The claim token an invitation's join link carries, where this is a claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
    /// Who they say they are, where they say so. Absent from an operator signing in
    /// with the machine's own password, which is nobody's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What was typed: at a claim, the password they chose.
    pub password: String,
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
pub enum SettingChoice {
    /// A value that is always a choice.
    ServiceConstant(super::ServiceConstant),
    /// Values that are choices only where `needs` is met.
    SettingChoiceNeeding(super::SettingChoiceNeeding),
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

/// One setting a change moves.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettingMove {
    /// What moving it costs, where it costs anything.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consequence: Option<String>,
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
}

/// One answer, tagged by the step it belongs to.
///
/// Read back as well as built, because a surface that is not in this process
/// submits one: the tag is the step, so an answer names the question it belongs
/// to rather than arriving as a field a reader has to guess the meaning of.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SetupAnswerBody {
    /// The protocol choice.
    #[serde(rename = "protocols")]
    Protocols(super::Protocols),
    /// Whether a VPN carries the torrents, and where none does, that the operator
    /// accepted what that means.
    #[serde(rename = "vpn")]
    Vpn(super::SetupVpn),
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
    Library(super::Library),
    /// How much the operator wants to be told about.
    #[serde(rename = "notifications")]
    Notifications(super::Appetite),
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

/// How a setup run ended.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    SetupVpnCarrying(super::SetupVpnCarrying),
    /// Carrying with nothing given, or nothing carrying it.
    SetupVpnSaid(super::SetupVpnSaid),
}

/// Carrying, with what was given.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SetupVpnCarrying {
    /// A VPN carries it, through the egress guard set up with what was given.
    #[serde(rename = "carrying")]
    Carrying(super::SetupGuarded),
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
pub enum Shape {
    /// The full walk: search, grab, download, import, and see it in the library.
    #[serde(rename = "pipeline")]
    Pipeline,
    /// The walk for a stack that acquires nothing: point at media already on disk and
    /// confirm the media server can see it.
    #[serde(rename = "library-only")]
    LibraryOnly,
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
}

/// Where a figure for the line came from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Source {
    /// The operator gave it, presumably off the plan they pay for.
    #[serde(rename = "declared")]
    Declared,
    /// The fastest the stack has been seen to move, which is what it achieved
    /// rather than what the line is sold as.
    #[serde(rename = "observed")]
    Observed,
}

/// What one line of the accounting is about.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "of")]
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
pub enum StackProtocol {
    /// Needs a Usenet provider.
    #[serde(rename = "usenet")]
    Usenet,
    /// Needs a VPN and a torrent client.
    #[serde(rename = "torrent")]
    Torrent,
}

/// What updating the stack would change, or what a run of it came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackUpdateReport {
    /// What became of each service the run reached, in the order it reached them.
    pub applied: Vec<super::UpdateApplied>,
    /// Where the backup taken before anything moved was written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<String>,
    /// What the release that brought these pins changed.
    ///
    /// The stack this would move to is the one this build carries, and the release
    /// that carried this build is what says why it moved. An operator weighing a
    /// stack update is weighing that, and being shown only which image numbers go up
    /// is being shown the arithmetic rather than the reason.
    pub changelog: super::Notes,
    /// What would move, and what taking each step means.
    pub changes: Vec<super::UpdateChange>,
    /// Whether the steps were agreed to, or only shown.
    pub confirmed: bool,
    /// Each of lemonfiber's own guards this run moved onto its plugin, or would move once
    /// confirmed, for good and with no yes of its own; one it could not move stays on
    /// offer and says why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guards: Option<Vec<super::StackMoveTaken>>,
    /// Why the stack is not as the run found it, where it is not.
    ///
    /// Two runs end that way and an operator has the same thing to do about either:
    /// one that met a service which would not come back and stopped there, and one
    /// where every step succeeded and the stack would not start again afterwards.
    /// The second is not a failure of the update — `state` still says `Updated`,
    /// because it is — but the stack came down for the capture and something has to
    /// say that it is still down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub halted: Option<String>,
    /// What the download clients are still working on, named so an operator can
    /// tell whether the thing they have been waiting for is among them.
    pub in_flight: Vec<String>,
    /// Each move in place this run took, or tried to take and could not, with why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moved: Option<Vec<super::StackMoveTaken>>,
    /// Each part this release installs as a plugin that the stack still runs as its own
    /// service after this run: a move in place on offer, which a confirmed update takes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moves: Option<Vec<super::MoveOffered>>,
    /// The offer this answers: every step, from the release each service stands on to
    /// the one it would move to, named so that an update carrying it back takes those
    /// steps or is refused.
    pub offer: String,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Stack files the operator had edited, left as they set them rather than
    /// overwritten with this build's own, each with the change that was held back.
    pub stack_edits: Vec<super::StackEdit>,
    /// The one word the run comes to.
    pub state: super::UpdateState,
    /// The operator's overlay and every service it changes, where there is one: merged
    /// over the project this run would start, and held to nothing lemonfiber vets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unvetted: Option<super::Unvetted>,
}

/// A stage in an item's journey, ordered from "nobody asked for it" to "playable". The
/// declaration order is the pipeline order, so one stage compares less than a later one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Stage {
    /// Nobody has asked for it — no \*arr is monitoring it.
    #[serde(rename = "not-monitored")]
    NotMonitored,
    /// Monitored, waiting to be searched for.
    #[serde(rename = "monitored")]
    Monitored,
    /// A search is running.
    #[serde(rename = "searching")]
    Searching,
    /// Releases were found by an indexer.
    #[serde(rename = "found")]
    Found,
    /// A release was sent to the download client.
    #[serde(rename = "grabbed")]
    Grabbed,
    /// The download is in progress.
    #[serde(rename = "downloading")]
    Downloading,
    /// The download finished.
    #[serde(rename = "downloaded")]
    Downloaded,
    /// The \*arr is importing it to the library.
    #[serde(rename = "importing")]
    Importing,
    /// It was imported to the library on disk.
    #[serde(rename = "imported")]
    Imported,
    /// It is visible and playable in the media server.
    #[serde(rename = "available")]
    Available,
}

/// Why an item is not moving.
///
/// Ordered by how much of the operator's attention each deserves, worst first, so
/// a summary that leads with the worst category needs no second ranking.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Stall {
    /// Fetched over and over: import is failing silently and being retried, which
    /// spends bandwidth and Usenet allowance indefinitely while looking normal.
    #[serde(rename = "redownload-loop")]
    RedownloadLoop,
    /// The same item has failed to import more than once. Structural — it will not
    /// resolve itself.
    #[serde(rename = "repeated-import-failure")]
    RepeatedImportFailure,
    /// Downloaded successfully and never imported. The failure nobody owns: the
    /// client considers it finished, the curator never picked it up, and from each
    /// service's own perspective there is nothing wrong.
    #[serde(rename = "completed-not-imported")]
    CompletedNotImported,
    /// On disk, and no curator knows about it.
    #[serde(rename = "orphaned")]
    Orphaned,
    /// No progress at all beyond the threshold.
    #[serde(rename = "stalled-download")]
    StalledDownload,
    /// Monitored, never grabbed — nothing matched, or the indexers are answering
    /// with nothing.
    #[serde(rename = "waiting-indefinitely")]
    WaitingIndefinitely,
    /// Moving, but slowly enough to be worth knowing about. Deliberately its own
    /// category rather than a stall: something still arriving needs patience, not
    /// intervention, and reporting the two alike is how a queue check gets muted.
    #[serde(rename = "slow")]
    Slow,
}

/// Where a proposed change stands once it has been read against what is in force.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Stance {
    /// The setting already holds what was asked for. Nothing to apply, nothing to
    /// confirm, and the file is not touched — a write of the same value would move
    /// the file's own timestamp and read afterwards as an edit somebody made.
    #[serde(rename = "unchanged")]
    Unchanged,
    /// Staged and not applied: nothing has been written. Either it is consequential
    /// and nobody has said yes to it yet, or it was rehearsed.
    ///
    /// The default, because nothing having happened yet is what every act starts as,
    /// and a report built and not filled in should say that rather than claim it acted.
    #[serde(rename = "pending")]
    Pending,
    /// Turned away. It cannot be applied safely, the file is exactly as it was, and
    /// the refusal says why.
    #[serde(rename = "blocked")]
    Blocked,
    /// Written.
    #[serde(rename = "applied")]
    Applied,
}

/// One Compose project on this machine that is not lemonfiber's.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StandingReport {
    /// The Compose project name.
    pub project: String,
    /// Its containers, by service name.
    pub services: Vec<super::OccupantReport>,
}

/// Where a title a member's search found stands in the house.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Stands {
    /// All of it is here.
    #[serde(rename = "here")]
    Here,
    /// Some of it is here, and the rest may be asked for.
    #[serde(rename = "partly-here")]
    PartlyHere,
    /// Somebody asked for it and it is coming.
    #[serde(rename = "on-its-way")]
    OnItsWay,
    /// It may be asked for.
    #[serde(rename = "askable")]
    Askable,
}

/// Work that outlives the request that started it: its name, and what it was.
///
/// The action is carried beside the name because a client holding several has to
/// tell them apart, and asking it to remember which name it gave which request is
/// asking it to keep a second copy of what this already knows.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Started {
    /// The action that was asked for, as it was named.
    pub action: String,
    /// The name to ask what became of this work by.
    pub job: String,
}

/// What each service is doing, and what that adds up to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatusReport {
    /// The forms the running services are up for, in the order the stack declares them.
    ///
    /// Read from the whole stack whichever forms were asked about. A form counts while
    /// every service it holds has been started and none has been stopped, and one
    /// wholly inside a broader form that counts is left out. Each service names the ones
    /// it is running for.
    pub active_forms: Vec<String>,
    /// What the services amount to, as one word.
    ///
    /// For the whole stack, counted over what the active forms hold and whatever else
    /// is there, so a stack running part of itself on purpose reads as active. With no
    /// form up, and for the forms asked about, counted over every service listed.
    pub condition: super::Condition,
    /// How long each way of acting on these services takes them away for.
    ///
    /// Here so that a surface can say it before it asks the operator to confirm,
    /// which is the only moment saying it is any use.
    pub disturbs: super::Disturbances,
    /// What those forms left out for want of a configured provider, service by service.
    ///
    /// Beside the services rather than among them: a service filtered out is the
    /// configuration being honoured, not a service that did not start.
    pub filtered: Vec<super::Filtered>,
    /// The forms asked about; empty means the whole stack was.
    pub forms: Vec<String>,
    /// Each service, worst first.
    ///
    /// A service the active forms filtered out is listed only while it is there; one
    /// that is not is in `filtered` and nowhere else.
    pub services: Vec<super::Service>,
    /// The containers running under this project that the stack never declared.
    ///
    /// Kept apart from the services rather than mixed in with them, because what is
    /// known about each is different: a service has a profile, a criticality and a
    /// description, and one of these has a name and a state and nothing else. Shown
    /// all the same — something running under this project's name that lemonfiber did
    /// not put there is the operator's business whether or not lemonfiber understands
    /// it.
    pub undeclared: Vec<super::Undeclared>,
    /// The services held back until the operator gives what they need to start, each
    /// with what it waits for and never a value.
    ///
    /// Beside the services, as what was filtered is: one that is not there is listed
    /// here and nowhere else, because it is waiting rather than failing to start.
    pub waiting: Vec<super::Waiting>,
}

/// What one step of a recipe came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StepCame {
    /// It was answered with what it should be, and took what it captures.
    #[serde(rename = "answered")]
    Answered,
    /// Its guard did not hold, so it was not made.
    #[serde(rename = "skipped")]
    Skipped,
    /// An earlier step did not come to what it should, so this one was not made.
    #[serde(rename = "not-reached")]
    NotReached,
    /// Nothing answered it.
    #[serde(rename = "unreachable")]
    Unreachable,
    /// It was not sent: its host stands for an address not out on the internet, or
    /// could not be resolved.
    #[serde(rename = "refused")]
    Refused,
    /// It was not sent: a value it carries may not go where it was going.
    #[serde(rename = "withheld")]
    Withheld,
    /// It was answered with something other than what it expects.
    #[serde(rename = "unexpected")]
    Unexpected,
    /// Its answer held nothing where a capture reads.
    #[serde(rename = "uncaptured")]
    Uncaptured,
    /// Its answer was larger than any answer a recipe reads.
    #[serde(rename = "oversized")]
    Oversized,
}

/// A walkthrough that stopped: where, why, what the services were saying, and what to do.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Stopped {
    /// What the services involved were saying at the time, shown inline rather than left
    /// for the operator to go and find — a fault report they have to research is a fault
    /// report they abandon.
    pub logs: Vec<String>,
    /// Why.
    pub reason: super::Reason,
    /// The one thing to try.
    pub remedy: String,
    /// The step it stopped at.
    pub step: super::WalkthroughStep,
}

/// The storage picture: what is free, when it runs out, and whether imports link.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Storage {
    /// Bytes free on the volume the services keep their configuration and databases
    /// on, which fills apart from the data volume where the two are different disks
    /// and stops every service when it does. A [`Reading`] for the same reason.
    pub config_free: super::DashboardReading,
    /// The time until the disk fills at the current rate of the queue draining
    /// onto it, or `None` where it is not projected to fill.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exhaustion: Option<super::Duration>,
    /// Bytes free on the data volume — a [`Reading`], since a volume that could
    /// not be read this refresh must not render as zero free.
    pub free: super::DashboardReading,
    /// Whether imports are linking or copying.
    pub hardlink: super::Hardlink,
}

/// Everything lemonfiber keeps on this machine, and what became of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Stored {
    /// What is on this machine that is not lemonfiber's to keep or remove.
    pub beside: Vec<super::StoredBeside>,
    /// Each thing kept, configuration first and then what can be made again.
    pub kept: Vec<super::Kept>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Whether this run removed any of it.
    pub removal: super::StoredRemoval,
    /// The two directories all of it lives under.
    pub roots: Vec<super::Root>,
}

/// Something on this machine that lemonfiber neither keeps nor removes.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StoredBeside {
    /// What it is.
    pub what: String,
    /// Whose it is, and why it is not lemonfiber's to take away.
    pub why: String,
}

/// Something a removal could not take away.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StoredLeft {
    /// The path that is still there.
    pub at: String,
    /// What the machine said about it, so it can be finished by hand.
    pub why: String,
}

/// Whether anything was removed on this run, and what became of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "state")]
pub enum StoredRemoval {
    /// Nobody asked. This is a listing.
    #[serde(rename = "not-asked")]
    NotAsked,
    /// Asked for without the agreement it takes, so nothing was touched.
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
    /// Carried out.
    #[serde(rename = "done")]
    Done {
        /// The directories that are gone.
        gone: Vec<String>,
        /// What could not be removed, each with the reason.
        left: Vec<super::StoredLeft>,
    },
}

/// Why playback here is likely to struggle, whatever app the household installs.
///
/// Present only where the preset in force asks for transcoding this platform cannot
/// do in hardware. It belongs to the guidance rather than to any one device: the
/// preset and the platform decide it between them, and every device in the table
/// meets it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Straining {
    /// What that preset asks of this machine, and what playback does where this
    /// machine cannot give it.
    pub caution: String,
    /// What makes it stop.
    pub instead: String,
    /// The preset in force, under the name it was chosen by.
    pub preset: String,
}

/// Which stream a log line arrived on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Stream {
    /// Standard output.
    #[serde(rename = "stdout")]
    Stdout,
    /// Standard error.
    #[serde(rename = "stderr")]
    Stderr,
}

/// One thing that is wrong, and why.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Stuck {
    /// What the service said was blocking it, in its own words, where it said
    /// anything. A permission denial from an import log is worth more than any
    /// interpretation of it, and it is the difference between "stuck" and
    /// something an operator can fix.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking: Option<String>,
    /// How long it has been that way, in seconds — what turns "stuck" into a
    /// sentence an operator can weigh.
    pub held_for: u64,
    /// How many items this stands for. One in the ordinary case; more where they
    /// share a cause and the cause is what is wrong — twenty downloads stopped by
    /// a full disk are one thing to fix, and twenty alerts about it are how an
    /// operator learns to mute the queue check.
    pub items: u64,
    /// Which item — or, where several share one cause, that cause.
    pub name: String,
    /// What is wrong with it.
    pub stall: super::Stall,
}

/// One stuck item queue health found, named so it links straight to its own trace.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StuckEntry {
    /// The curator whose queue is holding it.
    pub service: String,
    /// The stage its download is stuck at.
    pub stage: super::Stage,
    /// The item's title — the term a `trace` searches by.
    pub title: String,
}

/// The items whose downloads are stuck, across the curators — the landing point for "N
/// items stuck" that queue health reports, each entry naming the item so the operator
/// goes straight to its per-item trace rather than to a count to investigate.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StuckReport {
    /// Whether a curator's queue could not be read, so the list may be short — reported
    /// rather than read as "nothing stuck", the same honesty a trace keeps.
    pub incomplete: bool,
    /// The stuck items, each linkable to its trace.
    pub items: Vec<super::StuckEntry>,
}

/// One service standing in for another, worked out before anything is written.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Substitution {
    /// Every service that asks for it, so the reach of the change is visible.
    pub asked_by: Vec<String>,
    /// The capability whose filler changes.
    pub capability: String,
    /// Each kind of thing the part it stops holds that its replacement is not carried,
    /// read from what each contract declares its filler holds.
    pub leaves_behind: Vec<super::LeftBehind>,
    /// What this would leave with nothing filling it, each naming what asked.
    ///
    /// The one thing an operator cannot find out afterwards. A service filling two
    /// capabilities is replaced for one of them, and the other stops being filled —
    /// which is a working stack becoming a broken one, on a change that reads as
    /// swapping like for like.
    pub leaves_unfilled: Vec<super::Unfilled>,
    /// What would fill it.
    pub now: String,
    /// The setting the change writes.
    pub setting: String,
    /// Each part the change brings back to its form, started again.
    pub starts: Vec<String>,
    /// Each part the change takes out of its form, stopped with its configuration and
    /// data kept where they are.
    pub stops: Vec<String>,
    /// What fills it now, where anything does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was: Option<String>,
    /// What the operator said about the choice, where they said anything.
    ///
    /// Read back as the choice's own `why` wherever the choice is read, and absent
    /// where nothing was said: nothing supplies a reason on the operator's behalf.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// What substituting one service for another would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubstitutionReport {
    /// What this reading names itself, so a choice answering it can say which reading
    /// it answered.
    ///
    /// Named part by part — the choice itself, what fills the capability now, what asks
    /// for it, and what the change would leave unfilled — so a choice refused because the
    /// wiring moved is told which of those moved.
    pub agreement: String,
    /// Whether it was written, or only worked out.
    ///
    /// A run that only says what it would do writes nothing and reports the same
    /// answer, so the two are told apart here rather than by the caller remembering
    /// which flags it passed.
    pub applied: bool,
    /// Each of lemonfiber's own guards this act moved onto its plugin after its own
    /// change, or on a reading would move, for good and with no yes of its own; one it
    /// could not move stays on offer and says why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guards: Option<Vec<super::StackMoveTaken>>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// The change itself, and what it would leave with nothing filling it.
    pub substitution: super::Substitution,
    /// Each part the change was to stop or start and could not, with why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unran: Option<Vec<super::SubstitutionUnran>>,
    /// The operator's overlay and every service it changes, where there is one: merged
    /// over the project this choice would write, and held to nothing lemonfiber vets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unvetted: Option<super::Unvetted>,
}

/// A part a substitution was to stop or start and could not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubstitutionUnran {
    /// Why it was not.
    pub because: String,
    /// The part's service.
    pub service: String,
}

/// What a watch saw, once the data root it was guarding was lost.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SupervisionReport {
    /// The forms that were being watched, and are now stopped.
    pub forms: Vec<String>,
    /// Why the watch ended: the data root vanished, or a different volume took
    /// its place — or, on a run that only said what a watch would do, that nothing
    /// was watched at all.
    pub reason: String,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Whether stopping the services succeeded.
    pub stopped: bool,
    /// The watch this run would have kept, where it only said what it would do.
    ///
    /// A guard is the one command with no ending of its own, so a rehearsal of it
    /// cannot be the command with its last step left out — it would hold until the
    /// drive was pulled. What it answers with is this instead, and the fields above
    /// then describe a watch that never began: nothing ended, and nothing was
    /// stopped. Absent on every watch that actually ran.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub would: Option<super::Vigil>,
}

/// How well a device is served.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Support {
    /// An official app that works. Most people, most of the time.
    #[serde(rename = "good")]
    Good,
    /// It works, with something worth knowing before starting.
    #[serde(rename = "workable")]
    Workable,
    /// Poorly served. Said plainly, with somewhere else to go.
    #[serde(rename = "poor")]
    Poor,
    /// No installation, works anywhere, and is never unavailable.
    #[serde(rename = "fallback")]
    Fallback,
}

/// What narrowing the active set moved.
///
/// Three lists rather than a before and an after, because the operator's question
/// is not "what is running now" — they can ask that — but "what did that do". The
/// middle list is the one that makes the verb worth having: it is the promise that
/// a download in flight was not interrupted to change the shape of the stack
/// around it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Switched {
    /// Left running: the new closure holds them too, so nothing here asked them to
    /// stop. Not a promise that nothing touched them — Compose recreates a container
    /// whose configuration changed — but a promise that narrowing did not.
    pub kept: Vec<String>,
    /// Started, because the new closure holds them and they were not up.
    pub started: Vec<String>,
    /// The exact Compose invocation that stopped what fell outside, so a switch is
    /// no more a matter of trust than any other action. Absent where nothing had
    /// to stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_command: Option<Vec<String>>,
    /// Stopped, because the new closure does not hold them.
    pub stopped: Vec<String>,
}
