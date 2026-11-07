/// `POST /api/actions/walkthrough`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Walkthrough {
    /// The one thing to add end to end, as it would be said.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
}

impl crate::Action for Walkthrough {
    const NAME: &str = "walkthrough";
    const REHEARSAL: bool = false;
    const CONSENT: &[&str] = &[];
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

impl crate::Action for Watch {
    const NAME: &str = "watch";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
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

impl crate::Action for Watched {
    const NAME: &str = "watched";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
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

impl crate::Action for WiringFill {
    const NAME: &str = "wiring-fill";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &["offer"];
}
