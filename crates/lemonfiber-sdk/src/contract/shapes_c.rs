/// One completed download, and what reclaiming it would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Candidate {
    /// What it occupies.
    pub bytes: u64,
    /// What removing it costs, where it costs anything.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consequence: Option<String>,
    /// What both sides call it.
    pub name: String,
    /// Where it stands.
    pub standing: super::SeedingStanding,
}

/// A monthly allowance, and what to do at the end of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cap {
    /// What happens when it is reached, chosen when the cap was declared.
    pub exceeded: super::WhenExceeded,
    /// The allowance, in bytes.
    pub monthly: u64,
}

/// Every capability this stack has, by the path its request is served at.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Capabilities {
    /// What each comes to for the credential that asked. A request this stack does
    /// not have is absent rather than listed as anything.
    pub capabilities: std::collections::BTreeMap<String, super::CapabilityState>,
    /// Whose credential asked.
    pub scope: super::CredentialScope,
    /// The stack's own identifier, the one pairing material carries, to every credential
    /// alike: what tells two credentials apart from two stacks. Never the address or the
    /// certificate, which re-pairing exists to change. Absent only where this machine
    /// has nowhere to keep one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

/// What one capability comes to for the credential that asked.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CapabilityState {
    /// This stack can do it, and this credential may.
    #[serde(rename = "available")]
    Available,
    /// The stack has it, and a setting this machine has switched off has to be turned
    /// back on first.
    #[serde(rename = "unconfigured")]
    Unconfigured,
    /// The stack has it, and this credential may not ask for it.
    #[serde(rename = "unpermitted")]
    Unpermitted,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What the line was measured to carry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Capacity {
    /// Bytes a second down.
    pub down: u64,
    /// Where the figure came from.
    pub source: super::Source,
    /// When it was taken, in seconds since the epoch.
    pub taken: u64,
    /// Whether the path it was measured over goes through the VPN tunnel.
    pub through_tunnel: bool,
    /// Bytes a second up.
    ///
    /// Measured apart from the download, because a home connection is asymmetric
    /// and a single figure for both would make every upload share far larger than
    /// the operator meant.
    pub up: u64,
}

/// What adopting one existing service would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CarryingReport {
    /// Whether its database must be backed up before lemonfiber opens it.
    pub backup_first: bool,
    /// What that means for this service's data, in the operator's terms.
    pub because: String,
    /// The version standing here now.
    pub existing: String,
    /// The version lemonfiber pins.
    pub ours: String,
    /// Whether lemonfiber will not do this at all.
    pub refused: bool,
    /// The service, by the name lemonfiber runs it under.
    pub service: String,
    /// Which of the two is the later, in one word.
    pub verdict: String,
}

/// What this stack holds, and what it used to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CatalogueReport {
    /// The services this stack has dropped, in the order it records them.
    ///
    /// Empty for a stack that has never dropped anything, which is a different thing
    /// from a stack that keeps no record — and told apart by the fact that a stack
    /// keeping no record cannot be read as having dropped something it did.
    pub removed: Vec<super::RemovedService>,
    /// The services, in the order the stack declares them.
    ///
    /// Every service the manifest holds rather than the ones some form would start:
    /// what a service is *for* is the question being asked, and an answer narrowed to
    /// what is running would leave the operator unable to ask about the one they are
    /// deciding whether to run.
    pub services: Vec<super::CataloguedService>,
}

/// What one service is for, as the stack declares it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CataloguedService {
    /// How much its absence matters.
    pub criticality: super::Criticality,
    /// What it does for the operator, in plain language.
    pub describes: String,
    /// The service's id, which is also its Compose service name.
    pub id: String,
    /// What it is called in front of an operator.
    pub name: String,
    /// What going without it costs.
    ///
    /// Carried beside the description rather than left to a separate question,
    /// because the pair is what turns an inventory into a judgement: knowing that
    /// Bazarr finds subtitles says nothing about whether its being down matters.
    pub without_it: String,
}

/// One thing that could be behind a symptom, and how to tell it from the others.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cause {
    /// What is wrong.
    pub because: String,
    /// What to do about it.
    pub fix: String,
    /// How to tell this cause from the others under the same symptom.
    pub tell: String,
}

