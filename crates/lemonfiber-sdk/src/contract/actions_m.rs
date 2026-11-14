/// `POST /api/actions/migrate-adopt`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateAdoptAction {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for MigrateAdoptAction {
    const NAME: &str = "migrate-adopt";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["confirm"];
}

/// `POST /api/actions/migrate-beside`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateBesideAction {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for MigrateBesideAction {
    const NAME: &str = "migrate-beside";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["confirm"];
}

/// `POST /api/actions/migrate-import`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateImportAction {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for MigrateImportAction {
    const NAME: &str = "migrate-import";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["confirm"];
}

/// `POST /api/actions/migrate-replace`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateReplaceAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for MigrateReplaceAction {
    const NAME: &str = "migrate-replace";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}
