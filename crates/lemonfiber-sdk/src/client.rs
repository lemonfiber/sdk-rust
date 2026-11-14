use std::time::{Duration, SystemTime};

use crate::asking::ACTIONS;
use crate::contract::{reads, Capabilities, LogLine, Payload, Started};
use crate::envelope::raw;
use crate::files::{File, Picture, PICTURE_MOST, PICTURE_TYPES};
use crate::jobs::{standing_of, JobStanding};
use crate::line::{refused_pinned, succeeded, Asked, Line, DEFAULT_WAIT};
use crate::query::{located, segment};
use crate::refusal::refusal_of;
use crate::transport::{Method, Transport};
use crate::{
    Action, Address, Answer, CapabilitySet, Credential, Envelope, Error, FileRead, Http, Read,
};

const IDEMPOTENCY_HEADER: &str = "Idempotency-Key";

const CAPABILITIES: &str = "/api/capabilities";

const JOBS: &str = "/api/jobs/";

/// The name of one attempt at an action, so an action sent again after no answer arrived
/// is carried out once: one to 255 visible ASCII characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// The key `key`.
    ///
    /// # Errors
    ///
    /// [`Error::Unsendable`] where it is not one to 255 visible ASCII characters.
    pub fn new(key: impl Into<String>) -> Result<Self, Error> {
        let key = key.into();
        if (1..=255).contains(&key.len()) && key.bytes().all(|byte| byte.is_ascii_graphic()) {
            Ok(Self(key))
        } else {
            Err(Error::Unsendable(
                "An idempotency key is one to 255 visible ASCII characters; what was given is not one.".to_owned(),
            ))
        }
    }
}

/// What an action was answered with.
#[derive(Debug, Clone, PartialEq)]
pub enum Acted {
    /// The outcome, because the work was done by the time the answer was written.
    Answered(Envelope<Payload>),
    /// A name for the work, which goes on after the answer: follow it with
    /// [`Client::follow`].
    Started(Envelope<Started>),
}

/// What the logs read was answered with.
#[derive(Debug, Clone, PartialEq)]
pub enum Logged {
    /// The lines asked for, one envelope each.
    Lines(Vec<Envelope<LogLine>>),
    /// A name for the work following the logs as they grow; the lines arrive on the stream.
    Following(Envelope<Started>),
}

/// Talks to one stack with one credential.
///
/// Every answer that is a document is read through its envelope, so a version this client
/// cannot speak is refused rather than half understood. A read is asked again after a
/// passing failure; anything that changes something is asked once.
pub struct Client {
    pub(crate) line: Line,
    pub(crate) credential: Credential,
}

impl Client {
    /// A client reaching `address` through this crate's transport.
    ///
    /// # Errors
    ///
    /// [`Error::Unsendable`] where the transport could not be set up.
    pub fn new(address: Address, credential: Credential) -> Result<Self, Error> {
        let transport = Http::for_address(&address)?;
        Ok(Self {
            line: Line {
                address,
                transport: Box::new(transport),
                wait: DEFAULT_WAIT,
            },
            credential,
        })
    }

    /// A client reaching a stack on this machine through `transport`.
    ///
    /// # Errors
    ///
    /// [`Error::AddressRefused`] where `address` is pinned: a stack off this machine is
    /// reached only through this crate's transport, which checks its pin.
    pub fn with_transport(
        address: Address,
        credential: Credential,
        transport: impl Transport,
    ) -> Result<Self, Error> {
        refused_pinned(&address)?;
        Ok(Self {
            line: Line {
                address,
                transport: Box::new(transport),
                wait: DEFAULT_WAIT,
            },
            credential,
        })
    }

    /// The client, each call waiting at most `wait` for its answer.
    #[must_use]
    pub fn waiting(mut self, wait: Duration) -> Self {
        self.line.wait = wait;
        self
    }

    /// Where the client sends, and the pin it holds that address to.
    #[must_use]
    pub fn address(&self) -> &Address {
        &self.line.address
    }

    pub(crate) async fn ask(&self, asked: Asked) -> Result<(u16, crate::envelope::Raw), Error> {
        self.line
            .envelope(asked, Some(self.credential.secret()), None)
            .await
    }

    pub(crate) async fn expect<A: serde::de::DeserializeOwned>(
        &self,
        asked: Asked,
        kind: &str,
    ) -> Result<Envelope<A>, Error> {
        self.ask(asked).await?.1.expect(&[kind], false)
    }

    /// What the command a read mirrors prints under `--json`.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet; [`Error::UnexpectedKind`] where the answer is a
    /// kind the read does not answer with.
    pub async fn read<R: Read>(&self, read: &R) -> Result<Envelope<R::Answer>, Error> {
        let path = located(R::PATH, R::SEGMENTS, read)?;
        self.ask(Asked::read(path))
            .await?
            .1
            .expect(R::KINDS, <R::Answer as Answer>::TAGGED)
    }

