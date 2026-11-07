use std::time::Duration;

use serde::Serialize;
use tokio::time::Instant;

use crate::envelope::{raw, Raw};
use crate::error::RefusedBecause;
use crate::refusal::refusal_of;
use crate::transport::{Carry, Failure, Method, Request, Response};
use crate::{Address, Error};

/// How long a call waits for its answer, every attempt at it included, unless the client
/// is told otherwise.
pub const DEFAULT_WAIT: Duration = Duration::from_secs(30);

pub(crate) const JSON: &str = "application/json";

const ATTEMPTS: u32 = 3;

const FIRST_PAUSE: Duration = Duration::from_millis(250);

const PASSED_ON: &[u16] = &[502, 503, 504];

const NOT_ANSWERING: &str =
    "lemonfiber is not answering at that address. It may have been stopped.";

pub(crate) struct Line {
    pub(crate) address: Address,
    pub(crate) transport: Box<dyn Carry>,
    pub(crate) wait: Duration,
}

pub(crate) struct Asked {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) accept: String,
    pub(crate) headers: Vec<(&'static str, String)>,
    pub(crate) body: Option<Vec<u8>>,
    pub(crate) most: Option<usize>,
    pub(crate) again: bool,
}

impl Asked {
    pub(crate) fn read(path: String) -> Self {
        Self {
            method: Method::Get,
            path,
            accept: JSON.to_owned(),
            headers: Vec::new(),
            body: None,
            most: None,
            again: true,
        }
    }

    pub(crate) fn once(method: Method, path: String) -> Self {
        Self {
            again: false,
            method,
            ..Self::read(path)
        }
    }

    pub(crate) fn sending(
        method: Method,
        path: String,
        body: &impl Serialize,
    ) -> Result<Self, Error> {
        Self::carrying(method, path, serde_json::to_vec(body))
    }

    pub(crate) fn carrying(
        method: Method,
        path: String,
        body: Result<Vec<u8>, serde_json::Error>,
    ) -> Result<Self, Error> {
        let Ok(body) = body else {
            return Err(Error::Unsendable(
                "That request holds what cannot be sent.".to_owned(),
            ));
        };
        Ok(Self {
            headers: vec![("Content-Type", JSON.to_owned())],
            body: Some(body),
            ..Self::once(method, path)
        })
    }
}

pub(crate) fn refused_pinned(address: &Address) -> Result<(), Error> {
    if address.pin().is_some() {
        Err(Error::AddressRefused(
            "A stack off this machine is reached only through this client's own transport, which checks its pin."
                .to_owned(),
        ))
    } else {
        Ok(())
    }
}

pub(crate) fn succeeded(answer: &Response) -> bool {
    (200..300).contains(&answer.status)
}

impl Line {
    pub(crate) fn request(&self, asked: Asked, credential: Option<&str>) -> Request {
        let mut headers = vec![("Accept", asked.accept)];
        if let Some(secret) = credential {
            headers.push((crate::credential::HEADER, secret.to_owned()));
        }
        headers.extend(asked.headers);
        Request {
            method: asked.method,
            url: self.address.url(&asked.path),
            headers,
            body: asked.body,
            most: asked.most,
        }
    }

    pub(crate) async fn exchange(
        &self,
        asked: Asked,
        credential: Option<&str>,
    ) -> Result<Response, Error> {
        let again = asked.again;
        let request = self.request(asked, credential);
        let deadline = Instant::now() + self.wait;
        let attempts = async {
            let mut made = 0;
            let mut pause = FIRST_PAUSE;
            loop {
                made += 1;
                let answered = self.transport.carry(request.clone()).await;
                let passing = match &answered {
                    Err(Failure::Unreachable(_)) => true,
                    Ok(answer) => PASSED_ON.contains(&answer.status),
                    Err(_) => false,
                };
                if !again || made >= ATTEMPTS || !passing || Instant::now() + pause >= deadline {
                    return answered;
                }
                tokio::time::sleep(pause).await;
                pause *= 2;
            }
        };
        match tokio::time::timeout_at(deadline, attempts).await {
            Err(_) => Err(Error::TimedOut(self.wait)),
            Ok(Ok(answer)) => Ok(answer),
            Ok(Err(failure)) => Err(failed(&failure)),
        }
    }

    pub(crate) async fn envelope(
        &self,
        asked: Asked,
        credential: Option<&str>,
        door: Option<(RefusedBecause, &str)>,
    ) -> Result<(u16, Raw), Error> {
        let answer = self.exchange(asked, credential).await?;
        if succeeded(&answer) {
            Ok((answer.status, raw(&answer.body)?))
        } else {
            Err(refusal_of(&answer, door))
        }
    }
}

pub(crate) fn failed(failure: &Failure) -> Error {
    match failure {
        Failure::Unreachable(_) => Error::Unreachable(NOT_ANSWERING.to_owned()),
        Failure::CertificateRefused => Error::CertificateRefused,
        Failure::TooLarge => {
            Error::Unreadable("it is larger than an answer to this request may be".to_owned())
        }
    }
}
