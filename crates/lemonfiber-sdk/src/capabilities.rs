use std::collections::{BTreeMap, BTreeSet};
use std::time::SystemTime;

use crate::asking::ACTIONS;
use crate::contract::{CapabilityState, CredentialScope};
use crate::{Action, Read};

/// Every capability a stack has, by the path it is served at, as it stood for the
/// credential that asked when it was read. A capability the stack does not have is absent;
/// one this client does not know is kept.
#[derive(Debug, Clone, PartialEq)]
pub struct CapabilitySet {
    /// Each capability's path, to what it comes to for the credential that asked.
    pub states: BTreeMap<String, CapabilityState>,
    /// When the answer arrived. A set not read again lately is an opinion, not a fact.
    pub read_at: SystemTime,
    /// Whose credential asked.
    pub scope: CredentialScope,
    /// The stack's own identifier, where it has one.
    pub stack: Option<String>,
}

impl CapabilitySet {
    /// What the capability served at `path` comes to, none where the stack does not have it.
    #[must_use]
    pub fn of(&self, path: &str) -> Option<&CapabilityState> {
        self.states.get(path)
    }

    /// What the action `A` comes to, none where the stack does not have it.
    #[must_use]
    pub fn of_action<A: Action>(&self) -> Option<&CapabilityState> {
        self.of(&format!("{ACTIONS}{}", A::NAME))
    }

    /// What the read `R` comes to, none where the stack does not have it.
    #[must_use]
    pub fn of_read<R: Read>(&self) -> Option<&CapabilityState> {
        self.of(R::PATH)
    }

    /// The path of every capability in `state`.
    #[must_use]
    pub fn in_state(&self, state: &CapabilityState) -> BTreeSet<&str> {
        self.states
            .iter()
            .filter(|(_, held)| *held == state)
            .map(|(path, _)| path.as_str())
            .collect()
    }
}