/// What asking for the certificate to be replaced came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CertificateReport {
    /// What replacing it means for every phone already paired.
    pub consequence: String,
    /// What a phone would pin now: the new certificate where it was replaced, the one
    /// kept where it was not, and nothing where none has been made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Whether it was replaced. Unconfirmed, it is not, and what replacing it costs is
    /// what is said.
    pub replaced: bool,
}

/// One change lemonfiber made, and whether it could be put back.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangeReport {
    /// How many changes that one operation made, this one among them.
    ///
    /// An operation is the unit an operator agreed to, and undoing half of one leaves a
    /// machine in a state nobody chose — so what a single line would take with it is on
    /// the line rather than left to be counted off the list.
    pub alongside: u64,
    /// When it was made, as whole seconds since the Unix epoch, written in decimal.
    ///
    /// A string of digits rather than a number, because it is the stamp the record keeps
    /// and a stamp is compared and stored as text; what it counts is stated here so a
    /// reader can turn it into a time without guessing at a format.
    ///
    /// **`0` means the clock was unreadable when the change was written**, not that it
    /// was made at the epoch: it is how a machine whose clock would not answer stamps a
    /// change. It is not an instant, so two changes both stamped `0` were not made at
    /// the same moment, and a reader showing it as a date in 1970 would be inventing
    /// one.
    pub at: String,
    /// Why it could not go further, where it could not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub because: Option<String>,
    /// What it did, in the operator's terms.
    pub did: String,
    /// What to do instead, where there is something.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instead: Option<String>,
    /// The operation that made it — a seed, a reconfigure, an applied fix — so a
    /// history reads as what happened rather than as bare diffs.
    pub operation: String,
    /// How far it could be put back.
    pub reversal: super::ChangeReversal,
    /// What it was made to.
    pub target: String,
}

/// How far a change can be put back.
///
/// Published as the closed set it is, rather than as a word a reader has to trust will
/// be one of three: a surface that lays out a history branches on it, and a set the
/// contract names is one a generated reader can match exhaustively.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ChangeReversal {
    /// It can be put back exactly.
    #[serde(rename = "whole")]
    Whole,
    /// Some of it can, and the rest is stated rather than attempted.
    #[serde(rename = "partial")]
    Partial,
    /// None of it can.
    #[serde(rename = "none")]
    None,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What one run of this command did to the machine.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Changed {
    /// Whether it installed it, rather than took it back.
    pub installed: bool,
    /// The command it acted on.
    pub name: String,
    /// Whether this was a rehearsal, in which case nothing above happened.
    pub rehearsed: bool,
    /// Whether it started the command, which only installing does.
    pub started: bool,
    /// Everything it wrote or removed, so nothing goes unnamed in either direction.
    pub touched: Vec<String>,
}

/// Whether the record describes what this build could have shipped.
///
/// The three the specification names, and the distinction between the last two is
/// the one worth keeping: being behind the tags is a lag, and contradicting them is
/// a fault.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ChangelogState {
    /// The record holds this build's release, and claims nothing later.
    #[serde(rename = "current")]
    Current,
    /// This build's release has no notes yet, which is where a tag leaves things.
    #[serde(rename = "pending")]
    Pending,
    /// The record and what this build could have shipped disagree.
    #[serde(rename = "stale")]
    Stale,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What to do about an interrupted apply, once one is found.
///
/// The three exits the setup wizard promises for its one dangerous state: keep
/// going, walk it back, or drop it entirely. A surface offers these; [`Recovery`]
/// turns the chosen one into the work it means.
///
/// Read back as well as built, because the surface offering them may not be in this
/// process: a browser sends the one the operator picked by the name it is written
/// under here.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Choice {
    /// Carry on from where apply stopped, keeping what was already written.
    #[serde(rename = "resume")]
    Resume,
    /// Undo what was written and return to the reviewed answers, ready to apply
    /// again.
    #[serde(rename = "roll-back")]
    RollBack,
    /// Undo what was written and discard the answers too, back to a clean slate.
    #[serde(rename = "start-over")]
    StartOver,
}

