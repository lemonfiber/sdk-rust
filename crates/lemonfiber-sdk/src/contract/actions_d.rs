/// `POST /api/actions/diagnose`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Diagnose {
    /// The group of checks, or the one check, a diagnosis is narrowed to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only: Option<String>,
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
}

impl crate::Action for Diagnose {
    const NAME: &'static str = "diagnose";
    const REHEARSAL: bool = false;
    const CONSENT: &'static [&'static str] = &[];
}

/// `POST /api/actions/down`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Down {
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

impl crate::Action for Down {
    const NAME: &'static str = "down";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

/// `POST /api/actions/downloads-pause`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct DownloadsPause {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for DownloadsPause {
    const NAME: &'static str = "downloads-pause";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

/// `POST /api/actions/downloads-resume`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct DownloadsResume {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for DownloadsResume {
    const NAME: &'static str = "downloads-resume";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}
