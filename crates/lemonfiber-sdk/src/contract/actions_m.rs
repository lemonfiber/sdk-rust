/// `POST /api/actions/migrate-adopt`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateAdopt {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for MigrateAdopt {
    const NAME: &'static str = "migrate-adopt";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

/// `POST /api/actions/migrate-beside`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateBeside {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for MigrateBeside {
    const NAME: &'static str = "migrate-beside";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

/// `POST /api/actions/migrate-import`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateImport {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for MigrateImport {
    const NAME: &'static str = "migrate-import";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

/// `POST /api/actions/migrate-replace`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateReplace {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for MigrateReplace {
    const NAME: &'static str = "migrate-replace";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}
