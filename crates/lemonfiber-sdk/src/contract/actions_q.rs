/// `POST /api/actions/quality-reapply`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualityReapply {
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for QualityReapply {
    const NAME: &'static str = "quality-reapply";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

/// `POST /api/actions/quality-set`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualitySet {
    /// The quality preset, or the music format, as it is written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// The media type a quality choice applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for QualitySet {
    const NAME: &'static str = "quality-set";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

/// `POST /api/actions/quality-upgrade`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualityUpgrade {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for QualityUpgrade {
    const NAME: &'static str = "quality-upgrade";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}
