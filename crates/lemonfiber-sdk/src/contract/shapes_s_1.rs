/// How much of the stack a backup covers.
///
/// Whole-stack is the common case, but restoring one service is often what is
/// actually wanted — one curator's configuration mangled while the rest is fine —
/// so the scope is recorded in the archive and honoured on the way back.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "scope")]
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged, deserialize_with = "crate::open::unrecognised::<Scope, _>")]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Scope {
    const TAG: Option<&str> = Some("scope");
    const KNOWN: &[&str] = &["whole_stack", "service", "existing"];
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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<SeedSeverity, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for SeedSeverity {
    const TAG: Option<&str> = Some("severity");
    const KNOWN: &[&str] = &["informational", "warning"];
}

/// How one connection turned out after a seed pass.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "state")]
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<SeedState, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for SeedState {
    const TAG: Option<&str> = Some("state");
    const KNOWN: &[&str] = &[
        "wired",
        "restarted",
        "already-wired",
        "drifted",
        "kept",
        "stale",
        "conflicted",
        "adopted",
        "unmanaged",
        "would-wire",
        "would-adopt",
        "observed",
        "unmatched",
        "skipped",
        "failed",
        "refused",
    ];
}

/// Where one completed download stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "standing")]
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<SeedingStanding, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for SeedingStanding {
    const TAG: Option<&str> = Some("standing");
    const KNOWN: &[&str] = &["never_imported", "seeding", "left_alone"];
}

/// Where a copy of lemonfiber stands, in the words the specification uses.
///
/// Nothing known is the default, and it is the right one: a report built before
/// anything has been read has not established that this copy is current, and a
/// default that said so would be a claim made by an empty value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
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
#[non_exhaustive]
pub enum ServiceConstant {
    /// A string.
    Text(String),
    /// An integer.
    Integer(i64),
    /// A boolean.
    Boolean(bool),
    /// A value this client was not generated with, as it arrived.
    Unrecognised(serde_json::Value),
}

/// The container fields an install record keeps for a service, beyond those every
/// record already holds.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceFields {
    /// The arguments its image's own entrypoint is handed, in order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
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
#[non_exhaustive]
pub enum ServicePosture {
    /// Six kernel capabilities, no privilege gained, a bounded process count.
    #[serde(rename = "standard")]
    Standard,
    /// A read-only root and no kernel capability at all, besides.
    #[serde(rename = "confined")]
    Confined,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// How a manifest writes [`UserMode`]: a mode by name, or the variables an image reads
/// the operator's ids from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ServiceUserModeWritten {
    /// A mode by name.
    ServiceUserMode(super::ServiceUserMode),
    /// The variables its image reads the operator's uid and gid from.
    ServiceIdVariables(super::ServiceIdVariables),
    /// A value this client was not generated with, as it arrived.
    Unrecognised(serde_json::Value),
}

/// One environment variable's value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ServiceValue {
    /// A constant.
    ServiceConstant(super::ServiceConstant),
    /// An operator setting.
    ServiceSetting(super::ServiceSetting),
    /// A capability's filler's address.
    ServiceUrlOf(super::ServiceUrlOf),
    /// A declared secret.
    ServiceSecretOf(super::ServiceSecretOf),
    /// A value this client was not generated with, as it arrived.
    Unrecognised(serde_json::Value),
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
#[non_exhaustive]
pub enum SettingChoice {
    /// A value that is always a choice.
    ServiceConstant(super::ServiceConstant),
    /// Values that are choices only where `needs` is met.
    SettingChoiceNeeding(super::SettingChoiceNeeding),
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
