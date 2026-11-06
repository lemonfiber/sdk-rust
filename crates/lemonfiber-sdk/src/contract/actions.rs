//! Every action the web API takes: a struct for each, holding its arguments.

/// `POST /api/actions/accept`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Accept {
    /// The check a warning is being answered for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<String>,
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
}

/// `POST /api/actions/adopt`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Adopt {
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/ask`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Ask {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The title an action is about: the title or episode a player's progress is on,
    /// by the identifier the shelf lists it under, or the title a member asks for, by
    /// the identifier their search listed it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the title asked for is a series or a film, as a page of a member's search
    /// names it: `tv` or `movies`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The seasons of a series asked for. None names every season not already here or
    /// on its way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<u32>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

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

/// `POST /api/actions/companion-pair`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct CompanionPair {}

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

/// `POST /api/actions/diagnose`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Diagnose {
    /// The group of checks, or the one check, a diagnosis is narrowed to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only: Option<String>,
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
}

/// `POST /api/actions/down`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Down {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// The services to act on, leaving the rest of the form alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// Whether anything still downloading is let finish before the stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/downloads-pause`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct DownloadsPause {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/downloads-resume`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct DownloadsResume {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

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

/// `POST /api/actions/hosting-install`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct HostingInstall {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// Which command this machine is being asked to keep running, or stop keeping.
    ///
    /// A word rather than a switch, for the reason `policy` is one: there is more than
    /// one of them, and a word this build does not know is refused by name rather than
    /// silently taken for whichever the shape happened to default to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/hosting-remove`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct HostingRemove {
    /// Which command this machine is being asked to keep running, or stop keeping.
    ///
    /// A word rather than a switch, for the reason `policy` is one: there is more than
    /// one of them, and a word this build does not know is refused by name rather than
    /// silently taken for whichever the shape happened to default to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/household-allow`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct HouseholdAllow {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What is to happen to what the household asks for, as it is written.
    ///
    /// A word rather than a switch, for the reason `unrated` is one: there are three
    /// answers rather than two, and a word this build does not know is refused by name
    /// instead of falling to whichever arrangement the shape happened to default to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// How many requests a period allows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<u32>,
    /// How long that period is, in days.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<u32>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/household-approve`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct HouseholdApprove {
    /// The request being ruled on, by the number the request service files it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<i64>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/household-decline`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct HouseholdDecline {
    /// The request being ruled on, by the number the request service files it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<i64>,
    /// Why a request is being turned down, or why a service was chosen to fill a
    /// capability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/household-handoff`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct HouseholdHandoff {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/internal-certificate-replace`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct InternalCertificateReplace {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/invite`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Invite {
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

/// `POST /api/actions/migrate-adopt`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateAdopt {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/migrate-beside`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateBeside {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/migrate-import`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateImport {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/migrate-replace`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct MigrateReplace {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/plugin-install`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginInstall {
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

/// `POST /api/actions/plugin-prove`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginProve {
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

/// `POST /api/actions/plugin-remove`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginRemove {
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

/// `POST /api/actions/plugin-run`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginRun {
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

/// `POST /api/actions/plugin-update`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PluginUpdate {
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

/// `POST /api/actions/pull`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Pull {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/quality-reapply`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualityReapply {
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/quality-set`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualitySet {
    /// The quality preset, or the music format, as it is written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// The media type a quality choice applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/quality-upgrade`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct QualityUpgrade {
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

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

/// `POST /api/actions/search`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Search {
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
    /// What to follow, named as a person would say it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// The season a followed show is narrowed to, or every season where absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
}

/// `POST /api/actions/seed`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Seed {
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/settings-clear`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct SettingsClear {
    /// The settings to put back to their defaults, by the names the settings read lists
    /// them under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<String>>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/settings-set`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct SettingsSet {
    /// Each setting to change, by the name the settings read lists it under, with the
    /// value it is to hold, written as exactly itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<std::collections::BTreeMap<String, String>>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/space`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Space {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/stop-seeding`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct StopSeeding {
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// The completed download the client is to be asked to let go, by the name both
    /// sides use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/support`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Support {
    /// Whether to produce the bundle, rather than say what one would hold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    /// How many log lines to take from each service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<u32>,
    /// Whether media filenames are shown rather than replaced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filenames: Option<bool>,
    /// The settings to show as they are, named as the bundle names them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reveal: Option<Vec<String>>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/switch`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Switch {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/undo`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Undo {
    /// The stamp of the run to put back, as the history writes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/uninstall`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Uninstall {
    /// Whether anything still downloading is let finish before the stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Whether a cost the action would incur was agreed to in advance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Which of the four removals an uninstall was asked for.
    ///
    /// A word rather than a level, for the reason `policy` is a word: they are four
    /// decisions and not a scale, and one this build does not know is refused by
    /// name rather than falling to whichever removal the shape happened to default
    /// to — which on this subject would be a library gone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/up`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Up {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// The services to act on, leaving the rest of the form alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/update`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Update {
    /// Whether anything still downloading is let finish before the stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    /// The one service to act on instead of the whole stack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
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

/// `POST /api/actions/walkthrough`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Walkthrough {
    /// The one thing to add end to end, as it would be said.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
}

/// `POST /api/actions/watch`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Watch {
    /// The forms to act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<String>>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/watched`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Watched {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The title an action is about: the title or episode a player's progress is on,
    /// by the identifier the shelf lists it under, or the title a member asks for, by
    /// the identifier their search listed it under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// How far into it the member is, in whole seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// Whether the member finished it. Absent says nothing about finishing, which is
    /// a position like any other.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended: Option<bool>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

/// `POST /api/actions/wiring-fill`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct WiringFill {
    /// The one service to act on instead of the whole stack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// What was read before answering — the offer a repair's yes was read in, the
    /// listing a restore's was, what a replacement would stop — as it named itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Why a request is being turned down, or why a service was chosen to fill a
    /// capability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The capability whose filler is being chosen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Accept {
    const NAME: &'static str = "accept";
    const REHEARSAL: bool = false;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Adopt {
    const NAME: &'static str = "adopt";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Ask {
    const NAME: &'static str = "ask";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Backup {
    const NAME: &'static str = "backup";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Bandwidth {
    const NAME: &'static str = "bandwidth";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for CompanionPair {
    const NAME: &'static str = "companion-pair";
    const REHEARSAL: bool = false;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for ConfigSet {
    const NAME: &'static str = "config-set";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for CredentialsRevoke {
    const NAME: &'static str = "credentials-revoke";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for Diagnose {
    const NAME: &'static str = "diagnose";
    const REHEARSAL: bool = false;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Down {
    const NAME: &'static str = "down";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for DownloadsPause {
    const NAME: &'static str = "downloads-pause";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for DownloadsResume {
    const NAME: &'static str = "downloads-resume";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for Forget {
    const NAME: &'static str = "forget";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for Grant {
    const NAME: &'static str = "grant";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for HostingInstall {
    const NAME: &'static str = "hosting-install";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for HostingRemove {
    const NAME: &'static str = "hosting-remove";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for HouseholdAllow {
    const NAME: &'static str = "household-allow";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for HouseholdApprove {
    const NAME: &'static str = "household-approve";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for HouseholdDecline {
    const NAME: &'static str = "household-decline";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for HouseholdHandoff {
    const NAME: &'static str = "household-handoff";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for InternalCertificateReplace {
    const NAME: &'static str = "internal-certificate-replace";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for Invite {
    const NAME: &'static str = "invite";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for MigrateAdopt {
    const NAME: &'static str = "migrate-adopt";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for MigrateBeside {
    const NAME: &'static str = "migrate-beside";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for MigrateImport {
    const NAME: &'static str = "migrate-import";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for MigrateReplace {
    const NAME: &'static str = "migrate-replace";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for PluginInstall {
    const NAME: &'static str = "plugin-install";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "approved"];
}

impl crate::Action for PluginProve {
    const NAME: &'static str = "plugin-prove";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for PluginRemove {
    const NAME: &'static str = "plugin-remove";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for PluginRun {
    const NAME: &'static str = "plugin-run";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "approved"];
}

impl crate::Action for PluginUpdate {
    const NAME: &'static str = "plugin-update";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "approved"];
}

impl crate::Action for Pull {
    const NAME: &'static str = "pull";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for QualityReapply {
    const NAME: &'static str = "quality-reapply";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for QualitySet {
    const NAME: &'static str = "quality-set";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for QualityUpgrade {
    const NAME: &'static str = "quality-upgrade";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for Reissue {
    const NAME: &'static str = "reissue";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Remove {
    const NAME: &'static str = "remove";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for Repair {
    const NAME: &'static str = "repair";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "agreed", "confirm"];
}

impl crate::Action for Reset {
    const NAME: &'static str = "reset";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for Restart {
    const NAME: &'static str = "restart";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for Restore {
    const NAME: &'static str = "restore";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "confirm"];
}

impl crate::Action for Search {
    const NAME: &'static str = "search";
    const REHEARSAL: bool = false;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Seed {
    const NAME: &'static str = "seed";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for SettingsClear {
    const NAME: &'static str = "settings-clear";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for SettingsSet {
    const NAME: &'static str = "settings-set";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for Space {
    const NAME: &'static str = "space";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for StopSeeding {
    const NAME: &'static str = "stop-seeding";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

impl crate::Action for Support {
    const NAME: &'static str = "support";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["confirm"];
}

impl crate::Action for Switch {
    const NAME: &'static str = "switch";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Undo {
    const NAME: &'static str = "undo";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Uninstall {
    const NAME: &'static str = "uninstall";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "confirm"];
}

impl crate::Action for Up {
    const NAME: &'static str = "up";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Update {
    const NAME: &'static str = "update";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer", "confirm"];
}

impl crate::Action for Walkthrough {
    const NAME: &'static str = "walkthrough";
    const REHEARSAL: bool = false;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Watch {
    const NAME: &'static str = "watch";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for Watched {
    const NAME: &'static str = "watched";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}

impl crate::Action for WiringFill {
    const NAME: &'static str = "wiring-fill";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &["offer"];
}

/// The name of every action the web API takes.
pub const NAMES: &[&str] = &[
    "accept",
    "adopt",
    "ask",
    "backup",
    "bandwidth",
    "companion-pair",
    "config-set",
    "credentials-revoke",
    "diagnose",
    "down",
    "downloads-pause",
    "downloads-resume",
    "forget",
    "grant",
    "hosting-install",
    "hosting-remove",
    "household-allow",
    "household-approve",
    "household-decline",
    "household-handoff",
    "internal-certificate-replace",
    "invite",
    "migrate-adopt",
    "migrate-beside",
    "migrate-import",
    "migrate-replace",
    "plugin-install",
    "plugin-prove",
    "plugin-remove",
    "plugin-run",
    "plugin-update",
    "pull",
    "quality-reapply",
    "quality-set",
    "quality-upgrade",
    "reissue",
    "remove",
    "repair",
    "reset",
    "restart",
    "restore",
    "search",
    "seed",
    "settings-clear",
    "settings-set",
    "space",
    "stop-seeding",
    "support",
    "switch",
    "undo",
    "uninstall",
    "up",
    "update",
    "walkthrough",
    "watch",
    "watched",
    "wiring-fill",
];
