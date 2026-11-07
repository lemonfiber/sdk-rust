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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<PluginSourceStanding, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for PluginSourceStanding {
    const TAG: Option<&str> = Some("standing");
    const KNOWN: &[&str] = &["reachable", "unreachable", "unasked"];
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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<PluginVerdict, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for PluginVerdict {
    const TAG: Option<&str> = Some("outcome");
    const KNOWN: &[&str] = &[
        "passed",
        "failed",
        "unproven",
        "failing-as-declared",
        "open",
    ];
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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where a problem stands with respect to being fixed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
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
#[non_exhaustive]
pub enum Pulling {
    /// It is fetching, or would start on the next thing handed to it.
    #[serde(rename = "fetching")]
    Fetching,
    /// Nothing is moving and nothing new would start.
    #[serde(rename = "stopped")]
    Stopped,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}
