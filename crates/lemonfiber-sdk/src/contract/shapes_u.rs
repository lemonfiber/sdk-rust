/// A container running under this project that the stack description never declared.
///
/// Its own type rather than a [`Service`] with the fields left blank. A service
/// carries a profile and a criticality, and there is no honest value for either here:
/// filling them in would have lemonfiber asserting how much something matters when
/// the only thing it knows about it is that it exists.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Undeclared {
    /// What it does for the operator — which is exactly what is not known.
    pub describes: String,
    /// The Compose service name the engine reports it under.
    pub id: String,
    /// What it is doing, read the same way a declared service's state is.
    pub state: super::ServiceState,
}

/// A single reversal, for the surface to carry out.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Undo {
    /// What reversing it does.
    pub action: super::Action,
    /// The service or file to reverse it against.
    pub target: String,
}

/// One change a reversal did not put back, and why it did not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UndoLeft {
    /// Why it is still standing, in the operator's terms.
    pub because: String,
    /// What the change was against — a service, or lemonfiber's own environment file.
    pub target: String,
}

/// What putting one change back means beyond the change itself.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UndoNoted {
    /// What goes back, what does not go with it, and what to do instead.
    pub because: String,
    /// What the change was against.
    pub target: String,
}

/// What putting a run back came to.
///
/// A report rather than a bare list, because it is what an envelope carries and an
/// envelope carries a document. Two lists, and the second is the one that matters when
/// it is not empty: what went back, and what did not with the reason it did not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UndoReversal {
    /// What was not put back, each with the reason it was not.
    ///
    /// A reversal an operator asked for by name has to say what it did *not* do. Five
    /// changes asked back and three carried out is a machine in a state nobody has been
    /// told about, and "some of it worked" is the sentence that makes somebody go
    /// looking by hand. Empty where everything went back, which is the common case.
    ///
    /// On a run that only said what it would do, this is what it cannot promise: a
    /// change that goes back through the service that made it goes back only where that
    /// service is answering, and a rehearsal has not asked one.
    pub left: Vec<super::UndoLeft>,
    /// What putting these changes back means beyond the changes themselves.
    ///
    /// Empty on almost every run. What lands here is a change the judgement can put
    /// back in full and that still leaves something behind — the one in force today
    /// being a setting that re-points where data lives, which goes back while the
    /// library stays exactly where it was moved to.
    ///
    /// Neither list above can carry it. It did not fail to go back, so it is not what
    /// was left; and reporting only that it went back would send an operator looking
    /// for their files at an address that no longer names them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noted: Option<Vec<super::UndoNoted>>,
    /// Whether this run only said what it would put back.
    ///
    /// A flag rather than a second shape, because the two lists mean the same thing
    /// either way and a caller reading them should read one document. What changes is
    /// the tense a surface says them in.
    pub rehearsed: bool,
    /// What was put back, in the order it was — or, on a run that only said what it
    /// would do, what would go back.
    pub reversed: Vec<super::Undo>,
}

/// A capability something asks for and nothing fills, and what asked for it.
///
/// The pair rather than the name: a capability nothing fills is a fact about the
/// stack, and a capability *`seerr` asks for* and nothing fills is a thing somebody
/// can act on. Reporting the first and leaving the second to be worked out is the
/// obscure failure at the point of use this exists instead of.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Unfilled {
    /// The service that asked.
    pub by: String,
    /// What it asked for.
    pub capability: String,
    /// The medium it asked a filler of, where nothing filling it files that medium.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    /// The upstream's own API it asked in, or every API it asked in any of, where no
    /// upstream filling it declares one of them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<super::Apis>,
}

/// A removal, before or after it happened.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Uninstall {
    /// What removing would come to, or what it came to.
    pub manifest: super::UninstallManifest,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Whether anything was removed on this run.
    pub removal: super::UninstallRemoval,
}

/// How much of a manifest was read and how much stood in for what could not be.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UninstallConfidence {
    /// Whether every source this tier needed answered.
    pub complete: bool,
    /// What could not be read, each in the words of whatever refused.
    ///
    /// The point of the field: a manifest that is short says so and says why, rather
    /// than reading as a machine with less on it than it has.
    pub unread: Vec<String>,
}

/// Something a removal could not take.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UninstallLeft {
    /// How to finish it by hand.
    pub by_hand: String,
    /// What is still there.
    pub name: String,
    /// What the machine said about it, verbatim.
    pub why: String,
}

