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
