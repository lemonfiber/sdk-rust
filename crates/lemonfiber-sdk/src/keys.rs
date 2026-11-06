use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::contract::{
    DeviceChallenge, DeviceKeyListing, DeviceKeysBody, KeyClaim, KeyListing, KeysBody, MintedKey,
    RegisteredDeviceKey,
};
use crate::line::Asked;
use crate::query::segment;
use crate::transport::Method;
use crate::{Address, CertificatePin, Client, Envelope, Error};

const KEYS: &str = "/api/keys";

const DEVICE_KEYS: &str = "/api/device-keys";

const MINT_FORM: &str = "lemonfiber-mint/1";

const KEY_LINK: &str = "lemonfiber://key";

const LINKED: &[&str] = &["address", "claim", "expires", "fingerprint", "stack"];

const BASE64URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Bytes as the stack reads them in a body: base64url, unpadded.
#[must_use]
pub fn base64url(raw: &[u8]) -> String {
    let mut written = String::with_capacity(raw.len().div_ceil(3) * 4);
    for chunk in raw.chunks(3) {
        let [first, second, third] = [0, 1, 2].map(|at| chunk.get(at).copied().unwrap_or_default());
        let joined = u32::from(first) << 16 | u32::from(second) << 8 | u32::from(third);
        for place in 0..=chunk.len() {
            let index = (joined >> (18 - 6 * place)) & 0x3f;
            written.extend(
                BASE64URL
                    .get(index as usize)
                    .map(|letter| char::from(*letter)),
            );
        }
    }
    written
}

/// The text a device key signs to prove `minting` over `challenge`, as UTF-8: the form,
/// the challenge, the name, the scope, the purpose and `claim` or `key`, each ended by a
/// line feed.
///
/// # Errors
///
/// [`Error::Unsendable`] where a part holds a line feed, which would sign another mint.
pub fn signed_text(minting: &KeysBody, challenge: &str) -> Result<Vec<u8>, Error> {
    let what = if minting.claim == Some(true) {
        "claim"
    } else {
        "key"
    };
    let lines = [
        MINT_FORM,
        challenge,
        &minting.name,
        &minting.scope,
        &minting.purpose,
        what,
    ];
    if lines.iter().any(|line| line.contains('\n')) {
        return Err(Error::Unsendable(
            "A mint's name, scope, purpose and challenge are each one line of the signed text; one holds a line feed."
                .to_owned(),
        ));
    }
    Ok(lines
        .iter()
        .flat_map(|line| format!("{line}\n").into_bytes())
        .collect())
}

impl Client {
    /// The integration keys this credential may see, without their secrets.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn keys(&self) -> Result<Envelope<KeyListing>, Error> {
        self.expect(Asked::read(KEYS.to_owned()), "keys").await
    }

    /// Mints one integration key, its secret in this answer and in no other. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn mint(&self, minting: &KeysBody) -> Result<Envelope<MintedKey>, Error> {
        let minting = KeysBody {
            claim: None,
            ..minting.clone()
        };
        self.expect(
            Asked::sending(Method::Post, KEYS.to_owned(), &minting)?,
            "minted-key",
        )
        .await
    }

    /// Mints one integration key as a claim: answered with the key link that collects it,
    /// once, and no secret. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn mint_claim(&self, minting: &KeysBody) -> Result<Envelope<KeyClaim>, Error> {
        let minting = KeysBody {
            claim: Some(true),
            ..minting.clone()
        };
        self.expect(
            Asked::sending(Method::Post, KEYS.to_owned(), &minting)?,
            "key-claim",
        )
        .await
    }

    /// Revokes one integration key by its name, answered with the keys as they now stand.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn revoke(&self, name: &str) -> Result<Envelope<KeyListing>, Error> {
        self.expect(
            Asked::once(Method::Delete, format!("{KEYS}/{}", segment(name)?)),
            "keys",
        )
        .await
    }

    /// The operator's device keys, revoked ones included.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn device_keys(&self) -> Result<Envelope<DeviceKeyListing>, Error> {
        self.expect(Asked::read(DEVICE_KEYS.to_owned()), "device-keys")
            .await
    }

    /// Registers a phone's key pair as a device key. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn register_device_key(
        &self,
        registering: &DeviceKeysBody,
    ) -> Result<Envelope<RegisteredDeviceKey>, Error> {
        self.expect(
            Asked::sending(Method::Post, DEVICE_KEYS.to_owned(), registering)?,
            "device-key",
        )
        .await
    }

    /// A challenge for one device key, good for one mint within a minute. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn device_key_challenge(
        &self,
        name: &str,
    ) -> Result<Envelope<DeviceChallenge>, Error> {
        let path = format!("{DEVICE_KEYS}/{}/challenge", segment(name)?);
        self.expect(Asked::once(Method::Post, path), "device-challenge")
            .await
    }

    /// Revokes one device key by its name; the keys it proved stay. Sent once.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn revoke_device_key(&self, name: &str) -> Result<Envelope<DeviceKeyListing>, Error> {
        let path = format!("{DEVICE_KEYS}/{}", segment(name)?);
        self.expect(Asked::once(Method::Delete, path), "device-keys")
            .await
    }
}

