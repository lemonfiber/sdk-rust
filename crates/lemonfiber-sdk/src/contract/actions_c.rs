/// `POST /api/actions/companion-pair`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct CompanionPair {}

impl crate::Action for CompanionPair {
    const NAME: &'static str = "companion-pair";
    const REHEARSAL: bool = false;
    const CONSENT: &'static [&'static str] = &[];
}

/// `POST /api/actions/config-set`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct ConfigSet {
    /// Whether anything still downloading is let finish before the stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /// The configuration key to change.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// What to change the key to, written as exactly itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for ConfigSet {
    const NAME: &'static str = "config-set";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

/// `POST /api/actions/credentials-revoke`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct CredentialsRevoke {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The plugin an update or a removal acts on, by the id the record of what is
    /// installed lists it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
    /// The credential a grant to a third-party plugin is of, by the setting the
    /// credentials read lists it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for CredentialsRevoke {
    const NAME: &'static str = "credentials-revoke";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}
