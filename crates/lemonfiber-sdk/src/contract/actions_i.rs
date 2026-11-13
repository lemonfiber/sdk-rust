/// `POST /api/actions/internal-certificate-replace`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct InternalCertificateReplaceAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for InternalCertificateReplaceAction {
    const NAME: &str = "internal-certificate-replace";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/invite`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct InviteAction {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The libraries an invitation lets them open, by the names the media server
    /// gives those libraries. Empty is every one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<String>>,
    /// The age above which an invitation has the media server hold things back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age_limit: Option<u32>,
    /// What is to happen to content the media server has no rating for.
    ///
    /// A word rather than a switch, because the choice has a cost either way and a
    /// switch has a default nobody can see: `block` holds unrated content back, and
    /// `allow` lets it through. Carried as it was written and read next door, so a
    /// word this build does not know is refused by name rather than falling to
    /// whichever answer the shape happened to default to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unrated: Option<String>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for InviteAction {
    const NAME: &str = "invite";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["confirm"];
}
