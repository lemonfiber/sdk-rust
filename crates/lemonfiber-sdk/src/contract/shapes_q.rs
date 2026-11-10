/// What a quality choice may be made from, each in the plain terms the command takes it in.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualityOffered {
    /// Every format music may take, least to most demanding.
    pub formats: Vec<String>,
    /// Every kind of media a choice may be made for apart from everything at once.
    pub media_types: Vec<String>,
    /// Every resolution preset, least to most demanding.
    pub presets: Vec<String>,
}

/// The operator's quality choice, what each preset means, and what the command
/// did with it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualityReport {
    /// What a choice changes, in the stack's words; present where this answers one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<String>,
    /// The global choice first, then each media type set apart from it.
    pub choices: Vec<super::PresetChoice>,
    /// Whether the quality sync config has been hand-edited since lemonfiber wrote it —
    /// the `customised` state, in which the preset is no longer authoritative until
    /// it is deliberately re-asserted. For a reapply, whether an edit was overwritten.
    pub customised: bool,
    /// What became of the choice.
    pub disposition: super::Disposition,
    /// The audio-format choice for music, where one is set — media that has no
    /// resolution, so it is reported apart from the resolution presets rather than
    /// forced into their shape.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub music: Option<super::MusicChoice>,
    /// What a choice may be made from.
    pub offered: super::QualityOffered,
    /// The hand-edited config a reapply replaced — or, rehearsed, would replace — with
    /// the diff of what goes against what lands in its place.
    ///
    /// Absent everywhere else, and absent for a reapply over a config already in
    /// lemonfiber's own hand. Consent given against a yes-or-no is consent to
    /// something the operator was never shown: they know a file they edited is about
    /// to go, and not which of their lines is in it. The lines are masked the way
    /// every stack-file diff is, so a key that drifted is named without its value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwritten: Option<super::StackEdit>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}

/// One curator's queue, and how much of it is stuck.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Queue {
    /// How many items are queued.
    pub depth: u64,
    /// The service whose queue this is.
    pub service: String,
    /// How many of them are stuck rather than progressing.
    pub stuck: u64,
}

/// Whether healing acts, the strikes and grace it acts after, every unhealthy item of the
/// latest pass, what could not be reached, and the newest runs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QueueHealth {
    /// When the latest pass classified the queue, in seconds since the Unix epoch, where
    /// one ever has.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classified_at: Option<u64>,
    /// How long a newly added download is left alone, in seconds.
    pub grace_s: u64,
    /// Whether healing acts, or only classifies.
    pub healing: super::HealingSwitch,
    /// Whether the list may be short, because something could not be read.
    pub incomplete: bool,
    /// Every unhealthy item the latest pass found; a healthy one is left out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<super::Wedged>>,
    /// Where a download client files a curator's downloads outside the curator's folder,
    /// which nothing is acted on past.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mismatch: Option<String>,
    /// Whether this answer is a run that only said what it would do.
    pub rehearsed: bool,
    /// The newest runs first, each a pass that could act, rehearsed or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<super::HealRun>>,
    /// How many consecutive passes flag an item for one reason before it is acted on.
    pub strikes_needed: u32,
    /// Every curator or download client the latest pass could not reach or verify.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unreachable: Option<Vec<super::Unreached>>,
}
