use std::fmt;
use std::fmt::Write as _;
use std::net::{IpAddr, ToSocketAddrs};

use url::Url;

use crate::Error;

const PIN_DIGITS: usize = 64;

/// The SHA-256 of the one certificate a stack is permitted to present, over its DER, as
/// pairing material writes it: 64 lower-case hexadecimal characters.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CertificatePin(String);

impl CertificatePin {
    /// A pin from pairing material.
    ///
    /// # Errors
    ///
    /// [`Error::AddressRefused`] where it is not a SHA-256 digest in hexadecimal.
    pub fn new(digest: &str) -> Result<Self, Error> {
        let written = digest.trim().to_ascii_lowercase();
        if written.len() == PIN_DIGITS && written.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(written))
        } else {
            Err(Error::AddressRefused(
                "A certificate pin is the SHA-256 digest of the stack's certificate, written as 64 hexadecimal characters; what was given is not one."
                    .to_owned(),
            ))
        }
    }

    /// The digest, as 64 lower-case hexadecimal characters.
    #[must_use]
    pub fn hex(&self) -> &str {
        &self.0
    }

    pub(crate) fn of(der: &[u8]) -> Self {
        let digest = ring::digest::digest(&ring::digest::SHA256, der);
        Self(
            digest
                .as_ref()
                .iter()
                .fold(String::with_capacity(PIN_DIGITS), |mut hex, byte| {
                    let _ = write!(hex, "{byte:02x}");
                    hex
                }),
        )
    }
}

impl fmt::Debug for CertificatePin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CertificatePin({})", self.0)
    }
}

/// Turns a host name into every address it resolves to.
pub trait Resolve {
    /// Every address `host` resolves to, none where it resolves to nothing.
    fn resolve(&self, host: &str) -> Vec<IpAddr>;
}

/// The system's resolver.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemResolver;

impl Resolve for SystemResolver {
    fn resolve(&self, host: &str) -> Vec<IpAddr> {
        (host, 0)
            .to_socket_addrs()
            .map(|found| found.map(|address| address.ip()).collect())
            .unwrap_or_default()
    }
}

/// A stack's base address, and the pin it is held to where it is not on this machine.
///
/// A loopback address needs no pin, and any other needs one: an address and its pin are
/// given together, so there is no way to name a stack off this machine without one. A
/// host name given without a pin is resolved when the address is given, accepted only
/// where every address it resolves to is loopback, and reached at those addresses.
#[derive(Clone, PartialEq, Eq)]
pub struct Address {
    base: String,
    host: String,
    port: u16,
    pin: Option<CertificatePin>,
    reached: Vec<IpAddr>,
}

impl Address {
    /// An address as lemonfiber printed it, resolved by the system's resolver where it names
    /// a host and carries no pin.
    ///
    /// # Errors
    ///
    /// [`Error::AddressRefused`] where it is not an address lemonfiber prints, or names a
    /// stack off this machine without its pin.
    pub fn new(url: &str, pin: Option<CertificatePin>) -> Result<Self, Error> {
        Self::resolving(url, pin, &SystemResolver)
    }

    /// An address, resolved by `resolver` where it names a host and carries no pin.
    ///
    /// # Errors
    ///
    /// As [`Address::new`].
    pub fn resolving(
        url: &str,
        pin: Option<CertificatePin>,
        resolver: &dyn Resolve,
    ) -> Result<Self, Error> {
        let refused = |why: String| Error::AddressRefused(why);
        let parsed =
            Url::parse(url.trim()).map_err(|_| refused(format!("{url:?} is not an address.")))?;
        let scheme = parsed.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(refused(format!(
                "lemonfiber is reached over http or https, and {url:?} names neither."
            )));
        }
        if !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(refused(
                "That address carries more than an address: a credential, a query or a fragment."
                    .to_owned(),
            ));
        }
        if pin.is_some() && scheme != "https" {
            return Err(refused(
                "A pinned address is reached over https: a pin is checked against the certificate TLS presents."
                    .to_owned(),
            ));
        }
        let named = parsed.host_str().unwrap_or_default();
        let literal = named
            .trim_start_matches('[')
            .trim_end_matches(']')
            .parse::<IpAddr>()
            .ok();
        let host = named.to_owned();
        let port = parsed.port_or_known_default().unwrap_or_default();
        let reached = match (&pin, literal) {
            (Some(_), _) => Vec::new(),
            (None, Some(address)) => vec![address],
            (None, None) => {
                let mut found = Vec::new();
                for address in resolver.resolve(&host) {
                    if !found.contains(&address) {
                        found.push(address);
                    }
                }
                found
            }
        };
        if pin.is_none() && (reached.is_empty() || !reached.iter().all(is_loopback)) {
            return Err(refused(format!(
                "{host} is not on this machine, and a stack anywhere else is reached only with its certificate pin, given with the address."
            )));
        }
        let prefix = parsed.path().trim_end_matches('/');
        let base = format!("{scheme}://{host}:{port}{prefix}");
        Ok(Self {
            base,
            host: host
                .trim_start_matches('[')
                .trim_end_matches(']')
                .to_owned(),
            port,
            pin,
            reached: if literal.is_some() {
                Vec::new()
            } else {
                reached
            },
        })
    }

    /// The address every path is joined onto, its port written out.
    #[must_use]
    pub fn base(&self) -> &str {
        &self.base
    }

    /// The certificate this address is held to, where it is held to one.
    #[must_use]
    pub fn pin(&self) -> Option<&CertificatePin> {
        self.pin.as_ref()
    }

    /// The host as the address names it, without brackets.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// The port.
    #[must_use]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Where a host name given without a pin was found to resolve, every one loopback:
    /// where connections to it go. Empty for a literal address or a pinned one.
    #[must_use]
    pub fn reached(&self) -> &[IpAddr] {
        &self.reached
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base)
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({}, pin={:?})", self.base, self.pin)
    }
}

fn is_loopback(address: &IpAddr) -> bool {
    match address {
        IpAddr::V4(v4) => v4.is_loopback(),
        IpAddr::V6(v6) => {
            v6.is_loopback() || v6.to_ipv4_mapped().is_some_and(|v4| v4.is_loopback())
        }
    }
}
