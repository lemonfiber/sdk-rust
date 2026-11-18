//! What a household member's surface may not show, as the contract publishes it in
//! [`crate::contract::household_words`].

/// One name lemonfiber declares, with what it names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredName {
    /// The name.
    pub name: &'static str,
    /// What it names.
    pub names: &'static str,
}

/// One sentence a member may not be shown, with what in it is named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HouseholdExample {
    /// The sentence.
    pub text: &'static str,
    /// What in it a member may not be shown.
    pub names: &'static str,
}
