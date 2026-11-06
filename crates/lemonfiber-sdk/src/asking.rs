use serde::de::DeserializeOwned;
use serde::Serialize;

pub(crate) const ACTIONS: &str = "/api/actions/";

/// A read the web API answers with an envelope: a struct holding what it takes.
pub trait Read: Serialize {
    /// What it answers with, typed by its kind.
    type Answer: Answer;
    /// The path it is served at, a `{segment}` for each part the struct fills.
    const PATH: &str;
    /// The segments of the path the struct fills.
    const SEGMENTS: &[&str];
    /// The kinds it answers with.
    const KINDS: &[&str];
}

/// A read the web API answers with a file: a struct holding what it takes.
pub trait FileRead: Serialize {
    /// The path it is served at, a `{segment}` for each part the struct fills.
    const PATH: &str;
    /// The segments of the path the struct fills.
    const SEGMENTS: &[&str];
}

/// What a read answers with.
pub trait Answer: DeserializeOwned {
    /// Whether it is read from an envelope's kind and data together rather than from the
    /// data alone, because the read answers with more than one kind.
    const TAGGED: bool;
}

/// An action `POST /api/actions/<name>` takes: a struct holding its arguments.
pub trait Action: Serialize {
    /// Its name.
    const NAME: &str;
    /// Whether it takes `dry_run`, so it can be rehearsed first.
    const REHEARSAL: bool;
    /// The arguments that carry the operator's consent.
    const CONSENT: &[&str];
}

/// One code a refusal may carry, as the contract lists it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalCode {
    /// The code: `ADMIT-4`.
    pub code: &'static str,
    /// Its name in the core's registry: `NOT_ADMITTED`.
    pub name: &'static str,
    /// The status it is answered with.
    pub status: u16,
    /// The registry's line about it.
    pub description: &'static str,
}

impl RefusalCode {
    /// The listed code `code` is, where the contract lists it.
    #[must_use]
    pub fn listed(code: &str) -> Option<&'static Self> {
        crate::contract::refusals::ALL
            .iter()
            .find(|listed| listed.code == code)
    }
}

/// One action a key may call, and what the contract says of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyCallable {
    /// The action's name.
    pub action: &'static str,
    /// Whether it disturbs the running system.
    pub disturbs: bool,
    /// Whether it takes `dry_run`.
    pub rehearsal: bool,
    /// Whether calling it again with the same arguments leaves the stack as one call did.
    pub idempotent: bool,
    /// The code a moved offer is refused with, where the action answers an offer.
    pub moved: Option<&'static RefusalCode>,
}
