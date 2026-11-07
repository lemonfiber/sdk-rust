use std::fmt;

use crate::contract::Admitted;
use crate::Error;

pub(crate) const HEADER: &str = "X-Lemonfiber-Token";

/// A secret the stack admits, carried in `X-Lemonfiber-Token` and shown nowhere else: the
/// per-run token lemonfiber prints, a session's secret, or an integration key.
#[derive(Clone, PartialEq, Eq)]
pub struct Credential(String);

impl Credential {
    /// A credential holding `secret`.
    ///
    /// # Errors
    ///
    /// [`Error::Unsendable`] where it is not visible ASCII with no spaces, which is all a
    /// header can carry unchanged.
    pub fn new(secret: impl Into<String>) -> Result<Self, Error> {
        let secret = secret.into();
        if !secret.is_empty() && secret.bytes().all(|byte| byte.is_ascii_graphic()) {
            Ok(Self(secret))
        } else {
            Err(Error::Unsendable(
                "A credential is written in visible ASCII with no spaces; what was given is not one."
                    .to_owned(),
            ))
        }
    }

    pub(crate) fn secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credential(hidden)")
    }
}

/// A session opened at the door: the credential it is carried by, when it stops being one,
/// and the household member it is for, none for the operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    /// What every request in the session carries.
    pub credential: Credential,
    /// When it stops being admitted, as the stack wrote it.
    pub until: String,
    /// The household member it is for, none for the operator.
    pub member: Option<String>,
}

impl TryFrom<Admitted> for Session {
    type Error = Error;

    fn try_from(admitted: Admitted) -> Result<Self, Error> {
        Ok(Self {
            credential: Credential::new(admitted.token)?,
            until: admitted.until,
            member: admitted.member,
        })
    }
}
