use std::future::Future;
use std::pin::Pin;

use futures_util::Stream;

/// What a request asks the stack to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// Read.
    Get,
    /// Write.
    Post,
    /// Remove.
    Delete,
}

impl Method {
    /// The method as HTTP spells it.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Delete => "DELETE",
        }
    }
}

/// One request, as every transport sends it. Its headers carry the credential and its
/// body may carry a password, so neither is shown when it is printed.
#[derive(Clone, PartialEq, Eq)]
pub struct Request {
    /// What it asks.
    pub method: Method,
    /// The whole address it is sent to.
    pub url: String,
    /// Every header it carries.
    pub headers: Vec<(&'static str, String)>,
    /// What it sends, where it sends anything.
    pub body: Option<Vec<u8>>,
    /// The most bytes the answer may be: a transport reads one byte past it and no further,
    /// and none of an answer that declares more. None reads the whole answer.
    pub most: Option<usize>,
}

impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request({} {})", self.method.as_str(), self.url)
    }
}

/// One answer, as every transport hands it back. Its body may carry a secret, so it is
/// not shown when the answer is printed.
#[derive(Clone, PartialEq, Eq)]
pub struct Response {
    /// Its status.
    pub status: u16,
    /// Every header it carried.
    pub headers: Vec<(String, String)>,
    /// Its body.
    pub body: Vec<u8>,
}

impl std::fmt::Debug for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Response({})", self.status)
    }
}

impl Response {
    /// The value of the header `name`, matched without regard to case.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(held, _)| held.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}

/// The body of an opened stream, chunk by chunk as it arrives.
pub type Chunks = Pin<Box<dyn Stream<Item = Result<Vec<u8>, Failure>> + Send>>;

/// A stream as it opened: its status and headers, and its body as it arrives.
pub struct Opened {
    /// Its status.
    pub status: u16,
    /// Every header it carried.
    pub headers: Vec<(String, String)>,
    /// Its body.
    pub body: Chunks,
}

impl std::fmt::Debug for Opened {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Opened({})", self.status)
    }
}

/// Why a transport handed back no answer.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Failure {
    /// Nothing answered, or the connection failed, in the transport's words.
    #[error("nothing answered: {0}")]
    Unreachable(String),
    /// The stack presented a certificate the transport does not hold it to.
    #[error("the stack presented a certificate it is not held to")]
    CertificateRefused,
    /// The answer is larger than the request allowed.
    #[error("the answer is larger than the request allowed")]
    TooLarge,
}

/// What carries the client's requests to a stack and its answers back.
pub trait Transport: Send + Sync + 'static {
    /// Sends one request and reads its whole answer.
    fn send(&self, request: Request) -> impl Future<Output = Result<Response, Failure>> + Send;

    /// Sends one request and hands back its answer as it arrives.
    fn open(&self, request: Request) -> impl Future<Output = Result<Opened, Failure>> + Send;
}

type Carried<'a, T> = Pin<Box<dyn Future<Output = Result<T, Failure>> + Send + 'a>>;

pub(crate) trait Carry: Send + Sync {
    fn carry(&self, request: Request) -> Carried<'_, Response>;

    fn stream(&self, request: Request) -> Carried<'_, Opened>;
}

impl<T: Transport> Carry for T {
    fn carry(&self, request: Request) -> Carried<'_, Response> {
        Box::pin(self.send(request))
    }

    fn stream(&self, request: Request) -> Carried<'_, Opened> {
        Box::pin(self.open(request))
    }
}
