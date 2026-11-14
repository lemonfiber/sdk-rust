/// `POST /api/actions/plugin-install`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginInstallAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Where a plugin comes from: its name in the catalogue, its directory on this
    /// machine or the `plugin.toml` inside it, or a git repository, at a branch, tag or
    /// commit named after its last `@`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Every value a recipe would carry elsewhere that is approved, as `value@destination`,
    /// every privileged shape, as `shape@service`, and every credential granted to a
    /// plugin, as `setting@plugin`, exactly as the reading lists them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved: Option<Vec<String>>,
    /// Every value the operator supplies for an input a recipe asks for, as `name=value`.
    /// What follows the `=` may be a secret, and is never repeated back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for PluginInstallAction {
    const NAME: &str = "plugin-install";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer", "approved"];
}

/// `POST /api/actions/plugin-prove`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginProveAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The plugin an update or a removal acts on, by the id the record of what is
    /// installed lists it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for PluginProveAction {
    const NAME: &str = "plugin-prove";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/plugin-remove`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginRemoveAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The plugin an update or a removal acts on, by the id the record of what is
    /// installed lists it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for PluginRemoveAction {
    const NAME: &str = "plugin-remove";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}

/// `POST /api/actions/plugin-run`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginRunAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The plugin an update or a removal acts on, by the id the record of what is
    /// installed lists it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
    /// The demand recipe a run asks for, by the id the plugin's reading lists it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipe: Option<String>,
    /// Every value a recipe would carry elsewhere that is approved, as `value@destination`,
    /// every privileged shape, as `shape@service`, and every credential granted to a
    /// plugin, as `setting@plugin`, exactly as the reading lists them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved: Option<Vec<String>>,
    /// Every value the operator supplies for an input a recipe asks for, as `name=value`.
    /// What follows the `=` may be a secret, and is never repeated back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for PluginRunAction {
    const NAME: &str = "plugin-run";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer", "approved"];
}

/// `POST /api/actions/plugin-update`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginUpdateAction {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The plugin an update or a removal acts on, by the id the record of what is
    /// installed lists it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
    /// Where a plugin comes from: its name in the catalogue, its directory on this
    /// machine or the `plugin.toml` inside it, or a git repository, at a branch, tag or
    /// commit named after its last `@`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Every value a recipe would carry elsewhere that is approved, as `value@destination`,
    /// every privileged shape, as `shape@service`, and every credential granted to a
    /// plugin, as `setting@plugin`, exactly as the reading lists them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved: Option<Vec<String>>,
    /// Every value the operator supplies for an input a recipe asks for, as `name=value`.
    /// What follows the `=` may be a secret, and is never repeated back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for PluginUpdateAction {
    const NAME: &str = "plugin-update";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer", "approved"];
}

/// `POST /api/actions/pull`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PullAction {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for PullAction {
    const NAME: &str = "pull";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/push-woken`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PushWokenAction {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The thing an action is about, by its id: the title or episode a player's progress
    /// is on, by the identifier the shelf lists it under, the title a member asks for, by
    /// the identifier their search listed it under, or the wake a phone confirms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for PushWokenAction {
    const NAME: &str = "push-woken";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}
