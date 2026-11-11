/// `POST /api/actions/notify-choose`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct NotifyChoose {
    /// Each way to be told a member chooses, by the id the operator's offer lists it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for NotifyChoose {
    const NAME: &str = "notify-choose";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}
