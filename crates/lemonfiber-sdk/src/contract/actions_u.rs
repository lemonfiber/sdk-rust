/// `POST /api/actions/undo`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct UndoAction {
    /// The stamp of the run to put back, as the history writes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for UndoAction {
    const NAME: &str = "undo";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/uninstall`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct UninstallAction {
    /// Whether anything still downloading is let finish before the stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Which of the four removals an uninstall was asked for.
    ///
    /// A word rather than a level, for the reason `policy` is a word: they are four
    /// decisions and not a scale, and one this build does not know is refused by
    /// name rather than falling to whichever removal the shape happened to default
    /// to — which on this subject would be a library gone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for UninstallAction {
    const NAME: &str = "uninstall";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer", "confirm"];
}

/// `POST /api/actions/up`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct UpAction {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// The services to act on, leaving the rest of the form alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for UpAction {
    const NAME: &str = "up";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/update`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct UpdateAction {
    /// Whether anything still downloading is let finish before the stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /// The one service to act on instead of the whole stack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for UpdateAction {
    const NAME: &str = "update";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer", "confirm"];
}
