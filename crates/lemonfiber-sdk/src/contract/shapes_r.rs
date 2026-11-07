/// Whether a walk through setup was carried out or rehearsed, written as the bare
/// `true` or `false` every other report writes its `rehearsed` as.
///
/// Two words rather than a fourth switch on a report that already holds three, which
/// is a report somebody reads by remembering which `true` means what. On the wire it
/// is the same boolean every report carries, so a client reads one shape.
pub type Ran = bool;

/// What one age limit comes to, in the certificates named for it.
///
/// Both sides, because either alone misleads. What is allowed without what is held
/// back reads as a limit that stops nothing; what is held back without what is allowed
/// reads as a limit that stops everything.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rated {
    /// The certificates at the highest age this limit still lets through.
    ///
    /// Empty where the table names nothing at or below the limit, which is a limit
    /// that lets nothing rated through at all.
    pub allows: Vec<String>,
    /// Whether these came from lemonfiber's own mapping because the media server's
    /// table named no certificates.
    ///
    /// Carried rather than hidden: a certificate said to be this household's when it
    /// is this program's is the kind of claim a parent would act on.
    pub fell_back: bool,
    /// The certificates at the lowest age this limit holds back.
    ///
    /// Empty where the table names nothing above the limit, which is a limit that
    /// holds nothing rated back at all.
    pub holds_back: Vec<String>,
}

/// Where a month stands against a declared cap.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Reached {
    /// Comfortably inside it.
    #[serde(rename = "within")]
    Within,
    /// Close enough that there is still time to do something.
    #[serde(rename = "warning")]
    Warning,
    /// Reached, and the declared behaviour applies.
    #[serde(rename = "exceeded")]
    Exceeded,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What one link reaches, and how that was settled.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "how")]
