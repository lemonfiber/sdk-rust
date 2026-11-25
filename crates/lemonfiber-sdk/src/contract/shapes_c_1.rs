/// Who is calling.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "as")]
#[non_exhaustive]
pub enum Caller {
    /// The operator, on a session opened with the machine's password.
    #[serde(rename = "operator")]
    Operator,
    /// A household member, on a session of their own.
    #[serde(rename = "member")]
    Member {
        /// The id the media server files them under.
        id: String,
    },
    /// A program, on a key the operator or a member minted for it.
    #[serde(rename = "key")]
    Key {
        /// The key's name.
        name: String,
        /// What it admits: `read`, `act` or `member:<name>`.
        scope: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged, deserialize_with = "crate::open::unrecognised::<Caller, _>")]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Caller {
    const TAG: Option<&str> = Some("as");
    const KNOWN: &[&str] = &["operator", "member", "key"];
}

/// What the session or key a request carried came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "standing")]
#[non_exhaustive]
pub enum Calling {
    /// Somebody this run admits.
    #[serde(rename = "known")]
    Known {
        /// Who they are.
        caller: Box<super::Caller>,
    },
    /// Nobody this run admits.
    #[serde(rename = "nobody")]
    Nobody,
    /// A member the media server could not be asked about.
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
    /// A key held back by too many wrong answers.
    #[serde(rename = "held")]
    Held {
        /// How many whole seconds are left to wait.
        retry_after: u64,
    },
    /// A key that crossed a network in the clear.
    #[serde(rename = "exposed")]
    Exposed,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged, deserialize_with = "crate::open::unrecognised::<Calling, _>")]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Calling {
    const TAG: Option<&str> = Some("standing");
    const KNOWN: &[&str] = &["known", "nobody", "unconfirmed", "held", "exposed"];
}

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

/// Why the catalogue's index was not read.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CatalogueFailure {
    /// The problem code an install by name would be refused with.
    pub code: String,
    /// What went wrong, in one line.
    pub summary: String,
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

/// One plugin the catalogue offers.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CataloguedPlugin {
    /// What it does, in its author's words.
    pub description: String,
    /// The plugin's id, which is the name it is installed by.
    pub id: String,
    /// The version installed on this machine. Absent where it is not installed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installed: Option<super::InstalledVersion>,
    /// What it calls itself.
    pub name: String,
    /// The version the catalogue reviewed.
    pub version: String,
    /// What the household is without it.
    pub without_it: String,
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
    /// When it was made, in whole seconds since the Unix epoch.
    ///
    /// **`0` means the clock was unreadable when the change was written**, not that it
    /// was made at the epoch: it is how a machine whose clock would not answer stamps a
    /// change. Two changes both stamped `0` were not made at the same moment, and a
    /// reader showing it as a date in 1970 would be inventing one.
    pub at: super::Instant,
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

/// One live TV channel, as one member's account may watch it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Channel {
    /// The identifier the server tells it apart by.
    pub id: String,
    /// What it is called.
    pub name: String,
    /// The programme on it now, where the server knows one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub now: Option<super::Programme>,
    /// Its number, where it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

/// The live TV channels one member may watch, each with the programme on it now.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChannelsReport {
    /// Each channel their own account may watch whose programme now is within their
    /// limits. Empty where their account may not watch live TV.
    pub channels: Vec<super::Channel>,
    /// The identifier the media server files them under.
    pub id: String,
    /// The member this was asked for, by the name they are known by.
    pub member: String,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    pub rehearsed: bool,
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
        door: Box<super::Refusal>,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged, deserialize_with = "crate::open::unrecognised::<Chosen, _>")]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Chosen {
    const TAG: Option<&str> = Some("chosen");
    const KNOWN: &[&str] = &["derived", "named", "refused"];
}

/// Everything a plugin's source says, and what this build makes of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Claimed {
    /// What every verdict in this report was reached against.
    pub against: super::PluginEvidence,
    /// What its services declare they can do.
    pub capabilities: Vec<super::Claiming>,
    /// The checks it contributes, against the recordings they name.
    ///
    /// Reported and never counted against the install. A contributed check reports on a
    /// running stack rather than gating one, so a check that its own recording refuses
    /// is a check doing its job on a machine in the state it was recorded in — which is
    /// a different fact from a claim its recordings refute.
    pub checks: Vec<super::Asserted>,
    /// What it adds to lemonfiber's own registers.
    pub contributions: Vec<super::Contributed>,
    /// The extension points generation it was held to.
    pub extension_points_version: u32,
    /// The plugin's id.
    pub id: String,
    /// Whether this plugin would be installed as it stands.
    ///
    /// Carried rather than left for a reader to work out. It is one rule over two of
    /// the fields above, and a consumer deciding it for itself would be a second copy
    /// of that rule — free to disagree with the exit status this same answer produces.
    pub installable: bool,
    /// Its human-facing name.
    pub name: String,
    /// What must hold before it is installed, against the recordings it ships.
    pub proofs: Vec<super::Asserted>,
    /// Everything the two published vocabularies refuse, in one pass.
    pub refusals: Vec<super::Violation>,
    /// The author's own version of it.
    pub version: String,
    /// The capability vocabulary generation it was held to.
    ///
    /// Carried for the reason every other read under this word carries one: an author
    /// comparing what they were refused for with what they read needs to know whether
    /// the difference is their manifest or this build.
    pub vocabulary_version: u32,
}

