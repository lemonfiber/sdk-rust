/// `GET /api/uninstall`, answered with `uninstall`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Uninstall {
    /// The `tier` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl crate::Answer for super::Uninstall {
    const TAGGED: bool = false;
}

impl crate::Read for Uninstall {
    type Answer = super::Uninstall;
    const PATH: &'static str = "/api/uninstall";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["uninstall"];
}

/// `GET /api/update`, answered with `self-update` or `update`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Update {
    /// The `what` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what: Option<String>,
    /// The `to` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// What `GET /api/update` answers with, by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum UpdateAnswer {
    /// The `self-update` kind.
    #[serde(rename = "self-update")]
    SelfUpdate(Box<super::UpdateReport>),
    /// The `update` kind.
    #[serde(rename = "update")]
    Update(Box<super::StackUpdateReport>),
}

impl crate::Answer for UpdateAnswer {
    const TAGGED: bool = true;
}

impl crate::Read for Update {
    type Answer = UpdateAnswer;
    const PATH: &'static str = "/api/update";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["self-update", "update"];
}
