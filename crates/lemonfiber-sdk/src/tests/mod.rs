mod client;
mod door;
mod http;
mod keys;
mod stream;
mod wire;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use futures_util::stream as flowing;
use serde_json::{json, Value};

use crate::transport::{Failure, Opened, Request, Response, Transport};
use crate::{Address, Client, Credential};

type Outcome = Result<(), Box<dyn std::error::Error>>;

pub(crate) struct Opening {
    status: u16,
    chunks: Vec<Result<Vec<u8>, Failure>>,
    hangs: bool,
}

#[derive(Default)]
pub(crate) struct Script {
    answers: Mutex<VecDeque<Result<Response, Failure>>>,
    openings: Mutex<VecDeque<Result<Opening, Failure>>>,
    sent: Mutex<Vec<Request>>,
}

#[derive(Clone, Default)]
pub(crate) struct Scripted(Arc<Script>);

impl Scripted {
    fn answering(answers: Vec<Result<Response, Failure>>) -> Self {
        Self(Arc::new(Script {
            answers: Mutex::new(answers.into()),
            ..Script::default()
        }))
    }

    fn opening(openings: Vec<Result<Opening, Failure>>) -> Self {
        Self(Arc::new(Script {
            openings: Mutex::new(openings.into()),
            ..Script::default()
        }))
    }

    fn sent(&self) -> Vec<Request> {
        self.0
            .sent
            .lock()
            .map(|sent| sent.clone())
            .unwrap_or_default()
    }

    fn record(&self, request: Request) {
        if let Ok(mut sent) = self.0.sent.lock() {
            sent.push(request);
        }
    }
}

impl Transport for Scripted {
    async fn send(&self, request: Request) -> Result<Response, Failure> {
        self.record(request);
        self.0
            .answers
            .lock()
            .ok()
            .and_then(|mut answers| answers.pop_front())
            .unwrap_or_else(|| Err(Failure::Unreachable("nothing scripted".to_owned())))
    }

    async fn open(&self, request: Request) -> Result<Opened, Failure> {
        self.record(request);
        let opening = self
            .0
            .openings
            .lock()
            .ok()
            .and_then(|mut openings| openings.pop_front())
            .unwrap_or_else(|| Err(Failure::Unreachable("nothing scripted".to_owned())))?;
        let chunks = flowing::iter(opening.chunks);
        Ok(Opened {
            status: opening.status,
            headers: Vec::new(),
            body: if opening.hangs {
                Box::pin(futures_util::StreamExt::chain(chunks, flowing::pending()))
            } else {
                Box::pin(chunks)
            },
        })
    }
}

fn enveloped(kind: &str, data: &Value) -> Vec<u8> {
    json!({ "api_version": 1, "kind": kind, "data": data })
        .to_string()
        .into_bytes()
}

fn reply(status: u16, body: Vec<u8>) -> Response {
    Response {
        status,
        headers: Vec::new(),
        body,
    }
}

fn problem(code: &str, summary: &str) -> Vec<u8> {
    enveloped(
        "error",
        &json!({
            "code": code,
            "severity": "error",
            "state": "actionable",
            "summary": summary,
            "meaning": "What it means.",
            "remedies": [],
        }),
    )
}

fn started(job: &str) -> Vec<u8> {
    enveloped("job", &json!({ "action": "restart", "job": job }))
}

fn listing() -> Value {
    json!({ "keys": [], "purposes": "", "rehearsed": false })
}

fn loopback() -> Result<Address, crate::Error> {
    Address::new("http://127.0.0.1:9000", None)
}

fn client(scripted: Scripted) -> Result<(Client, Scripted), crate::Error> {
    Ok((
        Client::with_transport(loopback()?, Credential::new("token")?, scripted.clone())?,
        scripted,
    ))
}

fn stream_of(status: u16, chunks: &[&str], hangs: bool) -> Opening {
    Opening {
        status,
        chunks: chunks
            .iter()
            .map(|chunk| Ok(chunk.as_bytes().to_vec()))
            .collect(),
        hangs,
    }
}

pub(crate) struct Silent;

impl Transport for Silent {
    async fn send(&self, _request: Request) -> Result<Response, Failure> {
        std::future::pending().await
    }

    async fn open(&self, _request: Request) -> Result<Opened, Failure> {
        std::future::pending().await
    }
}