/// One capability a plugin's service declares, and what this build makes of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Claiming {
    /// What asking for it would come to, or nothing where nothing can ask.
    ///
    /// A namespaced capability has no answer here rather than an empty one: nothing
    /// consumes it, which is what *inert* means, and reporting it as unfilled would be
    /// describing a gap where there is a design.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filling: Option<super::Filling>,
    /// The capability's name.
    pub name: String,
    /// Whether it is this plugin's own rather than one from the core vocabulary.
    pub own: bool,
    /// Every probe the claim binds, in the order the vocabulary declares them.
    pub probes: Vec<super::ProbeRan>,
    /// The service declaring it.
    pub service: String,
    /// How far the claim has been shown.
    pub shown: super::Shown,
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
    /// Every answer setup writes, in the catalogue's order, whether or not anything holds
    /// it. Never a value: a value is read where it is held, withheld there as it is.
    pub decisions: Vec<super::DecisionReport>,
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
    /// Each area the operator declared unmanaged, in the order written, with what it
    /// covers now. Empty where nothing is declared. An area and a reason are the
    /// operator's own text, to be shown as plain text.
    pub unmanaged: Vec<super::UnmanagedReport>,
    /// How many declared areas `unmanaged` leaves out for being too many.
    pub unmanaged_more: u64,
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

/// What a plugin's recordings say about the contracts its adapters speak.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Conformed {
    /// Every case of every contract an adapter speaks, as its recording answered it.
    pub cases: Vec<super::Judged>,
    /// Whether nothing is refused and every case passed.
    pub conforms: bool,
    /// The plugin's id.
    pub id: String,
    /// Everything the manifest is refused for, which no recording can make up for.
    pub refusals: Vec<super::Violation>,
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

/// One row a plugin adds to a register lemonfiber already runs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contributed {
    /// The point it is made at.
    pub at: String,
    /// The check a remedy is for, where this is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#for: Option<String>,
    /// The identity it holds, namespaced with the plugin's id.
    pub id: String,
    /// What it says, in one line.
    pub says: String,
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

/// What a running copy copies, as its reading names it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "scope")]
#[non_exhaustive]
pub enum CopiedScope {
    /// Every service's configuration, plus lemonfiber's own and the stack.
    #[serde(rename = "whole_stack")]
    WholeStack,
    /// One named service's configuration alone.
    #[serde(rename = "service")]
    Service {
        /// The service whose configuration it copies.
        name: String,
    },
    /// An existing setup's own configuration.
    #[serde(rename = "existing")]
    Existing {
        /// The Compose project it copies, to the operator alone.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        project: Option<String>,
        /// The host trees it copies, to the operator alone.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        trees: Option<Vec<super::Tree>>,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<CopiedScope, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for CopiedScope {
    const TAG: Option<&str> = Some("scope");
    const KNOWN: &[&str] = &["whole_stack", "service", "existing"];
}

/// Whether a copy is a backup or a restore.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CopyWhat {
    /// A backup, writing an archive.
    #[serde(rename = "backup")]
    Backup,
    /// A restore, writing an archive back.
    #[serde(rename = "restore")]
    Restore,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// The backup or restore running on this machine now, or none.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CopyingReport {
    /// The copy running now, or none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<super::RunningCopy>,
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

/// What a thing a declaration covers is, read off where it was found.
///
/// Never off its name, since a stack may name a service `config`: one area can cover a
/// service, a file and a setting at once, each listed as what it is.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CoveredKind {
    /// A service the stack runs, which is not seeded and is observed rather than wired.
    #[serde(rename = "service")]
    Service,
    /// A file lemonfiber writes, which is not written on any run.
    #[serde(rename = "path")]
    Path,
    /// A setting, which `config set` refuses with the operator's reason.
    #[serde(rename = "setting")]
    Setting,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One thing a declaration covers.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoveredReport {
    /// What it is.
    pub kind: super::CoveredKind,
    /// Its name: a service's id, a file's place in the stack or the project, or a
    /// setting's key.
    pub name: String,
}

/// What a monitor lemonfiber provisions covers, which its tag names.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Covers {
    /// A service of the stack, by its id.
    #[serde(rename = "service")]
    Service(String),
    /// A household-facing entry point, by the id of the service it is.
    #[serde(rename = "entry-point")]
    EntryPoint(String),
    /// A job the stack runs on a schedule, by its id.
    #[serde(rename = "job")]
    Job(String),
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged, deserialize_with = "crate::open::unrecognised::<Covers, _>")]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Covers {
    const TAG: Option<&str> = None;
    const KNOWN: &[&str] = &["service", "entry-point", "job"];
}
