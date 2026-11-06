/// `POST /api/actions/grant`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Grant {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id a player keeps for the device it plays on, which a grant opens a session
    /// for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Grant {
    const NAME: &str = "grant";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}
