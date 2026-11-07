use std::collections::{BTreeMap, VecDeque};
use std::time::Duration;

use futures_util::StreamExt;
use tokio::time::Instant;

use crate::contract::Payload;
use crate::envelope::raw;
use crate::line::{failed, Asked};
use crate::refusal::refusal_of;
use crate::sse::{Parser, LARGEST};
use crate::transport::{Chunks, Failure, Response};
use crate::{Client, Envelope, Error};

/// How often the server speaks when it has nothing to say.
pub const HEARTBEAT: Duration = Duration::from_secs(15);

const EVENTS: &str = "/api/events";

const EVENT_STREAM: &str = "text/event-stream";

const RESUME_HEADER: &str = "Last-Event-ID";

const LONGEST_PAUSE: Duration = Duration::from_secs(30);

const REFUSED_OUTRIGHT: &[u16] = &[400, 401, 403, 404];

pub(crate) const MOST_SAID: usize = 64 * 1024;

/// How a stream is followed: how long a silence may last, and how it is reopened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Following {
    /// The silence after which the stream is broken rather than quiet: twice the heartbeat,
    /// so one missed beat is tolerated.
    pub silence: Duration,
    /// How many openings in a row may fail before following gives up.
    pub reconnects: u32,
    /// The pause before the first attempt to reopen; each failed attempt doubles it, to at
    /// most thirty seconds.
    pub first_pause: Duration,
}

impl Default for Following {
    fn default() -> Self {
        Self {
            silence: HEARTBEAT * 2,
            reconnects: 5,
            first_pause: Duration::from_secs(1),
        }
    }
}

/// How a stream stopped carrying anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Break {
    /// Nothing arrived, not even a heartbeat, for longer than the silence allowed.
    Silent,
    /// The server closed the stream.
    Ended,
    /// The connection failed.
    Dropped,
}

/// What following the stream hands over, in the order it happened.
#[derive(Debug, Clone, PartialEq)]
pub enum Arrival {
    /// A value the stream carried just now.
    Live(Envelope<Payload>),
    /// A value held from before a gap: the last thing the stream said, not what is true now.
    Stale {
        /// The value.
        envelope: Envelope<Payload>,
        /// How long ago the stream carried it.
        quiet_for: Duration,
    },
    /// The stream broke; every value held is stale until it is carried again.
    Gap {
        /// How it broke.
        why: Break,
        /// How long since anything at all arrived.
        quiet_for: Duration,
    },
    /// The stream carried a kind this client was not generated with.
    Unrecognised(String),
}

struct Held {
    envelope: Envelope<Payload>,
    at: Instant,
    live: bool,
}

/// The live stream, followed: what arrives, what has gone stale across a gap, and each gap.
pub struct EventStream<'a> {
    client: &'a Client,
    following: Following,
    held: BTreeMap<String, Held>,
    pending: VecDeque<Arrival>,
    parser: Parser,
    body: Option<Chunks>,
    last_id: Option<String>,
    last_heard: Instant,
    failures: u32,
    reopening: bool,
}

impl Client {
    /// Follows the live stream, as `following` says.
    #[must_use]
    pub fn events(&self, following: Following) -> EventStream<'_> {
        EventStream {
            client: self,
            following,
            held: BTreeMap::new(),
            pending: VecDeque::new(),
            parser: Parser::new(LARGEST),
            body: None,
            last_id: None,
            last_heard: Instant::now(),
            failures: 0,
            reopening: false,
        }
    }
}

