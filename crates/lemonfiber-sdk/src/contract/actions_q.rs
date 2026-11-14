/// `POST /api/actions/quality-reapply`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualityReapplyAction {
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for QualityReapplyAction {
    const NAME: &str = "quality-reapply";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/quality-set`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualitySetAction {
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

impl crate::Action for QualitySetAction {
    const NAME: &str = "quality-set";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["confirm"];
}

/// `POST /api/actions/quality-upgrade`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualityUpgradeAction {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for QualityUpgradeAction {
    const NAME: &str = "quality-upgrade";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["confirm"];
}
