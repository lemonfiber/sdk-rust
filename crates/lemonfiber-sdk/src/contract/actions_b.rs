/// `POST /api/actions/backup`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Backup {
    /// The one service to act on instead of the whole stack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Backup {
    const NAME: &'static str = "backup";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

/// `POST /api/actions/bandwidth`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Bandwidth {
    /// How much of the line downloads may take, as a share or a rate.
    ///
    /// Carried as it was written and read in the core, like `unrated` above and for
    /// the same reason: a surface that decided what `50%` meant would be a second
    /// answer to the question, and the two would part company on the first change
    /// to either — which a household would meet as an evening that went wrong on
    /// one surface and not on another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub down: Option<String>,
    /// How much of it uploads may take, set apart from the download.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub up: Option<String>,
    /// The hours the household is awake, as `HH:MM-HH:MM` on the wall clock.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
    /// What the line carries, as `<down>/<up>`, where the operator knows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
    /// A monthly allowance for what the stack itself moves.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<String>,
    /// What is to happen when that cap is reached, decided in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceeded: Option<String>,
    /// How many minutes to lift the limits for, and no longer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unrestricted_for: Option<u64>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Bandwidth {
    const NAME: &'static str = "bandwidth";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}