#[non_exhaustive]
pub enum Reaches {
    /// An ask for a capability.
    #[serde(rename = "asked")]
    Asked {
        /// The capability asked for.
        capability: String,
        /// The medium the filler it reaches files, where the ask is narrowed to one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        files: Option<String>,
        /// The upstream's own API the ask talks in, or every API it talks in any of,
        /// where it reaches the upstream an adapter fronts rather than the adapter.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        native: Option<super::Apis>,
        /// Where each service that claims it came from: this build's stack, or a
        /// named plugin.
        ///
        /// Every claimant rather than only what the ask reaches, because a contest
        /// reaches nothing and is exactly where an operator most needs to know which of
        /// the names in front of them is not the stack's.
        origins: std::collections::BTreeMap<String, super::ValueOrigin>,
        /// What the ask reaches — empty where nothing fills it or a contest stands.
        services: Vec<String>,
        /// How it was settled.
        settled: super::WiringSettled,
    },
    /// A link deliberately kept to a named service, shown as the exception it is.
    #[serde(rename = "by-name")]
    ByName {
        /// The service named.
        service: String,
        /// Why it is by name.
        why: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// One service reading a setting or a secret.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reader {
    /// What the service's manifest declares while no value is recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Whether those settings hold what it needs now.
    pub met: bool,
    /// The values of other settings under which the variable is written: each setting
    /// with the values it has to hold one of.
    pub needs: std::collections::BTreeMap<String, Vec<String>>,
    /// The service.
    pub service: String,
    /// The variable it is read into.
    pub variable: String,
}

/// Why a walkthrough could not go on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Reason {
    /// No indexer is configured, so there is nothing to search. Not a failure of the
    /// walk — a reason not to offer it at all.
    #[serde(rename = "no-indexers")]
    NoIndexers,
    /// The search could not be run: the indexers, or the service holding them, would not
    /// answer.
    #[serde(rename = "indexers-failed")]
    IndexersFailed,
    /// The search ran cleanly and matched nothing. An entirely different problem from
    /// the one above, and the one operators most often mistake for it.
    #[serde(rename = "nothing-matched")]
    NothingMatched,
    /// Releases exist, but none meets the chosen quality preset. The stack is working;
    /// the preset and what is out there disagree.
    #[serde(rename = "none-met-the-preset")]
    NoneMetThePreset,
    /// Torrents are in play and the tunnel is not verified up. Nothing is grabbed.
    #[serde(rename = "tunnel-down")]
    TunnelDown,
    /// The release was never handed to a download client.
    #[serde(rename = "not-grabbed")]
    NotGrabbed,
    /// The download stopped making progress.
    #[serde(rename = "stalled")]
    Stalled,
    /// The download finished and the library manager would not take it.
    #[serde(rename = "import-failed")]
    ImportFailed,
    /// There is no media server in the running form to make it playable.
    #[serde(rename = "no-media-server")]
    NoMediaServer,
    /// It was imported, the media server was told, and it still cannot be found there.
    #[serde(rename = "not-visible")]
    NotVisible,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where the disk stands, what is on it, and what could be got back.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reckoning {
    /// What this offer names itself, so an answer to it can say which offer it was
    /// answering. The answer is this name, and nothing else is a yes to a cleanup.
    pub agreement: String,
    /// The completed downloads, each with where it stands and what removing it
    /// would cost.
    pub candidates: Vec<super::Candidate>,
    /// Where the room went, one line per tree plus the services' own files, and
    /// one line for what is committed but has not landed yet.
    pub consumption: Vec<super::Consumption>,
    /// Whether new acquisitions are halted to keep the services writable.
    pub halted: bool,
    /// The imports that stopped part-way, with what is on disk for each.
    pub interrupted: Vec<super::Interrupted>,
    /// Where the stack stands, which is where its worst volume stands.
    pub level: super::Level,
    /// The files far enough out of line with the rest to be worth pointing at.
    pub outsized: Vec<super::Outsized>,
    /// What of that room could be got back, and what each would cost.
    ///
    /// A second reading of bytes already counted above rather than more of them: a
    /// seeding torrent's file is in the tree it lives in *and* here. Summing the
    /// two lists together would double what is on the disk, which is the mistake
    /// this whole module is arranged to avoid.
    pub reclaimable: Vec<super::Consumption>,
    /// What became of an answered cleanup, where the offer was answered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reclaimed: Option<super::Reclaimed>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// The volumes watched. Either filling stops the stack, so both are reported
    /// whether or not they are the same drive.
    pub volumes: Vec<super::Volume>,
}

/// What getting a line's room back costs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Reclaim {
    /// Only by deciding to lose something you chose to keep.
    #[serde(rename = "by_losing_content")]
    ByLosingContent,
    /// Nothing to reclaim: it is being written right now.
    #[serde(rename = "in_progress")]
    InProgress,
    /// Yes, and it costs standing with the trackers it came from.
    #[serde(rename = "at_the_cost_of_ratio")]
    AtTheCostOfRatio,
    /// Yes, and it costs nothing — the usual easy win.
    #[serde(rename = "the_easy_win")]
    TheEasyWin,
    /// Yes: the unpacked copy is the one being used.
    #[serde(rename = "already_have_it")]
    AlreadyHaveIt,
    /// A little, and rarely worth it.
    #[serde(rename = "marginally")]
    Marginally,
    /// No, because you said so.
    #[serde(rename = "you_said_not")]
    YouSaidNot,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What became of an answered cleanup.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reclaimed {
    /// What they occupied.
    pub bytes: u64,
    /// The paths that were taken, or would have been in a rehearsal.
    pub gone: Vec<String>,
    /// What could not be taken, and what the platform said about it.
    pub left: Vec<super::SpaceLeft>,
    /// Whether this was a rehearsal. Rehearsed, `gone` and `bytes` are what would have
    /// been taken and nothing was: no room was freed.
    pub rehearsed: bool,
}

/// One record carried across, or that would be.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecordReport {
    /// What kind of record it is, in the plural a person reads.
    pub kind: String,
    /// What it is called.
    pub name: String,
    /// The service it belongs to.
    pub service: String,
}

/// What a recording gives a service in the operator's place: a constant reaching no third
/// party, or, written `"minted"`, a value made for that run alone.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum RecordedWith {
    /// A string.
    Text(String),
    /// An integer.
    Integer(i64),
    /// A boolean.
    Boolean(bool),
    /// A value this client was not generated with, as it arrived.
    Unrecognised(serde_json::Value),
}

/// A named front door that is not one, and why it is not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Refusal {
    /// Why this stack will not send a household there.
    pub because: String,
    /// What the operator recorded, as they wrote it.
    pub named: String,
}

