use std::fmt;
use std::time::Duration;

use crate::contract::Problem;
use crate::RefusalCode;

/// Something stood between the caller and an answer from lemonfiber. Every sentence is
/// one a person can be shown.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    /// The client was given something it cannot be built with or send, and nothing was sent.
    #[error("{0}")]
    Unsendable(String),
    /// An address, or the pin given with it, is not one this client will reach, and nothing
    /// was sent.
    #[error("{0}")]
    AddressRefused(String),
    /// Nothing answered at the address, or what answered was not lemonfiber.
    #[error("{0}")]
    Unreachable(String),
    /// The stack presented a certificate other than the one it is pinned to, and nothing was
    /// sent.
    #[error("The stack's certificate is not the one pinned for it, or no trust store vouches for it, so nothing was sent.")]
    CertificateRefused,
    /// No answer arrived within the wait the call was given.
    #[error("lemonfiber did not answer within {} seconds.", .0.as_secs_f64())]
    TimedOut(Duration),
    /// An answer arrived that is not a document lemonfiber writes.
    #[error("That answer did not come from lemonfiber: {0}.")]
    Unreadable(String),
    /// The stack answered in a version of the interface this client does not speak.
    #[error("This client speaks version {spoken} of lemonfiber's interface and the stack answered in version {served}. Nothing from that answer was used; update whichever of the two is older.")]
    VersionMismatch {
        /// The version this client speaks.
        spoken: u32,
        /// The version the stack answered in.
        served: u64,
    },
    /// The stack answered with a kind of document this client was not generated with.
    #[error("The stack answered with a {0:?} document, which this client does not know. The stack is newer than this client; update the client.")]
    UnknownKind(String),
    /// An answer was a kind of document other than the ones the request answers with.
    #[error("The stack answered with a {received:?} document where {} was expected.", expected.join(" or "))]
    UnexpectedKind {
        /// The kinds the request answers with.
        expected: Vec<String>,
        /// The kind that arrived.
        received: String,
    },
    /// lemonfiber answered, and turned the request down.
    #[error("{0}")]
    Refused(Box<Refusal>),
    /// The name is not one this run of lemonfiber issued.
    #[error("lemonfiber has no work by the name {0:?} in this run.")]
    NoSuchJob(String),
    /// Work was followed for as long as the caller allowed, and it is still going.
    #[error("The work {job:?} was still going after {} seconds.", waited.as_secs_f64())]
    StillRunning {
        /// The work's name.
        job: String,
        /// How long it was followed for.
        waited: Duration,
    },
    /// The live stream broke and could not be reopened.
    #[error("The live stream broke and {0} attempts to reopen it failed. Everything shown is the last thing confirmed, not what is true now.")]
    StreamLost(u32),
    /// A key link's claim had lapsed before it could be sent, and nothing was sent.
    #[error("That key link's claim has lapsed, so nothing was sent. Ask for the key to be minted as a claim again.")]
    ClaimLapsed,
}

/// Why lemonfiber turned a request down, read from its code where the contract lists one
/// and from its status where it does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusedBecause {
    /// The credential is not one this stack admits: absent, wrong, expired, revoked or from
    /// another run. Signing in again, or another key, may help.
    NotAdmitted,
    /// The credential was read, and what is objected to is who is asking or where from.
    Declined,
    /// What the request named, this product does not have.
    Missing,
    /// The request could not be answered as it was asked.
    Misasked,
    /// Other work held the stack; the same request may be sent again once it is done.
    Busy,
    /// Nothing about the request was wrong; the machine could not answer it.
    Failed,
    /// Too many wrong answers lately.
    TooManyAttempts,
    /// The password offered at the door was wrong, or none is configured.
    PasswordRefused,
    /// The claim collects no key: none waits on it, it was collected, or it lapsed.
    ClaimRefused,
}

/// One refusal: why, the sentence lemonfiber said, the status, and the code and problem
/// where it carried them.
#[derive(Debug, Clone, PartialEq)]
pub struct Refusal {
    /// Why, as a caller acts on it.
    pub because: RefusedBecause,
    /// The sentence, for a person; never decided on.
    pub sentence: String,
    /// The status it was answered with.
    pub status: u16,
    /// The code, where it is one the contract lists.
    pub code: Option<&'static RefusalCode>,
    /// The problem the refusal carried, where it carried one. Its `detail` quotes what a
    /// service said, fit to show the person who asked and not to forward.
    pub problem: Option<Problem>,
    /// How many seconds are left before asking again may succeed, where the stack said.
    pub retry_after: Option<u64>,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.sentence)
    }
}

impl From<Refusal> for Error {
    fn from(refusal: Refusal) -> Self {
        Self::Refused(Box::new(refusal))
    }
}
