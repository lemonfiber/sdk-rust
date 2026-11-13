/// `POST /api/actions/accept`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct AcceptAction {
    /// The check a warning is being answered for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<String>,
    /// Whether the checks that disturb the running system are included.
    pub disruptive: bool,
}

impl crate::Action for AcceptAction {
    const NAME: &str = "accept";
    const REHEARSAL: bool = false;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/adopt`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct AdoptAction {
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for AdoptAction {
    const NAME: &str = "adopt";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}

/// `POST /api/actions/ask`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct AskAction {
    /// Who an action addressed to somebody in the household is for, as they sign in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The thing an action is about, by its id: the title or episode a player's progress
    /// is on, by the identifier the shelf lists it under, the title a member asks for, by
    /// the identifier their search listed it under, or the wake a phone confirms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the title asked for is a series, a film, music or books, as a page of a
    /// member's search names it: `tv`, `movies`, `music` or `books`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The seasons of a series asked for. None names every season not already here or
    /// on its way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<u32>>,
    /// What is asked for within music or books, as a page of a member's search names
    /// it: `album` or `artist`, `book`, `author` or `series`. Absent asks for an album
    /// or a book.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Whether to rehearse: write nothing, and answer with what the action would do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl crate::Action for AskAction {
    const NAME: &str = "ask";
    const REHEARSAL: bool = true;
    const CONSENT: &[&str] = &[];
}