    /// A file a read hands over, kept as the bytes that arrived.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn file<R: FileRead>(&self, read: &R) -> Result<File, Error> {
        let path = located(R::PATH, R::SEGMENTS, read)?;
        let answer = self
            .line
            .exchange(
                Asked {
                    accept: "*/*".to_owned(),
                    ..Asked::read(path)
                },
                Some(self.credential.secret()),
            )
            .await?;
        if !succeeded(&answer) {
            return Err(refusal_of(&answer, None));
        }
        let content_type = answer.header("content-type").map(str::to_owned);
        Ok(File {
            content: answer.body,
            content_type,
        })
    }

    /// One of a title's pictures: a raster image of at most [`PICTURE_MOST`] bytes.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet; [`Error::Unreadable`] where what arrived is
    /// larger, or is not a raster image.
    pub async fn picture<R: FileRead>(&self, read: &R) -> Result<Picture, Error> {
        let path = located(R::PATH, R::SEGMENTS, read)?;
        let answer = self
            .line
            .exchange(
                Asked {
                    accept: PICTURE_TYPES.join(", "),
                    most: Some(PICTURE_MOST),
                    ..Asked::read(path)
                },
                Some(self.credential.secret()),
            )
            .await?;
        if !succeeded(&answer) {
            return Err(refusal_of(&answer, None));
        }
        let labelled = answer.header("content-type").unwrap_or_default();
        let media_type = labelled
            .split(';')
            .next()
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase();
        if !PICTURE_TYPES.contains(&media_type.as_str()) {
            return Err(Error::Unreadable(format!(
                "the picture is labelled {labelled:?}, and a picture is one of {}",
                PICTURE_TYPES.join(", ")
            )));
        }
        Ok(Picture {
            content: answer.body,
            media_type,
        })
    }

    /// What the services have been saying, a `log` envelope a line, or the work following
    /// them as they grow where `follow` was asked for.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn logs(&self, read: &reads::LogsRead) -> Result<Logged, Error> {
        let path = located(<reads::LogsRead as Read>::PATH, &[], read)?;
        let answer = self
            .line
            .exchange(Asked::read(path), Some(self.credential.secret()))
            .await?;
        if !succeeded(&answer) {
            return Err(refusal_of(&answer, None));
        }
        let mut lines = Vec::new();
        for line in answer
            .body
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.trim_ascii().is_empty())
        {
            let envelope = raw(line)?;
            if envelope.kind() == "job" {
                return Ok(Logged::Following(envelope.expect(&["job"], false)?));
            }
            lines.push(envelope.expect(&["log"], false)?);
        }
        Ok(Logged::Lines(lines))
    }

    /// Tells lemonfiber to do something the command line could also do. Sent once, never
    /// again.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn act<A: Action>(&self, action: &A) -> Result<Acted, Error> {
        self.acting(action, None).await
    }

    /// Tells lemonfiber to do something, naming the attempt with `key`, so sending it again
    /// after no answer arrived is answered with what the first send was and is not carried
    /// out twice.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn act_once<A: Action>(
        &self,
        action: &A,
        key: &IdempotencyKey,
    ) -> Result<Acted, Error> {
        self.acting(action, Some(key)).await
    }

    async fn acting<A: Action>(
        &self,
        action: &A,
        key: Option<&IdempotencyKey>,
    ) -> Result<Acted, Error> {
        let path = format!("{ACTIONS}{}", segment(A::NAME)?);
        let mut asked = Asked::sending(Method::Post, path, action)?;
        if let Some(key) = key {
            asked.headers.push((IDEMPOTENCY_HEADER, key.0.clone()));
        }
        let (status, envelope) = self.ask(asked).await?;
        if status == 202 {
            Ok(Acted::Started(envelope.expect(&["job"], false)?))
        } else {
            Ok(Acted::Answered(envelope.payload()?))
        }
    }

    /// What the stack can do, for the credential this client holds, as it stands now.
    ///
    /// # Errors
    ///
    /// Every [`Error`] a request can meet.
    pub async fn capabilities(&self) -> Result<CapabilitySet, Error> {
        let answered: Envelope<Capabilities> = self
            .expect(Asked::read(CAPABILITIES.to_owned()), "capabilities")
            .await?;
        Ok(CapabilitySet {
            states: answered.data.capabilities,
            read_at: SystemTime::now(),
            scope: answered.data.scope,
            stack: answered.data.stack,
        })
    }

    /// Where the work `job` names got to.
    ///
    /// # Errors
    ///
    /// [`Error::NoSuchJob`] where this run issued no such name, and every [`Error`] a
    /// request can meet.
    pub async fn job(&self, job: &str) -> Result<JobStanding, Error> {
        self.standing(Method::Get, job).await
    }

    /// Lets the name `job` go, ending the work it stands for, and says where it now stands.
    ///
    /// # Errors
    ///
    /// As [`Client::job`].
    pub async fn release(&self, job: &str) -> Result<JobStanding, Error> {
        self.standing(Method::Delete, job).await
    }

    async fn standing(&self, method: Method, job: &str) -> Result<JobStanding, Error> {
        let asked = Asked {
            again: method == Method::Get,
            ..Asked::once(method, format!("{JOBS}{}", segment(job)?))
        };
        let answer = self
            .line
            .exchange(asked, Some(self.credential.secret()))
            .await?;
        standing_of(job, &answer)
    }

    /// Asks where the work `job` names stands every `every` until it is no longer going.
    ///
    /// # Errors
    ///
    /// [`Error::StillRunning`] once `within` has passed with it still going, and every
    /// error [`Client::job`] can meet.
    pub async fn follow(
        &self,
        job: &str,
        every: Duration,
        within: Option<Duration>,
    ) -> Result<JobStanding, Error> {
        let started = tokio::time::Instant::now();
        loop {
            let standing = self.job(job).await?;
            if !matches!(standing, JobStanding::Running(_)) {
                return Ok(standing);
            }
            let waited = started.elapsed();
            let pause = match within {
                Some(within) if waited >= within => {
                    return Err(Error::StillRunning {
                        job: job.to_owned(),
                        waited,
                    })
                }
                Some(within) => every.min(within.saturating_sub(waited)),
                None => every,
            };
            tokio::time::sleep(pause).await;
        }
    }
}