/// What removing would come to, shown before anything is removed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UninstallManifest {
    /// What this reading names itself, so an answer says which reading it answered.
    pub agreement: String,
    /// Whether a backup was offered before configuration is destroyed, and how.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<String>,
    /// What the lines that are going occupy, where that is knowable.
    pub bytes: u64,
    /// What is still coming down, which stopping would interrupt.
    pub coming: Vec<super::Coming>,
    /// How much of this was read, and what could not be.
    pub confidence: super::UninstallConfidence,
    /// What is beneath the data location that the stack did not put there.
    ///
    /// Not a warning. While this is non-empty the data location is never removed as
    /// one tree, and only the stack's own directories beneath it are offered.
    pub foreign: Vec<super::Foreign>,
    /// Every line it reaches, each said to be going or said to be kept.
    pub items: Vec<super::Item>,
    /// What it leaves alone, in the operator's words.
    pub keeps: String,
    /// What lemonfiber cannot remove, each with how to remove it by hand.
    pub outside: Vec<super::Outside>,
    /// What it takes, in the operator's words.
    pub removes: String,
    /// Which removal this is.
    pub tier: super::Tier,
    /// Whether the data location is on a network share or a drive that unplugs.
    ///
    /// Said where it is, so removing across a mount an operator forgot was a mount is
    /// something they read before agreeing rather than after.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
}

/// Whether anything was removed on this run, and what became of it.
///
/// Four states rather than the five a removal passes through. `removing` is the
/// interval between the last two and is said through the narrator as it happens — a
/// value returned at the end cannot be the state a run is in while it runs, and a
/// variant nothing could ever answer with would be a state that is documentation
/// pretending to be a value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "state")]
pub enum UninstallRemoval {
    /// Everything is enumerated with its size, and nothing has been removed.
    #[serde(rename = "surveyed")]
    Surveyed,
    /// The tier and the manifest were agreed to, and this run changes nothing — the
    /// state a rehearsal ends in.
    #[serde(rename = "confirmed")]
    Confirmed,
    /// Everything the manifest named as going is gone.
    #[serde(rename = "complete")]
    Complete {
        /// The credentials this destroyed, said rather than left to be inferred.
        credentials: Vec<String>,
        /// What went, by the name the manifest gave it.
        gone: Vec<String>,
    },
    /// Some of it could not be removed, and each of those is named with how to
    /// finish it by hand.
    #[serde(rename = "partial")]
    Partial {
        /// The credentials this destroyed.
        credentials: Vec<String>,
        /// What went, by the name the manifest gave it.
        gone: Vec<String>,
        /// What is still there, and how to remove it.
        left: Vec<super::UninstallLeft>,
    },
}

/// What is to happen to content the media server has no rating for.
///
/// A choice rather than a default, because a great deal of content carries no rating
/// and either answer is wrong for somebody: holding it back makes legitimate content
/// invisible, and letting it through lets through the one thing nobody vetted.
///
/// Spelled `HeldBack` and `LetThrough` rather than blocked and allowed, because
/// [`Allowed`] is the shape this sits on and a field called `unrated: Allowed` would
/// read as the opposite of what it is.
///
/// A word rather than a flag on both the read and the write, so the setting is named
/// the same in the answer that reports it as in the call that made it — and so neither
/// [`Access`] nor the report built from it becomes a row of four unlabelled booleans.
///
/// Letting it through is the default because it is the media server's: a new account
/// is made holding nothing back, so that is the state an account is found in rather
/// than a decision anybody took.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Unrated {
    /// Content with no rating is held back.
    #[serde(rename = "held-back")]
    HeldBack,
    /// Content with no rating is let through.
    #[serde(rename = "let-through")]
    LetThrough,
}

/// Something lemonfiber cannot act on, named rather than passed over.
///
/// Written for a migration survey and read by two reports now. The second is the
/// status of a stack the operator maintains themselves, where a service declaring an
/// API this build cannot reach is named the same way — what, and why — rather than
/// being dropped from every feature that would have used it. One shape for both,
/// because "named rather than passed over" is the whole of what either is saying and
/// two shapes would be two ways of saying it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnsupportedReport {
    /// Why lemonfiber cannot act on it, in the operator's terms.
    pub because: String,
    /// What was found, by the name the thing that found it gives it — a project and
    /// service for a survey, a service id for a stack lemonfiber runs.
    pub what: String,
}

/// The operator's own overlay, named because nothing in it was held to the model.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Unvetted {
    /// Where the overlay is.
    pub overlay: String,
    /// Every service it changes, by the names it gives them.
    pub services: Vec<String>,
}

/// An install of a part the bundle installs that a start offers, by the plugin and the
/// offer its install reads as.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpInstallOffered {
    /// The name its install's offer goes by, where it could be read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The plugin the bundle installs, in place of the stack's own service of its id.
    pub plugin: String,
    /// Why its install could not be read, where it could not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// What one service's update came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateApplied {
    /// What went wrong, where anything did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// How it ended.
    pub ending: super::Ending,
    /// The version it was standing on before the run.
    pub from: String,
    /// How it could be put back, given how it ended.
    pub reversal: super::UpdateReversal,
    /// The service it is about.
    pub service: String,
    /// The version the run was moving it to.
    pub to: String,
}