/// What was said when one request was turned down.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Refused {
    /// When it was turned down, so somebody reading it later knows which answer this
    /// was rather than assuming it is the newest.
    ///
    /// Absent where the machine's clock could not be written as a date, which is a
    /// refusal worth keeping the words of and not worth losing them over.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// Whether nobody ruled on it and the period the household agreed to closed it,
    /// rather than an operator turning it down.
    ///
    /// The words are then this program's own, which is why the two are told apart here
    /// and not left to be read off the sentence: an operator scanning for what happened
    /// while they were away is looking for exactly the ones nobody answered, and a
    /// member is owed the difference between having been refused and having run out.
    ///
    /// Absent from every record written before a household could arrange this, which is
    /// the right reading of them — they are all somebody's own refusals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// Why, in the words it was turned down in.
    pub reason: String,
    /// Whether the words have been carried to whoever asked, and where to.
    ///
    /// Absent until the one attempt has been made. **This is what makes a second one
    /// impossible rather than unlikely**: carrying happens only where this is absent,
    /// and it is written whether anything was reached or not — so a member with nowhere
    /// to send to is asked about once, and a household is never told the same thing
    /// twice by a run that happened to be started twice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub told: Option<super::Passed>,
}

/// A device key registered.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegisteredDeviceKey {
    /// The app that made it, as `LEMONFIBER_DEVICE_KEY_APPS` spells an identity.
    pub app: String,
    /// Where it lives, as its attestation says.
    pub backing: super::DeviceKeyBacking,
    /// The SHA-256 of its public half's `SubjectPublicKeyInfo`, in lower-case hex.
    pub fingerprint: String,
    /// The name it was registered under.
    pub name: String,
    /// When it was registered.
    pub registered: String,
}

/// One release, and everything the record holds about it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Release {
    /// The version whose goals this tag carried, where that is not its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carried: Option<String>,
    /// What it set out to deliver, in the words the version was staged under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivers: Option<String>,
    /// The changes, gathered by what kind of change each is.
    pub groups: Vec<super::Group>,
    /// The version this one patched, where it is a patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patches: Option<String>,
    /// The day it was published, where the record of it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub released_on: Option<String>,
    /// The tag it was cut from.
    pub tag: String,
    /// Whether anything in it is a change an operator would notice.
    pub user_facing: bool,
    /// The version, without the tag's leading letter.
    pub version: String,
    /// Why it was withdrawn, where it was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawn: Option<String>,
}

/// One release as a listing shows it: everything but what it changed.
///
/// Kept apart from [`Release`] rather than being it with the entries left out,
/// because the two are read for different things. A listing answers which releases
/// there have been and which of them was taken back; only the one being read needs
/// to carry every line of what it changed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReleaseSummary {
    /// What it set out to deliver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivers: Option<String>,
    /// The version this one patched, where it is a patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patches: Option<String>,
    /// The day it was published, where the record of it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub released_on: Option<String>,
    /// Whether anything in it is a change an operator would notice.
    pub user_facing: bool,
    /// The version.
    pub version: String,
    /// Why it was withdrawn, where it was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawn: Option<String>,
}

/// A restore whose archive was taken against a different data root than the one
/// configured now, so its stored paths would land where nothing exists.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Relocation {
    /// The data root configured now.
    pub now: String,
    /// The data root the archive was taken against.
    pub was: String,
}

/// One thing the operator can do about a problem.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Remedy {
    /// The action, phrased as something to do rather than something to know.
    pub action: String,
    /// Where to look, when that helps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// A service this stack used to carry, and what became of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemovedService {
    /// The id it was declared under, which is the name an operator will look for.
    pub id: String,
    /// Why it went.
    pub reason: String,
    /// The stack version whose catalogue stopped carrying it.
    pub removed_in: String,
    /// What took its place, where anything did.
    ///
    /// Absent is an answer and the commonest one: most things that go are not
    /// replaced, and a record that named the nearest surviving service to avoid an
    /// empty field would be pointing an operator at something that does not do the
    /// job they are looking for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<String>,
}

