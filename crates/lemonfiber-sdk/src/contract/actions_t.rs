/// `POST /api/actions/tune`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct TuneAction {
    /// The thing an action is about, by its id: the title or episode a player's progress
    /// is on, by the identifier the shelf lists it under, the title a member asks for, by
    /// the identifier their search listed it under, or the wake a phone confirms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The live stream a renewal or a stop names, as a channel opened for the member
    /// answered it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
    /// Whether to close the live stream named rather than renew its passes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for TuneAction {
    const NAME: &str = "tune";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}
