use std::time::{Duration, SystemTime};

use crate::contract::{Admitted, KeysClaimBody, MintedKey, SessionBody};
use crate::error::RefusedBecause;
use crate::line::{refused_pinned, Asked, Line, DEFAULT_WAIT};
use crate::transport::{Method, Transport};
use crate::{Address, Envelope, Error, Http, KeyLink, Session};

const SESSION: &str = "/api/session";

const KEY_CLAIMS: &str = "/api/keys/claim";

const REFUSED_AT_THE_DOOR: &str = "That is not the password, or none is configured.";

const CLAIM_REFUSED: &str = "That key link collects no key: it may have collected its key already, or its claim may have lapsed. Ask for the key to be minted as a claim again.";

/// The two requests a stack answers without a credential: opening a session with a
/// password, and collecting a key with its claim.
pub struct Door {
    pub(crate) line: Line,
}

impl Door {
    /// The door of the stack at `address`, reached through this crate's transport.
    ///
    /// # Errors
    ///
    /// [`Error::Unsendable`] where the transport could not be set up.
    pub fn new(address: Address) -> Result<Self, Error> {
        let transport = Http::for_address(&address)?;
        Ok(Self {
            line: Line {
                address,
                transport: Box::new(transport),
                wait: DEFAULT_WAIT,
            },
        })
    }

    /// Collects the key a key link's claim was minted for, once, over the TLS the link's
    /// fingerprint verifies.
    ///
    /// # Errors
    ///
    /// [`Error::ClaimLapsed`] where the claim had lapsed before it was sent, and as
    /// [`Door::collect`].
    pub async fn collect_link(link: &KeyLink) -> Result<Envelope<MintedKey>, Error> {
        if link.lapsed(SystemTime::now()) {
            return Err(Error::ClaimLapsed);
        }
        Self::new(link.address().clone())?
            .collect(&KeysClaimBody {
                claim: link.claim().to_owned(),
            })
            .await
    }

    /// The door of a stack on this machine, reached through `transport`.
    ///
    /// # Errors
    ///
    /// [`Error::AddressRefused`] where `address` is pinned: a stack off this machine is
    /// reached only through this crate's transport, which checks its pin.
    pub fn with_transport(address: Address, transport: impl Transport) -> Result<Self, Error> {
        refused_pinned(&address)?;
        Ok(Self {
            line: Line {
                address,
                transport: Box::new(transport),
                wait: DEFAULT_WAIT,
            },
        })
    }

    /// The door, each request waiting at most `wait` for its answer.
    #[must_use]
    pub fn waiting(mut self, wait: Duration) -> Self {
        self.line.wait = wait;
        self
    }

    /// Offers a password once, or claims an invitation, and comes away with a session.
    ///
    /// # Errors
    ///
    /// A [`RefusedBecause::PasswordRefused`] refusal where the password, or the claim, was
    /// refused, and every [`Error`] a request can meet.
    pub async fn admit(&self, sign_in: &SessionBody) -> Result<Session, Error> {
        let asked = Asked::sending(Method::Post, SESSION.to_owned(), sign_in)?;
        let (_, envelope) = self
            .line
            .envelope(
                asked,
                None,
                Some((RefusedBecause::PasswordRefused, REFUSED_AT_THE_DOOR)),
            )
            .await?;
        Session::try_from(envelope.expect::<Admitted>(&["admission"], false)?.data)
    }

    /// Collects the key a claim was minted for, with the claim alone.
    ///
    /// # Errors
    ///
    /// A [`RefusedBecause::ClaimRefused`] refusal where the claim collects nothing, and
    /// every [`Error`] a request can meet.
    pub async fn collect(&self, claim: &KeysClaimBody) -> Result<Envelope<MintedKey>, Error> {
        let asked = Asked::sending(Method::Post, KEY_CLAIMS.to_owned(), claim)?;
        let (_, envelope) = self
            .line
            .envelope(
                asked,
                None,
                Some((RefusedBecause::ClaimRefused, CLAIM_REFUSED)),
            )
            .await?;
        envelope.expect(&["minted-key"], false)
    }
}