/// One value a setting accepts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Choosable {
    /// The values of other settings under which it is a choice: each setting with the
    /// values it has to hold one of, and none where it always is.
    pub needs: std::collections::BTreeMap<String, Vec<String>>,
    /// Whether it is a choice now.
    pub open: bool,
    /// The value.
    pub value: String,
}

/// How the front door came to be the one it is.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "chosen")]
#[non_exhaustive]
pub enum Chosen {
    /// Worked out from what the stack declares, which is what a stack whose
    /// operator has named nothing answers.
    #[serde(rename = "derived")]
    Derived,
    /// Named by the operator, by the id the stack declares it under, and it is the
    /// door.
    #[serde(rename = "named")]
    Named {
        /// The `door` field.
        door: String,
    },
    /// Named by the operator and refused. The worked-out door stands, and this
    /// carries what was named and why it is not it.
    #[serde(rename = "refused")]
    Refused {
        /// The `door` field.
        door: super::Refusal,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// A stable identifier for a kind of problem.
///
/// Stability is the whole point: an operator who searches for a code should find
/// the same answer a year later. Every code is declared in the error crate's `codes`
/// module, and a code is never recycled.
pub type Code = String;

/// One download still coming down when the removal was asked for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Coming {
    /// What it is, as the client names it.
    pub name: String,
    /// How far along, from zero to a hundred.
    pub progress: u8,
}

/// What a whole set of services amounts to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Condition {
    /// Nothing is running.
    #[serde(rename = "inactive")]
    Inactive,
    /// Running, with at least one service in a state that wants attention.
    #[serde(rename = "degraded")]
    Degraded,
    /// Some services are up and others are not.
    #[serde(rename = "partial")]
    Partial,
    /// Everything expected is up and answering.
    #[serde(rename = "active")]
    Active,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// The difference between the configuration in force and the one proposed.
///
/// One setting, because one call changes one setting. What makes it a diff rather than
/// a value is `from`: an operator deciding whether to go ahead is deciding between two
/// things, and a report that showed only the new one would be asking them to remember
/// the old one correctly.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigChange {
    /// Whether applying it is cheap or consequential.
    pub cost: super::Cost,
    /// What it holds now, withheld where it is a credential, and absent where the
    /// setting holds nothing yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The setting the change names.
    pub key: String,
    /// What it would hold, withheld the same way.
    pub to: String,
}

/// The answer to a configuration command.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigReport {
    /// Whether this command changed, or would change, a setting.
    pub changed: bool,
    /// What this change costs, where making it decides something with a
    /// consequence — moving the library, turning port forwarding off, or naming a
    /// front door. Stated for a change that is only staged as well as one that
    /// landed, since the moment before it happens is the moment it is worth reading.
    /// Absent for a read, and for a change to a setting nobody catalogued a cost for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consequence: Option<String>,
    /// Whether this was a rehearsal, so a change that `changed` reports was one
    /// that *would* be made rather than one that was.
    pub rehearsed: bool,
    /// The difference between the configuration in force and the one proposed, and
    /// where that proposal stands: applied, staged for a confirmation, turned away,
    /// or nothing to do. Absent for a read, which proposes nothing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<super::Review>,
    /// The settings asked about — one for a lookup, all of them for a listing.
    pub settings: Vec<super::SettingReport>,
}

/// A port lemonfiber wants for a service that something else already answers on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConflictReport {
    /// The project already holding it.
    pub held_by: String,
    /// The host port both want.
    pub port: u16,
    /// The lemonfiber service that would publish it.
    pub wanted_by: String,
}

/// One line of the accounting.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Consumption {
    /// What it is about.
    pub category: super::SpaceCategory,
    /// What getting it back would cost.
    pub reclaim: super::Reclaim,
    /// What it occupies, counted both ways.
    pub tally: super::Tally,
}

