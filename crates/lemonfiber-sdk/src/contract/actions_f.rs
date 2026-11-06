/// `POST /api/actions/forget`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Forget {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Forget {
    const NAME: &str = "forget";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["confirm"];
}
