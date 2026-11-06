/// `POST /api/actions/reissue`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Reissue {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Reissue {
    const NAME: &'static str = "reissue";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

/// `POST /api/actions/remove`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Remove {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Remove {
    const NAME: &'static str = "remove";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

/// `POST /api/actions/repair`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Repair {
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The checks whose repairs were agreed to, as that offer names them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreed: Option<Vec<String>>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Repair {
    const NAME: &'static str = "repair";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "agreed", "confirm"];
}

/// `POST /api/actions/reset`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Reset {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Reset {
    const NAME: &'static str = "reset";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

/// `POST /api/actions/restart`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Restart {
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

impl crate::Action for Restart {
    const NAME: &'static str = "restart";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

/// `POST /api/actions/restore`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Restore {
    /// The backup to restore from, by the name it was written under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<String>,
    /// Whether re-pointing to this machine's data root was accepted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repoint: Option<bool>,
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

impl crate::Action for Restore {
    const NAME: &'static str = "restore";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "confirm"];
}
