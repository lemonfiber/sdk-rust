//! The Rust client for lemonfiber's web API: every read and action the surface serves,
//! typed from the contract artefact the core publishes, with typed errors, jobs, keys and
//! the live stream, over a transport a caller may replace for a stack on this machine.

pub mod contract;

mod address;
mod asking;
mod capabilities;
mod client;
mod credential;
mod door;
mod envelope;
mod error;
mod files;
mod http;
mod jobs;
mod keys;
mod line;
mod query;
mod refusal;
mod setup;
mod sse;
mod stream;
mod transport;

#[cfg(test)]
mod tests;

pub use address::{Address, CertificatePin, Resolve, SystemResolver};
pub use asking::{Action, Answer, FileRead, KeyCallable, Read, RefusalCode};
pub use capabilities::CapabilitySet;
pub use client::{Acted, Client, IdempotencyKey, Logged};
pub use credential::{Credential, Session};
pub use door::Door;
pub use envelope::Envelope;
pub use error::{Error, Refusal, RefusedBecause};
pub use files::{File, Picture, PICTURE_MOST, PICTURE_TYPES};
pub use http::Http;
pub use jobs::JobStanding;
pub use keys::{base64url, signed_text, KeyLink};
pub use line::DEFAULT_WAIT;
pub use stream::{Arrival, Break, EventStream, Following, HEARTBEAT};
pub use transport::{Chunks, Failure, Method, Opened, Request, Response, Transport};