/// Everything gathered for a bundle, and everything that could not be.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contents {
    /// What could not be collected, named.
    ///
    /// Named rather than passed over: a bundle from a machine whose diagnostics will not
    /// run is exactly the bundle worth having, and a gap nobody mentions reads as an
    /// absence of trouble rather than as an absence of information.
    pub missing: Vec<String>,
    /// The files, in the order a reader would want them.
    pub pieces: Vec<super::Piece>,
    /// Where and when it came from.
    pub taken: super::Taken,
    /// How it was made, and what its operator chose.
    pub terms: super::Terms,
}

/// A row in a register lemonfiber already runs.
///
/// The fields beyond `at` and `id` are the row its point declares, and the point is
/// published — so the required set, the optional set, the closed sets and the bounds
/// are read from `extension-points.json` rather than restated here. What this type
/// fixes is that a contribution is declared in this block and nowhere else, and that
/// it carries nothing outside the union of the rows the published points take.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contribution {
    /// What to do, in the imperative.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// A point the build publishes. One it does not is refused by name.
    pub at: String,
    /// Which family a check is narrowed to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The technical half of a remedy, which must not lead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// What the answer must be.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expect: Option<super::Expect>,
    /// Recordings the check fails on, each with the constraint that fails there and why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<Vec<super::PluginExpectedFailure>>,
    /// The recorded response the check is proved against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<String>,
    /// The check a remedy is for, which must be one this same plugin declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#for: Option<String>,
    /// Namespaced with the declaring plugin's id, always.
    pub id: String,
    /// Method and path on the plugin's own service.
    ///
    /// There is no field for a host, so a check cannot be pointed at another service,
    /// at the machine, or off it. A plugin wanting to say something about a service it
    /// did not install is asking to speak for somebody else's software.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<super::PluginRequest>,
    /// Which service the finding is about. Defaults to the plugin's own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// How long a check may run, within the bounds the point declares.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_s: Option<u32>,
    /// The one-line summary of what was checked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Why this is worth checking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// What changing a decision costs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Cost {
    /// Applied with a restart of the services it affects, and nothing else moves.
    #[serde(rename = "cheap")]
    Cheap,
    /// Said and confirmed before it is applied: it may move data, invalidate library
    /// paths, or take a service away.
    #[serde(rename = "consequential")]
    Consequential,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One of the two counts the request service keeps, and what it has left.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Counted {
    /// How many the period allows. Absent where nothing limits them, which is a
    /// different answer from a limit of nought.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// How long the period is, in the words a household says it in — absent where
    /// the count runs from the beginning rather than over a window.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// How many more they may ask for. Absent where nothing limits them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining: Option<u32>,
    /// How many the period has already counted against them.
    pub used: u32,
}

/// How much of a traced series is here, season by season — the aggregate that turns a
/// single furthest stage into an answer about the whole. The counts are of parts someone
/// asked for; what nobody asked for is reported beside them, never folded in.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Coverage {
    /// How many wanted parts are here, across every season.
    pub have: u64,
    /// Each season, in order.
    pub seasons: Vec<super::SeasonCoverage>,
    /// How many parts nobody asked for, across every season.
    pub unmonitored: u64,
    /// How many parts were asked for, across every season.
    pub wanted: u64,
}

/// A credential of another service granted to a third-party plugin, named by the setting it
/// is held under and never by its value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CredentialGranted {
    /// When the operator approved it.
    pub at: String,
    /// The setting the credential is held under, as the inventory names it.
    pub credential: String,
    /// The capability of the plugin's own it was granted to fill.
    pub r#for: String,
    /// The service whose credential it is.
    pub of: String,
    /// The plugin it is granted to.
    pub plugin: String,
}