/// One repair lemonfiber could carry out.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Repair {
    /// The check whose finding this answers, as the finding names it.
    pub check: String,
    /// What it would do, in the words the operator will read before confirming.
    pub does: String,
    /// What else changes if it does.
    ///
    /// Stated before it is confirmed and never afterwards, because an effect an operator
    /// learns about after the fact is not something they agreed to. Empty where a repair
    /// touches nothing but the thing it names.
    pub effects: Vec<String>,
    /// Whether carrying it out is recorded well enough to be undone.
    ///
    /// A repair that cannot be reversed is still worth offering — restarting a container
    /// is not undoable and is usually right — but the operator confirming one deserves to
    /// know which kind they are agreeing to.
    pub reversible: bool,
}

/// How a repair turned out, once the check that raised the finding has been asked again.
///
/// Deliberately not a boolean. "It ran" and "it worked" are different claims, and a model
/// that cannot tell them apart will eventually report the first as the second.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "outcome")]
#[non_exhaustive]
pub enum RepairOutcome {
    /// It ran, and the check now passes.
    #[serde(rename = "fixed")]
    Fixed,
    /// It ran, and the check still fails.
    #[serde(rename = "fix_failed")]
    FixFailed,
    /// It stopped partway, leaving this.
    ///
    /// Named precisely rather than as "failed": a half-applied change is a different
    /// state to be in from an unchanged one, and the operator has to know which they are
    /// looking at before they try anything else.
    #[serde(rename = "stopped")]
    Stopped {
        /// What the machine is now in, said plainly.
        leaving: String,
    },
    /// Not carried out, because the operator said no.
    #[serde(rename = "declined")]
    Declined,
    /// Refused, because it would have written over something changed by hand.
    #[serde(rename = "would_overwrite")]
    WouldOverwrite,
    /// Not carried out, because the operator declared the area it would write
    /// unmanaged.
    ///
    /// Apart from [`Self::WouldOverwrite`], which is lemonfiber declining to write over
    /// a change it can see. This is lemonfiber obeying an instruction it was given, and
    /// telling somebody the first when they wrote the second would send them looking
    /// for a change they did not make.
    #[serde(rename = "unmanaged")]
    Unmanaged,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// What a repairing run offered, and what it did.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RepairReport {
    /// Whether this run was allowed to act at all.
    pub acted: bool,
    /// What this offer is, so consent given for it can name which offer it read.
    ///
    /// Carried on every report rather than only on the ones that offer something: a
    /// surface that has to look for it is a surface that can fail to find it, and an
    /// offer of nothing is still an offer somebody may agree to nothing of.
    pub agreement: String,
    /// What has been tried too often to keep offering.
    ///
    /// Said rather than passed over. A repair that quietly stopped being offered leaves
    /// the operator watching a fault nobody mentions any more, which is worse than being
    /// told plainly that this is past what lemonfiber can work out.
    pub beyond: Vec<super::Beyond>,
    /// What was carried out, in the order it was.
    pub mended: Vec<super::Mended>,
    /// What could be put right, whether or not it was.
    pub offered: Vec<super::Repair>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}

/// What standing in place of a setup already here came to, or would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReplaceReport {
    /// What this offer names itself: the project and every service it would stop.
    ///
    /// The answer to it is this name, and nothing else is a yes to a replacement. Empty
    /// where there is nothing to stand in place of, because there is nothing to agree to.
    pub agreement: String,
    /// The project that would be stood in place of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Why nothing was stopped, where nothing was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Where the act stands.
    pub stance: super::Stance,
    /// The services that would not stop and are still up.
    pub still_running: Vec<String>,
    /// The services that were stopped.
    pub stopped: Vec<String>,
    /// The services that would be stopped, by name.
    pub would_stop: Vec<String>,
}

