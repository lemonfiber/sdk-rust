/// `POST /api/actions/diagnose`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct DiagnoseAction {
    /// The group of checks, or the one check, a diagnosis is narrowed to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only: Option<String>,
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
}

impl crate::Action for DiagnoseAction {
    const NAME: &str = "diagnose";
    const REHEARSAL: bool = false;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/down`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct DownAction {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// The services to act on, leaving the rest of the form alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// Whether anything still downloading is let finish before the stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for DownAction {
    const NAME: &str = "down";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/downloads-pause`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct DownloadsPauseAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for DownloadsPauseAction {
    const NAME: &str = "downloads-pause";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/downloads-resume`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct DownloadsResumeAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for DownloadsResumeAction {
    const NAME: &str = "downloads-resume";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}
