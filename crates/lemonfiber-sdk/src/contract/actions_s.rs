/// `POST /api/actions/search`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SearchAction {
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
    /// What to follow, named as a person would say it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// The season a followed show is narrowed to, or every season where absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
}

impl crate::Action for SearchAction {
    const NAME: &str = "search";
    const REHEARSAL: bool = false;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/seed`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct SeedAction {
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for SeedAction {
    const NAME: &str = "seed";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/settings-clear`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct SettingsClearAction {
    /// The settings to put back to their defaults, by the names the settings read lists
    /// them under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<String>>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for SettingsClearAction {
    const NAME: &str = "settings-clear";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/settings-set`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct SettingsSetAction {
    /// Each setting to change, by the name the settings read lists it under, with the
    /// value it is to hold, written as exactly itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<std::collections::BTreeMap<String, String>>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for SettingsSetAction {
    const NAME: &str = "settings-set";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/space`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct SpaceAction {
    /// The categories of what the offer holds as costing nothing to take, orphaned or extracted; all it holds where none is named.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only: Option<Vec<String>>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for SpaceAction {
    const NAME: &str = "space";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/stop-seeding`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct StopSeedingAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The completed download the client is to be asked to let go, by the name both
    /// sides use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for StopSeedingAction {
    const NAME: &str = "stop-seeding";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/support`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct SupportAction {
    /// Whether to produce the bundle, rather than say what one would hold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    /// How many log lines to take from each service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<u32>,
    /// Whether media filenames are shown rather than replaced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filenames: Option<bool>,
    /// The settings to show as they are, named as the bundle names them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reveal: Option<Vec<String>>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for SupportAction {
    const NAME: &str = "support";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["confirm"];
}

/// `POST /api/actions/switch`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct SwitchAction {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for SwitchAction {
    const NAME: &str = "switch";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}