impl EventStream<'_> {
    /// The next arrival, waiting for it.
    ///
    /// # Errors
    ///
    /// [`Error::StreamLost`] once more openings in a row have failed than are allowed, a
    /// refusal the stack answered the opening with, [`Error::CertificateRefused`], and an
    /// event this client cannot read.
    pub async fn next(&mut self) -> Result<Arrival, Error> {
        loop {
            if let Some(arrival) = self.pending.pop_front() {
                return Ok(arrival);
            }
            match self.body.take() {
                None => self.open().await?,
                Some(mut body) => {
                    match tokio::time::timeout(self.following.silence, body.next()).await {
                        Ok(Some(Ok(chunk))) => {
                            self.heard(&chunk)?;
                            self.body = Some(body);
                        }
                        Ok(Some(Err(_))) => self.broke(Break::Dropped),
                        Ok(None) => self.broke(Break::Ended),
                        Err(_) => self.broke(Break::Silent),
                    }
                }
            }
        }
    }

    /// The last value of each kind the stream carried, and whether it is still current.
    #[must_use]
    pub fn held(&self) -> BTreeMap<&str, Arrival> {
        let now = Instant::now();
        self.held
            .iter()
            .map(|(kind, held)| {
                let arrival = if held.live {
                    Arrival::Live(held.envelope.clone())
                } else {
                    Arrival::Stale {
                        envelope: held.envelope.clone(),
                        quiet_for: now - held.at,
                    }
                };
                (kind.as_str(), arrival)
            })
            .collect()
    }

    async fn open(&mut self) -> Result<(), Error> {
        if self.reopening {
            self.failures += 1;
            if self.failures > self.following.reconnects {
                return Err(Error::StreamLost(self.following.reconnects));
            }
            let doubled = self.following.first_pause * 2u32.saturating_pow(self.failures - 1);
            tokio::time::sleep(doubled.min(LONGEST_PAUSE)).await;
        }
        self.reopening = true;
        let mut asked = Asked::read(EVENTS.to_owned());
        EVENT_STREAM.clone_into(&mut asked.accept);
        if let Some(id) = &self.last_id {
            asked.headers.push((RESUME_HEADER, id.clone()));
        }
        let line = &self.client.line;
        let request = line.request(asked, Some(self.client.credential.secret()));
        match tokio::time::timeout(line.wait, line.transport.stream(request)).await {
            Ok(Ok(opened)) if opened.status == 200 => {
                self.body = Some(opened.body);
                self.parser = Parser::new(LARGEST);
                self.last_heard = Instant::now();
                Ok(())
            }
            Ok(Ok(mut opened)) if REFUSED_OUTRIGHT.contains(&opened.status) => {
                let mut said = Vec::new();
                while let Some(Ok(chunk)) = opened.body.next().await {
                    said.extend_from_slice(&chunk);
                    if said.len() > MOST_SAID {
                        break;
                    }
                }
                Err(refusal_of(
                    &Response {
                        status: opened.status,
                        headers: opened.headers,
                        body: said,
                    },
                    None,
                ))
            }
            Ok(Err(Failure::CertificateRefused)) => Err(failed(&Failure::CertificateRefused)),
            _ => Ok(()),
        }
    }

    fn heard(&mut self, chunk: &[u8]) -> Result<(), Error> {
        let now = Instant::now();
        self.last_heard = now;
        self.failures = 0;
        self.reopening = false;
        for event in self.parser.push(chunk)? {
            if event.id.is_some() {
                self.last_id = event.id;
            }
            let envelope = match raw(event.data.as_bytes())?.payload() {
                Err(Error::UnknownKind(kind)) => {
                    self.pending.push_back(Arrival::Unrecognised(kind));
                    continue;
                }
                read => read?,
            };
            self.held.insert(
                envelope.kind.clone(),
                Held {
                    envelope: envelope.clone(),
                    at: now,
                    live: true,
                },
            );
            self.pending.push_back(Arrival::Live(envelope));
        }
        Ok(())
    }

    fn broke(&mut self, why: Break) {
        let now = Instant::now();
        self.reopening = true;
        self.pending.push_back(Arrival::Gap {
            why,
            quiet_for: now - self.last_heard,
        });
        for held in self.held.values_mut() {
            held.live = false;
            self.pending.push_back(Arrival::Stale {
                envelope: held.envelope.clone(),
                quiet_for: now - held.at,
            });
        }
    }
}