/// Where one request stands, in the words the person who made it would use.
///
/// Deliberately coarser than a [`crate::trace::Stage`]: a member does not need to know
/// that a release was grabbed but not imported, only that it is on its way. The trace is
/// where that detail stays, and a request names the item so it can be asked for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum RequestState {
    /// Asked for, and nobody has approved or refused it yet.
    #[serde(rename = "waiting-for-approval")]
    WaitingForApproval,
    /// Turned down — it will not be fetched.
    #[serde(rename = "declined")]
    Declined,
    /// Approved, but the attempt to fetch it failed.
    #[serde(rename = "failed")]
    Failed,
    /// Approved and on its way — being searched for, downloaded or imported.
    #[serde(rename = "getting")]
    Getting,
    /// Some of it is here: a series with only some of its episodes.
    #[serde(rename = "partly-here")]
    PartlyHere,
    /// Here, and playable.
    #[serde(rename = "here")]
    Here,
    /// It was here and has since been removed.
    #[serde(rename = "gone")]
    Gone,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One requirement, and every release that shipped something citing it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Requirement {
    /// The feature it belongs to, in words.
    pub feature: String,
    /// Every version that shipped something citing it, newest first.
    pub shipped_in: Vec<String>,
    /// Where it is defined, unless it has since been withdrawn.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether it was withdrawn after it shipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawn: Option<bool>,
}

/// What a full reset did, or — until it is confirmed — would do: the operator edits it
/// reverts back to lemonfiber's own state, and whether it was carried out or only shown.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResetReport {
    /// Whether the reset was carried out, or only previewed pending confirmation.
    pub confirmed: bool,
    /// Each of lemonfiber's own guards this act moved onto its plugin after its own
    /// change, or on a reading would move, for good and with no yes of its own; one it
    /// could not move stays on offer and says why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guards: Option<Vec<super::StackMoveTaken>>,
    /// The name of this reading, which the reset takes as its yes.
    pub offer: String,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// The operator's edits that were reverted — or, unconfirmed, that a reset would
    /// revert — each with the diff of what is lost against what lemonfiber restores.
    pub reverted: Vec<super::StackEdit>,
    /// The service connections whose drifted value was reverted to lemonfiber's — or,
    /// unconfirmed, would be — each named as it reads in a seed report.
    pub reverted_connections: Vec<String>,
}

/// What a limit comes to once it is weighed against a measured line.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "is")]
#[non_exhaustive]
pub enum Resolved {
    /// Nothing holds it back.
    #[serde(rename = "unlimited")]
    Unlimited,
    /// This many bytes a second.
    #[serde(rename = "at")]
    At {
        /// The `bytes_per_second` field.
        bytes_per_second: u64,
    },
    /// A proportion was asked for and nothing has measured the line.
    ///
    /// Deliberately not folded into [`Self::Unlimited`]. "Half of an unknown
    /// number" resolving to "no limit at all" is the shape of a setting an
    /// operator believes is in force while the stack takes the whole line.
    #[serde(rename = "unmeasured")]
    Unmeasured,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// Where a respite stands against the clock.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "standing")]
#[non_exhaustive]
pub enum RespiteStanding {
    /// None was asked for.
    #[serde(rename = "none")]
    None,
    /// In force, with this long left.
    #[serde(rename = "in-force")]
    InForce {
        /// The `seconds` field.
        seconds: u64,
    },
    /// It ran out, this long ago. Said once, then cleared.
    #[serde(rename = "expired")]
    Expired {
        /// The `seconds` field.
        seconds: u64,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// What a restore said: what it would overwrite, and whether it did.
///
/// The listing is present either way, and that is the point of the shape. It is not
/// a separate request a surface may or may not make — it is the half of a restore
/// that happens before anything is overwritten, so every answer carries it and an
/// answer that overwrote nothing is one whose `done` is absent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Restoration {
    /// What was put back, or nothing where nothing was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub done: Option<super::RestoreReport>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// What the archive holds and what restoring it would come to, read before
    /// anything was touched.
    pub would: super::Preview,
}

/// What a restore did.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RestoreReport {
    /// The lemonfiber version the archive was written by.
    pub from_version: String,
    /// The data root that was re-pointed, where the restore accepted one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relocated: Option<super::Relocation>,
    /// What was restored.
    pub scope: super::Scope,
}

/// Where the line stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Restraint {
    /// Nothing is configured.
    #[serde(rename = "unlimited")]
    Unlimited,
    /// Limits are in force, with no schedule to switch them.
    #[serde(rename = "limited")]
    Limited,
    /// Inside the household's active hours, so the limits apply.
    #[serde(rename = "scheduled-active")]
    ScheduledActive,
    /// Outside them, so the line is the stack's.
    #[serde(rename = "scheduled-quiet")]
    ScheduledQuiet,
    /// A temporary override is lifting the limits, and will expire.
    #[serde(rename = "overridden")]
    Overridden,
    /// The month is close enough to a declared cap to say so.
    #[serde(rename = "cap-warning")]
    CapWarning,
    /// The cap has been reached and the declared behaviour applies.
    #[serde(rename = "cap-exceeded")]
    CapExceeded,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What one member is held to, in the words a household would use.
///
/// The two restrictions are one decision and two services: the media server decides
/// what may be *watched* and the request service what may be *asked for*. Setting one
/// without the other is the hole this vocabulary exists to name — a child who cannot
/// watch something but can pull it into the library has parents who set a limit and got
/// half of one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Restriction {
    /// Nothing is held back from them.
    #[serde(rename = "unrestricted")]
    Unrestricted,
    /// Held to a highest rating.
    #[serde(rename = "rating-limited")]
    RatingLimited,
    /// Held to some of the libraries.
    #[serde(rename = "library-limited")]
    LibraryLimited,
    /// Held to both a rating and some of the libraries.
    #[serde(rename = "both")]
    Both,
    /// What they may watch is limited and what they may ask for is not.
    #[serde(rename = "inconsistent")]
    Inconsistent,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One stored value, handed back because the operator asked for it and said so.
///
/// Separate from [`Held`] rather than a field on it, so that the inventory cannot
/// carry a value by accident: a surface that renders the inventory has nothing to
/// render, whatever it does.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Revealed {
    /// Which credential this is.
    pub name: String,
    /// The value, present only where the ask was confirmed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// What the operator is told before it appears, whether or not it appears.
    pub warning: String,
}

