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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<StoredRemoval, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for StoredRemoval {
    const TAG: Option<&str> = Some("state");
    const KNOWN: &[&str] = &["not-asked", "unconfirmed", "done"];
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
#[non_exhaustive]
pub enum Stream {
    /// Standard output.
    #[serde(rename = "stdout")]
    Stdout,
    /// Standard error.
    #[serde(rename = "stderr")]
    Stderr,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
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
#[non_exhaustive]
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
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
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
