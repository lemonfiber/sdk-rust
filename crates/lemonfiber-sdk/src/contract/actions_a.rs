/// `POST /api/actions/accept`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Accept {
    /// The check a warning is being answered for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<String>,
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
}

impl crate::Action for Accept {
    const NAME: &'static str = "accept";
    const REHEARSAL: bool = false;
    const CONSENT: &'static [&'static str] = &[];
}

/// `POST /api/actions/adopt`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct Adopt {
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for Adopt {
    const NAME: &'static str = "adopt";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
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

impl crate::Action for Ask {
    const NAME: &'static str = "ask";
    const REHEARSAL: bool = true;
    const CONSENT: &'static [&'static str] = &[];
}