/// A key link, as a key minted as a claim hands one over: where the stack is, its pin, and
/// the claim that collects the key once. The claim is not shown when it is printed.
#[derive(Clone, PartialEq, Eq)]
pub struct KeyLink {
    address: Address,
    stack: String,
    lapses: SystemTime,
    claim: String,
}

impl std::fmt::Debug for KeyLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyLink({}, stack={})", self.address.base(), self.stack)
    }
}

impl KeyLink {
    /// A key link, refusing one that is not exactly what a claim's mint writes.
    ///
    /// # Errors
    ///
    /// [`Error::Unsendable`] where the link is not one, and [`Error::AddressRefused`] where
    /// its address or fingerprint is not one a key is collected at.
    pub fn new(link: &str) -> Result<Self, Error> {
        let refused = || {
            Error::Unsendable(format!(
                "A key link begins with {KEY_LINK} and carries {}, each once, and nothing else; this one does not.",
                LINKED.join(", ")
            ))
        };
        let query = link
            .trim()
            .strip_prefix(KEY_LINK)
            .and_then(|rest| rest.strip_prefix('?'))
            .ok_or_else(refused)?;
        let mut pairs: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();
        pairs.sort();
        let names: Vec<&str> = pairs.iter().map(|(name, _)| name.as_str()).collect();
        let [(_, address), (_, claim), (_, expires), (_, fingerprint), (_, stack)] =
            pairs.as_slice()
        else {
            return Err(refused());
        };
        if names != LINKED || claim.is_empty() || stack.is_empty() {
            return Err(refused());
        }
        let lapses = expires
            .parse::<u64>()
            .ok()
            .and_then(|seconds| UNIX_EPOCH.checked_add(Duration::from_secs(seconds)))
            .ok_or_else(refused)?;
        Ok(Self {
            address: Address::new(address, Some(CertificatePin::new(fingerprint)?))?,
            stack: stack.clone(),
            lapses,
            claim: claim.clone(),
        })
    }

    /// Where the key is collected, held to the link's fingerprint.
    #[must_use]
    pub fn address(&self) -> &Address {
        &self.address
    }

    /// The identifier of the stack the key is for.
    #[must_use]
    pub fn stack(&self) -> &str {
        &self.stack
    }

    /// When the claim stops collecting the key.
    #[must_use]
    pub fn lapses(&self) -> SystemTime {
        self.lapses
    }

    /// Whether the claim had lapsed by `now`.
    #[must_use]
    pub fn lapsed(&self, now: SystemTime) -> bool {
        now >= self.lapses
    }

    pub(crate) fn claim(&self) -> &str {
        &self.claim
    }
}
