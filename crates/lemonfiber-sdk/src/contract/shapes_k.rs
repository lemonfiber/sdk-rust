/// One thing lemonfiber keeps on this machine.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Kept {
    /// Where it is, in full.
    pub at: String,
    /// Whether it holds a credential, which is what decides how carefully a copy of
    /// it has to be treated.
    pub secret: bool,
    /// What it is, in the operator's words.
    pub what: String,
    /// Why it is kept.
    pub why: String,
}

/// A key minted as a claim: the link that collects it, and no secret.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeyClaim {
    /// When the claim stops collecting the key, written as every other instant is.
    pub lapses: String,
    /// The key link: `lemonfiber://key` with the address, the certificate fingerprint,
    /// the stack, when the claim lapses and the claim. Shown once, and drawn as a code.
    pub link: String,
    /// The name it was minted under.
    pub name: String,
    /// What the minter said it is for.
    pub purpose: super::KeyPurpose,
    /// What it admits: `read`, `act` or `member:<name>`.
    pub scope: String,
}

/// Every key this machine has minted, and what became of a revoke.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeyListing {
    /// In the order they were minted.
    pub keys: Vec<super::ListedKey>,
    /// What a purpose in the listing is worth, said with it.
    pub purposes: String,
    /// Whether this was a rehearsal: what revoking would come to, with nothing revoked.
    pub rehearsed: bool,
    /// The key this run revoked, where it revoked one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<String>,
}

/// What the minter said a key is for.
///
/// A label and nothing more. The core cannot tell what a program does with a key, so the
/// listing shows this as the minter's own declaration rather than as anything verified.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum KeyPurpose {
    /// Home Assistant.
    #[serde(rename = "home-assistant")]
    HomeAssistant,
    /// An assistant reaching the stack through the Model Context Protocol.
    #[serde(rename = "mcp")]
    Mcp,
    /// Anything else.
    #[serde(rename = "other")]
    Other,
}

/// Where a key stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum KeyState {
    /// Admitted within its scope.
    #[serde(rename = "active")]
    Active,
    /// Minted as a claim and not collected yet: it has no secret, admits nothing, and is
    /// revoked when its claim lapses.
    #[serde(rename = "waiting")]
    Waiting,
    /// Refused as a wrong token is, and kept in the listing with when it was revoked.
    #[serde(rename = "revoked")]
    Revoked,
    /// A member's key whose account has left the household: refused, and listed so it
    /// can be revoked.
    #[serde(rename = "orphaned")]
    Orphaned,
    /// A member's key whose account the media server could not be asked about just now.
    #[serde(rename = "unconfirmed")]
    Unconfirmed,
}

/// What a mint is asked with.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeysBody {
    /// The challenge the core issued for this device key, as it was handed over.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    /// Whether to mint it as a claim: the reply carries the link a program collects it
    /// with, once, within ten minutes, and no secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<bool>,
    /// The name of the device key whose signature stands in for the password.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// What to call it.
    pub name: String,
    /// The operator's password, given again; or, from a phone, nothing, and the three
    /// fields that prove the operator with a device key in its place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// What it is for: `home-assistant`, `mcp` or `other`.
    pub purpose: String,
    /// What it admits: `read`, `act` or `member:<account>`.
    pub scope: String,
    /// ECDSA P-256 over SHA-256, in DER, in base64url, of the UTF-8 lines
    /// `lemonfiber-mint/1`, the challenge, the name, the scope, the purpose and `claim` or
    /// `key`, each ended by a line feed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// What a collection is asked with: the claim alone.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeysClaimBody {
    /// The claim the key link carried.
    pub claim: String,
}