/// One credential, described without being disclosed.
///
/// There is deliberately no value here, and no field a value could be put in later
/// without the change being visible in review.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CredentialHeld {
    /// What is worth saying about this one, where anything is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advisory: Option<String>,
    /// Everything that authenticates with it. Named individually, because a
    /// consumer left off this list is a consumer a rotation would silently strand.
    pub consumers: Vec<String>,
    /// A short likeness of the value, for telling two copies apart in a report.
    ///
    /// Absent where there is no value to take one of. Never reversible and never a
    /// proof — see [`fingerprint`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Whose line it is: the stack's own, or an installed plugin's, named.
    ///
    /// A different question from who produced it. A plugin's secret is minted by the
    /// service it belongs to like any other, and what the operator needs to know as well
    /// is that the service is one a plugin brought.
    pub from: super::ValueOrigin,
    /// Where the value lives, as a path or a description of one.
    pub location: String,
    /// What it is, in the operator's words — `Media server administrator password`.
    pub name: String,
    /// Who produced it.
    pub origin: super::Origin,
    /// The setting it is recorded under, which is a name and never a value.
    pub setting: String,
    /// Where it stands.
    pub state: super::CredentialState,
}

/// How far a rotation reached one consumer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "reach")]
#[non_exhaustive]
pub enum CredentialReach {
    /// It now holds the replacement.
    #[serde(rename = "updated")]
    Updated,
    /// It will hold the replacement once one more thing happens, and that thing is
    /// named. A consumer reading the value out of a container's environment has it
    /// fixed at the moment the container was created, so recording a new one is
    /// only half of reaching it.
    #[serde(rename = "pending")]
    Pending {
        /// What still has to happen, written as the command that does it.
        detail: String,
    },
    /// It could not be updated. Named rather than dropped, because a consumer left
    /// holding the old value is the failure this list exists to surface.
    #[serde(rename = "failed")]
    Failed {
        /// Why it could not be.
        detail: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// What revoking one grant came to, or would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CredentialRevoked {
    /// The name this reading goes by, which the revoke takes as its yes.
    pub agreement: String,
    /// Whether the grant was ended, rather than read or rehearsed.
    pub ended: bool,
    /// The grant it ends.
    pub grant: super::CredentialGranted,
    /// The plugin's services that read the credential, restarted once it is withdrawn.
    pub restarts: Vec<String>,
    /// Why its services did not restart, where they did not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unrestarted: Option<String>,
}

/// Whose credential asked, as a client is told it.
///
/// Said rather than left to be inferred from which requests are permitted: a client
/// guessing a member from a missing read would guess wrong the day that read moves.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CredentialScope {
    /// The operator: the password's session, or the token printed at this machine.
    #[serde(rename = "operator")]
    Operator,
    /// A key that reads and acts on nothing.
    #[serde(rename = "read")]
    Read,
    /// A key that reads, and calls the actions the contract publishes for a key.
    #[serde(rename = "act")]
    Act,
    /// A household member: their own session, or a key scoped to them.
    #[serde(rename = "member")]
    Member,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where a credential stands, as far as lemonfiber can tell without spending it.
///
/// Six states rather than a boolean because the operator's next move differs for
/// each: an absent credential is one to supply, a stale one is one to prove, an
/// invalid one is one to replace, and the two rotation states exist so a run
/// interrupted half-way through a replacement is legible rather than mysterious.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CredentialState {
    /// Required by what this stack runs, and not supplied.
    #[serde(rename = "absent")]
    Absent,
    /// Present, and proven against the service it authenticates to.
    #[serde(rename = "active")]
    Active,
    /// Present, and never proven — or not proven since it was last written.
    #[serde(rename = "stale")]
    Stale,
    /// Present, and refused by the service the last time it was offered.
    #[serde(rename = "invalid")]
    Invalid,
    /// A replacement is being proven; the existing value is still the one in force.
    #[serde(rename = "rotating")]
    Rotating,
    /// Replaced. What is left is the old value, waiting to be destroyed.
    #[serde(rename = "superseded")]
    Superseded,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// How much a service's absence costs.
///
/// Serialisable as well as readable, because it reaches an operator: a status
/// report that says a service is down without saying whether that matters
/// leaves them to guess, and the manifest already holds the answer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Criticality {
    /// Its failure has consequences outside the machine.
    #[serde(rename = "critical")]
    Critical,
    /// The form does not work without it.
    #[serde(rename = "core")]
    Core,
    /// The form works, badly.
    #[serde(rename = "important")]
    Important,
    /// Makes things better.
    #[serde(rename = "enhancing")]
    Enhancing,
    /// Nice to have.
    #[serde(rename = "optional")]
    Optional,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}