/// A proposed change, read against what is in force, and where it stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Review {
    /// The difference, as it would be applied.
    pub change: super::ConfigChange,
    /// What the change comes to on this machine, beyond the value it changes.
    ///
    /// Filled by whoever went and asked — the services where they file, the clients
    /// what they are fetching — and empty on every change that comes to nothing
    /// beyond its value. Carried on a staged proposal as well as an applied one: a
    /// review that withheld this until after the yes would be asking for a yes to
    /// something unstated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub findings: Option<super::Findings>,
    /// What proving the replacement credential came to, where one was proven.
    ///
    /// A replacement is proven against its live service before the credential it
    /// replaces is discarded, so this is present for exactly those settings and
    /// absent everywhere else.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<super::Validation>,
    /// Why nothing was written, where nothing was and the reason is not simply that
    /// somebody has yet to say yes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Where the proposal stands.
    pub stance: super::Stance,
}

/// How far a removal got, across the two services a household member exists on.
///
/// The media server is removed first and the request service second, because the
/// request service authenticates *through* the media server — so once the first is gone
/// they can do nothing either way, and a failure at the second leaves an account that
/// cannot sign in rather than somebody who can still watch.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Revoked {
    /// Both, which is what removal means.
    #[serde(rename = "everywhere")]
    Everywhere,
    /// The media server only, and the request service still holds an account.
    ///
    /// They can neither watch nor ask — the account left behind signs in through the one
    /// that is gone — but something is there that should not be, and the next run of this
    /// command takes it.
    #[serde(rename = "media-server-only")]
    MediaServerOnly,
    /// Nothing was removed, because nothing was confirmed.
    #[serde(rename = "nothing")]
    Nothing,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// The hours the household is awake, declared once for every download client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rhythm {
    /// When the household's day starts.
    pub from: String,
    /// When it ends, which may be the next morning.
    pub to: String,
}

/// Which of the two volumes a reading is about.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Role {
    /// Where the media and the downloads live.
    #[serde(rename = "data")]
    Data,
    /// Where the services keep their own configuration and databases.
    #[serde(rename = "services")]
    Services,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// A directory everything lemonfiber keeps sits under.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Root {
    /// The directory itself.
    pub at: String,
    /// What lives under it, and what losing it would cost.
    pub what: String,
}

/// What one rotation came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rotation {
    /// Every consumer, and how far the rotation reached it.
    pub consumers: Vec<super::Propagation>,
    /// Which credential was to be replaced.
    pub credential: String,
    /// The offer a replacement that takes one answers, where this reading named one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// What became of the replacement.
    pub settled: super::Settled,
}