/// What updating one service would change.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateChange {
    /// What the step means, in the words an operator decides on.
    pub because: String,
    /// The version it is standing on now.
    pub current: String,
    /// Whether taking it is a step nothing walks back.
    pub irreversible: bool,
    /// How large the step between them is.
    pub jump: super::Jump,
    /// Whether lemonfiber refuses to take it at all.
    pub refused: bool,
    /// The service, by its manifest id, which is also its Compose service name.
    pub service: String,
    /// The version this build pins for it.
    pub target: String,
}

/// Where this copy of lemonfiber stands, and what moving it would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateReport {
    /// What updating leaves alone, and what it needs afterwards.
    pub afterwards: String,
    /// The version the operator asked to move to, where they asked for one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asked: Option<String>,
    /// Where the running binary is, with any link followed, or nothing where this
    /// machine would not say.
    ///
    /// The answer to which of several copies on a search path is the one that ran, so
    /// a version somebody quotes can be attributed to a file rather than to a name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// What a release brings besides the program, and when any of it is fetched.
    pub carries: String,
    /// What the version on offer says it changed, as its release page words it.
    ///
    /// The question an operator is actually weighing. Carried as the notes were
    /// written rather than taken apart here, because what a surface does with them
    /// is a surface's business — a terminal flattens them, a browser renders them,
    /// and a script wants them as they came.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    /// Exactly what to type, where there is something exact to type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Whether the version named can read the configuration on this machine.
    ///
    /// Only where a version was named, since it is the question a downgrade asks and
    /// nothing else does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// How this copy got onto the machine.
    pub installed: super::Installed,
    /// Why there is nothing exact to type, where there is not; or, where typing the
    /// command is not the whole of the move, what has to follow it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instead: Option<String>,
    /// The newest version released, where the check could read one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offered: Option<String>,
    /// The tool that owns this copy, where one does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Whether the directory holding the running binary can be written to.
    ///
    /// Nothing where it was not asked, which is every copy a package manager owns —
    /// replacing one of those is that tool's business and not this one's. Asked by
    /// trying rather than by reading permission bits, and reported rather than acted
    /// on: a copy this operator cannot replace is a thing to say with the path, never
    /// a reason to go looking for a way to become somebody else.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,
    /// The version running now.
    pub running: String,
    /// Which of the states this is.
    pub standing: super::SelfUpdateStanding,
    /// Why availability could not be told, where it could not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub untold: Option<String>,
}

/// How a service could be put back the way it was.
///
/// The distinction is the whole of why this is reported rather than left to be
/// worked out: pinning the previous image again is a minute's work, and restoring a
/// backup is an evening. Offering the first where only the second can succeed is
/// worse than offering nothing, because it is acted on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UpdateReversal {
    /// Nothing opened its state on the newer image, so the previous one runs again
    /// exactly as it did.
    #[serde(rename = "rollback")]
    Rollback,
    /// The newer image was started, so its state may already have been migrated and
    /// the backup is the only way back.
    #[serde(rename = "restore")]
    Restore,
}

/// Where the stack stands against the versions this build pins.
///
/// Exactly one of these is true of a run at a time. A surface that had to say
/// "updates available, and also partly applied" would be reporting the question
/// rather than the answer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UpdateState {
    /// Every service is standing on the version pinned for it.
    #[serde(rename = "current")]
    Current,
    /// Newer versions are pinned than what is running, and nothing has been applied.
    #[serde(rename = "updates-available")]
    UpdatesAvailable,
    /// Everything that had an update took it.
    #[serde(rename = "updated")]
    Updated,
    /// Some were updated and one was not; the run stopped there.
    #[serde(rename = "partial")]
    Partial,
    /// Nothing was updated, and the run stopped where it did.
    #[serde(rename = "failed")]
    Failed,
}

/// One media type an upgrade covers: its chosen quality, that quality's cost, and —
/// once confirmed — what became of asking its service to re-search.
///
/// Reported per media type rather than as one figure, because each type carries its
/// own preset and so its own cost: film at maximum and television at space-saving are
/// upgraded to different bars, and a single number would misstate one of them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpgradeMedia {
    /// The media type — `tv` or `movies`.
    pub media_type: String,
    /// What became of the re-search, or `None` where the upgrade was not confirmed
    /// and only the cost was stated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<super::Triggered>,
    /// The preset in force for it.
    pub preset: String,
    /// Roughly what an hour of it costs at that preset.
    pub size_per_hour: String,
}

/// What upgrading existing content did, or — unconfirmed — would do.
///
/// Upgrading re-acquires the existing library at the chosen quality, which is a
/// large, bandwidth-expensive operation, so it is a separate explicit action whose
/// cost is stated before it runs and which does nothing until confirmed. Each curator
/// re-searches against its own current cutoff, so the report speaks per media type
/// rather than asserting one preset across the library.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpgradeReport {
    /// Whether the operator confirmed; without it nothing was triggered, only the
    /// cost stated.
    pub confirmed: bool,
    /// Per media type: its preset, that preset's cost, and — confirmed — the outcome.
    pub media: Vec<super::UpgradeMedia>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}
