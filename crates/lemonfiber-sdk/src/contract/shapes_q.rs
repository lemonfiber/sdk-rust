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
