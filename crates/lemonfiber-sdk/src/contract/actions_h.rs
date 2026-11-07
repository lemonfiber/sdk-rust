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

impl crate::Action for HostingInstall {
    const NAME: &str = "hosting-install";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
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

impl crate::Action for HostingRemove {
    const NAME: &str = "hosting-remove";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
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

impl crate::Action for HouseholdAllow {
    const NAME: &str = "household-allow";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
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

impl crate::Action for HouseholdApprove {
    const NAME: &str = "household-approve";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
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

impl crate::Action for HouseholdDecline {
    const NAME: &str = "household-decline";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
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

impl crate::Action for HouseholdHandoff {
    const NAME: &str = "household-handoff";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}
